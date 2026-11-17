//! Iconos de objetos: se descargan bajo demanda de assets.tarkov.dev, se validan y se guardan.

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

const MAX_ICON: usize = 96 * 1024;
const MAX_SIDE: u32 = 256;
static DOWNLOADS: Semaphore = Semaphore::const_new(4);

pub fn valid_id(id: &str) -> bool {
    id.len() == 24 && id.bytes().all(|b| b.is_ascii_hexdigit())
}

fn validate(bytes: &[u8]) -> AppResult<Kind> {
    let (kind, (w, h)) = sniff(bytes).ok_or_else(|| AppError::pack("invalid icon"))?;
    if bytes.len() > MAX_ICON || w > MAX_SIDE || h > MAX_SIDE || w == 0 || h == 0 {
        return Err(AppError::pack("icon too large"));
    }
    Ok(kind)
}

impl App {
    fn item_icon_path(&self, id: &str) -> PathBuf {
        self.paths.base.join("market").join("icons").join(format!("{id}.img"))
    }

    /// Devuelve el icono (bytes y tipo). Solo se conecta a assets.tarkov.dev si no está en caché.
    pub async fn item_icon(&self, id: &str) -> AppResult<(Vec<u8>, Kind)> {
        if !valid_id(id) {
            return Err(AppError::NotFound);
        }
        let path = self.item_icon_path(id);
        if let Ok(Some(bytes)) = read_limited(&path, MAX_ICON as u64) {
            if let Ok(kind) = validate(&bytes) {
                return Ok((bytes, kind));
            }
        }
        // After a network failure, don't try again for a minute: offline, hundreds of rows would
        // each wait for their own timeout.
        if OFFLINE_UNTIL.lock().map(|t| t.is_some_and(|t| Instant::now() < t)).unwrap_or(false) {
            return Err(AppError::NotFound);
        }
        let _permit = DOWNLOADS.acquire().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let (url, _) = inspect(&format!("https://assets.tarkov.dev/{id}-icon.webp"))?;
        let fetched =
            fetch_with(url, MAX_ICON, Options { json_body: None, allowed_hosts: Some(super::fetch::HOSTS) }).await;
        if let Err(AppError::Download(e)) = &fetched {
            if e.starts_with("could not connect") || e.starts_with("could not resolve") || e.starts_with("DNS") {
                if let Ok(mut t) = OFFLINE_UNTIL.lock() {
                    *t = Some(Instant::now() + Duration::from_secs(60));
                }
            }
        }
        let bytes = fetched?;
        let kind = validate(&bytes)?;
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
