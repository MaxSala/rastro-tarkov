//! Descarga del catálogo (bajo petición), caché en disco y estado visible.

use super::build::{self, Locales};
use super::raw::{parse_locale, ItemsDoc, TradersDoc};
use super::{Catalog, MarketState, MarketStatus};
use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use crate::maps::download::{fetch_with, inspect, Options};
use crate::party::lock;
use crate::state::App;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HOSTS: &[&str] = &["json.tarkov.dev", "assets.tarkov.dev"];
const BASE: &str = "https://json.tarkov.dev/";
const MAX_ITEMS_DOC: usize = 160 << 20;
const MAX_LOCALE_DOC: usize = 64 << 20;
const MAX_CACHE: u64 = 64 << 20;

async fn get(url: &str, max: usize) -> AppResult<Vec<u8>> {
    let (u, _) = inspect(url)?;
    fetch_with(u, max, Options { json_body: None, allowed_hosts: Some(HOSTS) }).await
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

impl App {
    fn market_cache(&self, mode: &str, lang: &str) -> PathBuf {
        self.paths.base.join("market").join(format!("{mode}-{lang}.json"))
    }

    fn market_key(&self) -> (String, String) {
        let lang = lock(&self.inner).config.language.code().to_owned();
        (self.effective_mode().api_name().to_owned(), lang)
    }

    /// Catálogo para la web: el de memoria si coincide con modo e idioma; si no, el de disco.
    pub fn market_catalog(&self) -> Option<Arc<Catalog>> {
        let (mode, lang) = self.market_key();
        if let Some(c) = lock(&self.market).1.clone().filter(|c| c.mode == mode && c.lang == lang) {
            return Some(c);
        }
        let bytes = read_limited(&self.market_cache(&mode, &lang), MAX_CACHE).ok().flatten()?;
        let cat: Catalog = serde_json::from_slice(&bytes).ok()?;
        let cat = Arc::new(cat);
        let mut m = lock(&self.market);
        m.0 = MarketStatus {
            state: MarketState::Ready,
            fetched_at: Some(cat.fetched_at),
            count: cat.items.len(),
            mode: Some(mode),
            lang: Some(lang),
            error: None,
        };
        m.1 = Some(cat.clone());
        Some(cat)
    }

    /// Estado para la vista. Si cambian el idioma o el modo, el estado anterior ya no vale: se
    /// busca en disco el catálogo que toca (o se queda vacío hasta que el usuario lo descargue).
    pub fn market_status(&self) -> MarketStatus {
        let (mode, lang) = self.market_key();
        let stale = {
            let mut m = lock(&self.market);
            let other = m.0.mode.as_deref() != Some(mode.as_str()) || m.0.lang.as_deref() != Some(lang.as_str());
            let stale = m.0.state != MarketState::Loading && (m.0.state == MarketState::Empty || other);
            if stale {
                m.0 = MarketStatus::default();
            }
            stale
        };
        if stale {
            let _ = self.market_catalog();
        }
        lock(&self.market).0.clone()
    }

    /// Descarga en segundo plano. `flea_name`: texto del mercadillo en el idioma de la interfaz.
    pub fn market_refresh(self: &Arc<Self>, flea_name: String) -> AppResult<()> {
        {
            let mut m = lock(&self.market);
            if m.0.state == MarketState::Loading {
                return Err(AppError::user("marketBusy", "A price download is already running."));
            }
            m.0.state = MarketState::Loading;
            m.0.error = None;
        }
        self.notify();
        let app = self.clone();
        let (mode, lang) = self.market_key();
        tokio::spawn(async move {
            let result = download(&mode, &lang, &flea_name).await;
            match result {
                Ok(cat) => {
                    tracing::info!("Market: {} items ({mode}, {lang})", cat.items.len());
                    let path = app.market_cache(&mode, &lang);
                    let saved = path.parent().map(std::fs::create_dir_all).transpose().and_then(|_| {
                        let json = serde_json::to_vec(&cat).map_err(std::io::Error::other)?;
                        std::fs::write(path.with_extension("tmp"), json)?;
                        std::fs::rename(path.with_extension("tmp"), &path)
                    });
                    if let Err(e) = saved {
                        tracing::warn!("Market: could not save the cache: {e}");
                    }
                    let mut m = lock(&app.market);
                    m.0 = MarketStatus {
                        state: MarketState::Ready,
                        fetched_at: Some(cat.fetched_at),
                        count: cat.items.len(),
                        mode: Some(mode),
                        lang: Some(lang),
                        error: None,
                    };
                    m.1 = Some(Arc::new(cat));
                }
                Err(e) => {
                    tracing::warn!("Market: {e}");
                    let mut m = lock(&app.market);
                    let have = m.1.as_ref().is_some_and(|c| c.mode == mode && c.lang == lang);
                    m.0.state = if have { MarketState::Ready } else { MarketState::Error };
                    m.0.mode = Some(mode);
                    m.0.lang = Some(lang);
                    m.0.error = Some(e.user_msg());
                }
            }
            app.notify();
        });
        Ok(())
    }
}

async fn download(mode: &str, lang: &str, flea_name: &str) -> AppResult<Catalog> {
    let items = get(&format!("{BASE}{mode}/items"), MAX_ITEMS_DOC).await?;
    let traders = get(&format!("{BASE}{mode}/traders"), 4 << 20).await?;
    let primary = get(&format!("{BASE}{mode}/items_{lang}"), MAX_LOCALE_DOC).await.unwrap_or_default();
    let traders_loc = get(&format!("{BASE}{mode}/traders_{lang}"), 4 << 20).await.unwrap_or_default();
    // Respaldo en inglés para lo que no esté traducido (como hace tarkov.dev).
    let (fallback, traders_en) = if lang == "en" {
        (Vec::new(), Vec::new())
    } else {
        (
            get(&format!("{BASE}{mode}/items_en"), MAX_LOCALE_DOC).await.unwrap_or_default(),
            get(&format!("{BASE}{mode}/traders_en"), 4 << 20).await.unwrap_or_default(),
        )
    };
    let flea_name = flea_name.to_owned();
    let (mode, lang) = (mode.to_owned(), lang.to_owned());
    // El análisis de decenas de MB va fuera del runtime asíncrono.
    tokio::task::spawn_blocking(move || -> AppResult<Catalog> {
        let doc: ItemsDoc =
            serde_json::from_slice(&items).map_err(|e| AppError::Download(format!("invalid items: {e}")))?;
        let tdoc: TradersDoc =
            serde_json::from_slice(&traders).map_err(|e| AppError::Download(format!("invalid traders: {e}")))?;
        let mut primary_loc = parse_locale(&primary);
        primary_loc.extend(parse_locale(&traders_loc));
        let mut fallback_loc = parse_locale(&fallback);
        fallback_loc.extend(parse_locale(&traders_en));
        let locales = Locales { primary: primary_loc, fallback: fallback_loc };
        let traders = build::traders(&tdoc, &locales);
        let items = build::items(doc, &traders, &locales, &flea_name);
        if items.is_empty() {
            return Err(AppError::Download("the catalog is empty".into()));
        }
        Ok(Catalog { mode, lang, fetched_at: now(), items })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}
