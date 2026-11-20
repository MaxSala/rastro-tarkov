//! Quest data: downloaded only when the player asks, cached on disk per mode and language, and
//! loaded from that cache on the next start. json.tarkov.dev's static files come first; the
//! GraphQL API is only tried if they fail, the same way the maps are downloaded.

use super::jsondoc::{self, Docs, ItemNames};
use super::{query, QuestDb, QuestsState, QuestsStatus};
use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use crate::maps::download::{fetch_with, inspect, Options};
use crate::party::lock;
use crate::state::App;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HOSTS: &[&str] = &["json.tarkov.dev", "api.tarkov.dev"];
const API: &str = "https://api.tarkov.dev/graphql";
const BASE: &str = "https://json.tarkov.dev/";
const MAX_ANSWER: usize = 48 << 20;
const MAX_SMALL: usize = 16 << 20;
const MAX_ITEM_NAMES: usize = 64 << 20;
const MAX_CACHE: u64 = 32 << 20;

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

impl App {
    fn quests_cache(&self, mode: &str, lang: &str) -> PathBuf {
        self.paths.base.join("quests").join(format!("{mode}-{lang}.json"))
    }

    pub(crate) fn quests_key(&self) -> (String, String) {
        let lang = lock(&self.inner).config.language.code().to_owned();
        (self.effective_mode().api_name().to_owned(), lang)
    }

    /// Quest data for the current mode and language: in memory, or else from the disk cache.
    pub fn quest_db(&self) -> Option<Arc<QuestDb>> {
        let (mode, lang) = self.quests_key();
        if let Some(db) = lock(&self.quests).db.clone().filter(|d| d.mode == mode && d.lang == lang) {
            return Some(db);
        }
        let bytes = read_limited(&self.quests_cache(&mode, &lang), MAX_CACHE).ok().flatten()?;
        let db: QuestDb = serde_json::from_slice(&bytes).ok()?;
        let db = Arc::new(db);
        {
            let mut q = lock(&self.quests);
            q.status = ready(&db);
            q.db = Some(db.clone());
        }
        // With the data, what the logs said can also complete the quests it implies.
        self.apply_quest_logs();
        Some(db)
    }

    /// Status for the snapshot. A change of mode or language looks for the matching cache.
    pub fn quests_status(&self) -> QuestsStatus {
        let (mode, lang) = self.quests_key();
        let stale = {
            let mut q = lock(&self.quests);
            let other =
                q.status.mode.as_deref() != Some(mode.as_str()) || q.status.lang.as_deref() != Some(lang.as_str());
            let stale = q.status.state != QuestsState::Loading && (q.status.state == QuestsState::Empty || other);
            if stale {
                q.status = QuestsStatus::default();
                q.db = None;
            }
            stale
        };
        if stale {
            let _ = self.quest_db();
        }
        let mut status = lock(&self.quests).status.clone();
        status.logs = self.quest_log_view();
        status
    }

    /// Background download (player's request).
    pub fn quests_refresh(self: &Arc<Self>) -> AppResult<()> {
        {
            let mut q = lock(&self.quests);
            if q.status.state == QuestsState::Loading {
                return Err(AppError::user("questsBusy", "Quests are already being downloaded."));
            }
            q.status.state = QuestsState::Loading;
            q.status.error = None;
        }
        self.notify();
        let app = self.clone();
        let (mode, lang) = self.quests_key();
        // Item names come from the market catalog when it's there, saving a big download.
        let names: Option<ItemNames> =
            self.market_catalog().map(|c| c.items.iter().map(|i| (i.id.clone(), i.name.clone())).collect());
        tokio::spawn(async move {
            match download(&mode, &lang, names).await {
                Ok(db) => {
                    tracing::info!("Quests: {} quests ({mode}, {lang})", db.quests.len());
                    if let Err(e) = save(&app.quests_cache(&mode, &lang), &db) {
                        tracing::warn!("Quests: could not save the cache: {e}");
                    }
                    let db = Arc::new(db);
                    {
                        let mut q = lock(&app.quests);
                        q.status = ready(&db);
                        q.db = Some(db);
                    }
                    app.apply_quest_logs();
                }
                Err(e) => {
                    tracing::warn!("Quests: {e}");
                    let mut q = lock(&app.quests);
                    let have = q.db.as_ref().is_some_and(|d| d.mode == mode && d.lang == lang);
                    q.status.state = if have { QuestsState::Ready } else { QuestsState::Error };
                    q.status.mode = Some(mode);
                    q.status.lang = Some(lang);
                    q.status.error = Some(e.user_msg());
                }
            }
            app.notify();
        });
        Ok(())
    }
}

