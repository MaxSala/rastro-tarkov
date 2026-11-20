//! Pictures from assets.tarkov.dev: item icons (Market, keys, quest items) and the portraits of
//! traders and quests. They're downloaded only when the interface shows them, validated by their
//! header and size, and kept on disk. Every address is built here from a 24-hex id, never taken
//! from downloaded data.

use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use crate::maps::download::{fetch_with, inspect, Options};
use crate::maps::image::{sniff, Kind};
use crate::state::App;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

static OFFLINE_UNTIL: Mutex<Option<Instant>> = Mutex::new(None);
static DOWNLOADS: Semaphore = Semaphore::const_new(4);

pub fn valid_id(id: &str) -> bool {
    id.len() == 24 && id.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The two kinds of picture Rastro asks assets.tarkov.dev for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Asset {
    /// `<id>-icon.webp`: an item's inventory icon.
    ItemIcon,
    /// `<id>.webp`: a trader's portrait or a quest's picture.
    Picture,
}

impl Asset {
    fn url(self, id: &str) -> String {
        match self {
            Self::ItemIcon => format!("https://assets.tarkov.dev/{id}-icon.webp"),
            Self::Picture => format!("https://assets.tarkov.dev/{id}.webp"),
        }
    }

    /// Largest file and side accepted.
    fn limits(self) -> (usize, u32) {
        match self {
            Self::ItemIcon => (96 * 1024, 256),
            Self::Picture => (1024 * 1024, 2048),
        }
    }

    fn dir(self) -> &'static str {
        match self {
            Self::ItemIcon => "icons",
            Self::Picture => "pictures",
        }
    }
}

fn validate(bytes: &[u8], asset: Asset) -> AppResult<Kind> {
    let (max_bytes, max_side) = asset.limits();
    let (kind, (w, h)) = sniff(bytes).ok_or_else(|| AppError::pack("invalid image"))?;
    if bytes.len() > max_bytes || w > max_side || h > max_side || w == 0 || h == 0 {
        return Err(AppError::pack("image too large"));
    }
    Ok(kind)
}

fn offline() -> bool {
    OFFLINE_UNTIL.lock().map(|t| t.is_some_and(|t| Instant::now() < t)).unwrap_or(false)
}

impl App {
    fn asset_path(&self, asset: Asset, id: &str) -> PathBuf {
        self.paths.base.join("market").join(asset.dir()).join(format!("{id}.img"))
    }

    /// An item icon (bytes and type). Only connects to assets.tarkov.dev if it isn't cached.
    pub async fn item_icon(&self, id: &str) -> AppResult<(Vec<u8>, Kind)> {
        self.tarkov_asset(Asset::ItemIcon, id).await
    }

    pub async fn tarkov_asset(&self, asset: Asset, id: &str) -> AppResult<(Vec<u8>, Kind)> {
        if !valid_id(id) {
            return Err(AppError::NotFound);
        }
        let id = id.to_ascii_lowercase();
        let path = self.asset_path(asset, &id);
        let (max_bytes, _) = asset.limits();
        if let Ok(Some(bytes)) = read_limited(&path, max_bytes as u64) {
            if let Ok(kind) = validate(&bytes, asset) {
                return Ok((bytes, kind));
            }
        }
        // After a network failure, don't try again for a minute: offline, hundreds of rows would
        // each wait for their own timeout.
        if offline() {
            return Err(AppError::NotFound);
        }
        let _permit = DOWNLOADS.acquire().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let (url, _) = inspect(&asset.url(&id))?;
        let fetched =
            fetch_with(url, max_bytes, Options { json_body: None, allowed_hosts: Some(super::fetch::HOSTS) }).await;
        if let Err(AppError::Download(e)) = &fetched {
            if e.starts_with("could not connect") || e.starts_with("could not resolve") || e.starts_with("DNS") {
                if let Ok(mut t) = OFFLINE_UNTIL.lock() {
                    *t = Some(Instant::now() + Duration::from_secs(60));
                }
            }
        }
        let bytes = fetched?;
        let kind = validate(&bytes, asset)?;
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
            let tmp = path.with_extension("tmp");
            if std::fs::write(&tmp, &bytes).is_ok() {
                let _ = std::fs::rename(&tmp, &path);
            }
        }
        Ok((bytes, kind))
    }
}
