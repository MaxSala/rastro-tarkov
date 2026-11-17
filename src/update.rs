//! Optional update check. Rastro asks GitHub for its latest release only if the player agreed
//! to it (or pressed "Check now"). The request carries nothing about the player beyond what any
//! HTTPS request does; the answer is validated before anything of it reaches the interface.

use crate::error::{AppError, AppResult};
use crate::maps::download::{fetch_with, inspect, Options};
use crate::msg::UserMsg;
use crate::party::lock;
use crate::state::App;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const HOSTS: &[&str] = &["api.github.com"];
const MAX_BYTES: usize = 512 * 1024;
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const CURRENT: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UpdateState {
    #[default]
    Idle,
    Checking,
    UpToDate,
    Available,
    Error,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub state: UpdateState,
    /// Latest published version, as `v1.2.3`.
    pub latest: Option<String>,
    /// Release page on GitHub (always under this project's `/releases/`).
    pub url: Option<String>,
    pub error: Option<UserMsg>,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
}

/// `v1.2.3` or `1.2.3` (anything after the patch number is ignored).
pub fn parse_version(s: &str) -> Option<(u32, u32, u32)> {
    let mut it = s.strip_prefix('v').unwrap_or(s).split('.');
    let major = it.next()?.parse().ok()?;
    let minor = it.next()?.parse().ok()?;
    let patch = it.next()?.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()?;
    Some((major, minor, patch))
}

/// Checks GitHub's answer: (is newer, clean tag, release page).
pub fn evaluate(body: &[u8], current: &str, repository: &str) -> Result<(bool, String, String), String> {
    let r: Release = serde_json::from_slice(body).map_err(|e| format!("invalid release data: {e}"))?;
    if r.draft || r.prerelease {
        return Err("the latest release is not a final release".into());
    }
    let (major, minor, patch) = parse_version(&r.tag_name).ok_or("invalid release tag")?;
    let ours = parse_version(current).ok_or("invalid current version")?;
    let prefix = format!("{repository}/releases/");
    let url_ok = r.html_url.starts_with(&prefix)
        && r.html_url.len() <= 200
        && r.html_url.chars().all(|c| c.is_ascii_graphic() && !matches!(c, '"' | '<' | '>' | '\\'));
    if !url_ok {
        return Err("unexpected release page address".into());
    }
    Ok(((major, minor, patch) > ours, format!("v{major}.{minor}.{patch}"), r.html_url))
}

async fn fetch_latest() -> AppResult<(bool, String, String)> {
    let repo = REPOSITORY.strip_prefix("https://github.com/").filter(|r| r.split('/').count() == 2);
    let repo = repo.ok_or_else(|| AppError::Internal("no GitHub repository configured".into()))?;
    let (url, _) = inspect(&format!("https://api.github.com/repos/{repo}/releases/latest"))?;
    let body = fetch_with(url, MAX_BYTES, Options { json_body: None, allowed_hosts: Some(HOSTS) }).await?;
    evaluate(&body, CURRENT, REPOSITORY).map_err(AppError::Download)
}

impl App {
    /// Starts a check in the background (ignored if one is already running).
    pub fn check_updates(self: &Arc<Self>) {
        {
            let mut u = lock(&self.update);
            if u.state == UpdateState::Checking {
                return;
            }
            u.state = UpdateState::Checking;
            u.error = None;
        }
        self.notify();
        let app = self.clone();
        tokio::spawn(async move {
            let result = fetch_latest().await;
            {
                let mut u = lock(&app.update);
                match result {
                    Ok((newer, tag, url)) => {
                        tracing::info!("Update check: latest is {tag} (this is v{CURRENT})");
                        u.state = if newer { UpdateState::Available } else { UpdateState::UpToDate };
                        u.latest = Some(tag);
                        u.url = Some(url);
                    }
                    Err(e) => {
                        tracing::warn!("Update check failed: {e}");
                        u.state = UpdateState::Error;
                        u.error = Some(e.user_msg());
                    }
                }
            }
            app.notify();
        });
    }

    pub fn check_updates_on_start(&self) -> bool {
        lock(&self.inner).config.check_updates == Some(true)
    }

    /// The player's answer to "check for updates on startup?".
    pub fn set_check_updates(self: &Arc<Self>, enabled: bool) -> AppResult<()> {
        {
            let mut inner = lock(&self.inner);
            inner.config.check_updates = Some(enabled);
            crate::config::save(&self.paths.config_file, &inner.config)?;
        }
        if enabled {
            self.check_updates();
        }
        self.notify();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPO: &str = "https://github.com/MaxSala/rastro-tarkov";

    fn release(tag: &str, url: &str) -> Vec<u8> {
        format!(r#"{{"tag_name":"{tag}","html_url":"{url}","draft":false,"prerelease":false,"body":"x"}}"#).into_bytes()
    }

    #[test]
    fn compares_versions() {
        let page = "https://github.com/MaxSala/rastro-tarkov/releases/tag/v0.5.0";
        let (newer, tag, url) = evaluate(&release("v0.5.0", page), "0.4.0", REPO).expect("ok");
        assert!(newer);
        assert_eq!((tag.as_str(), url.as_str()), ("v0.5.0", page));
        assert!(!evaluate(&release("v0.4.0", page), "0.4.0", REPO).expect("ok").0);
        assert!(!evaluate(&release("v0.3.9", page), "0.4.0", REPO).expect("ok").0);
        assert!(evaluate(&release("v0.10.0", page), "0.9.9", REPO).expect("ok").0, "numeric, not text, order");
    }

    #[test]
    fn rejects_suspicious_answers() {
        let other = "https://evil.example/releases/v9.9.9";
        assert!(evaluate(&release("v9.9.9", other), "0.4.0", REPO).is_err(), "page outside the project");
        let inject = "https://github.com/MaxSala/rastro-tarkov/releases/\"><script>";
        assert!(evaluate(&release("v9.9.9", inject), "0.4.0", REPO).is_err());
        let page = "https://github.com/MaxSala/rastro-tarkov/releases/tag/x";
        assert!(evaluate(&release("latest", page), "0.4.0", REPO).is_err(), "tag that isn't a version");
        let draft = br#"{"tag_name":"v9.0.0","html_url":"https://github.com/MaxSala/rastro-tarkov/releases/tag/v9.0.0","draft":true}"#;
        assert!(evaluate(draft, "0.4.0", REPO).is_err());
        assert!(evaluate(b"not json", "0.4.0", REPO).is_err());
    }

    #[test]
    fn repository_is_configured() {
        assert!(REPOSITORY.starts_with("https://github.com/"));
    }
}