fn ready(db: &QuestDb) -> QuestsStatus {
    QuestsStatus {
        state: QuestsState::Ready,
        fetched_at: Some(db.fetched_at),
        count: db.quests.len(),
        mode: Some(db.mode.clone()),
        lang: Some(db.lang.clone()),
        error: None,
        logs: Default::default(),
    }
}

fn save(path: &std::path::Path, db: &QuestDb) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_vec(db).map_err(std::io::Error::other)?;
    std::fs::write(path.with_extension("tmp"), json)?;
    std::fs::rename(path.with_extension("tmp"), path)
}

async fn get(url: &str, max: usize) -> AppResult<Vec<u8>> {
    let (u, _) = inspect(url)?;
    fetch_with(u, max, Options { json_body: None, allowed_hosts: Some(HOSTS) }).await
}

/// `mode` and `lang` come from closed lists (game mode and the 16 languages), never from input.
async fn download(mode: &str, lang: &str, names: Option<ItemNames>) -> AppResult<QuestDb> {
    let json_err = match from_json(mode, lang, names).await {
        Ok(db) => return Ok(db),
        Err(e) => e,
    };
    tracing::warn!("Quests: json.tarkov.dev failed ({json_err}); trying GraphQL");
    from_graphql(mode, lang)
        .await
        .map_err(|e| AppError::Download(format!("json.tarkov.dev: {json_err} · GraphQL: {e}")))
}

async fn from_json(mode: &str, lang: &str, names: Option<ItemNames>) -> AppResult<QuestDb> {
    let docs = Docs {
        tasks: get(&format!("{BASE}{mode}/tasks"), MAX_ANSWER).await?,
        tasks_locale: get(&format!("{BASE}{mode}/tasks_{lang}"), MAX_SMALL).await?,
        traders: get(&format!("{BASE}{mode}/traders"), 4 << 20).await?,
        traders_locale: get(&format!("{BASE}{mode}/traders_{lang}"), 4 << 20).await.unwrap_or_default(),
        maps: get(&format!("{BASE}{mode}/maps"), 32 << 20).await?,
    };
    let names_doc = match names {
        Some(_) => None,
        None => Some(get(&format!("{BASE}{mode}/items_{lang}"), MAX_ITEM_NAMES).await.unwrap_or_default()),
    };
    let (mode, lang) = (mode.to_owned(), lang.to_owned());
    tokio::task::spawn_blocking(move || {
        let names = names.unwrap_or_else(|| item_names(names_doc.as_deref().unwrap_or_default()));
        let v = jsondoc::to_graphql(&docs, &names).map_err(AppError::Download)?;
        query::parse_value(v, &mode, &lang, now()).map_err(AppError::Download)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

/// `items_{lang}`: keys are `<item id> Name`.
fn item_names(doc: &[u8]) -> ItemNames {
    crate::market::raw::parse_locale(doc)
        .into_iter()
        .filter_map(|(k, v)| Some((k.strip_suffix(" Name")?.to_owned(), v)))
        .collect()
}

async fn from_graphql(mode: &str, lang: &str) -> AppResult<QuestDb> {
    let body = serde_json::json!({ "query": query::QUERY.replace("{lang}", lang).replace("{mode}", mode) }).to_string();
    let (url, _) = inspect(API)?;
    let bytes = fetch_with(url, MAX_ANSWER, Options { json_body: Some(&body), allowed_hosts: Some(HOSTS) }).await?;
    let (mode, lang) = (mode.to_owned(), lang.to_owned());
    tokio::task::spawn_blocking(move || query::parse(&bytes, &mode, &lang, now()).map_err(AppError::Download))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
}
