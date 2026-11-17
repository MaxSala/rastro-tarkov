//! Vigila la carpeta de capturas. Solo mira nombres de archivo: no lee, mueve ni borra nada.

use super::parser::{self, Capture};
use crate::msg::UserMsg;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CaptureEvent {
    pub file_name: String,
    pub capture: Capture,
}

/// Mientras exista, la carpeta sigue vigilada.
pub struct WatchGuard {
    _watcher: RecommendedWatcher,
}

pub fn spawn(dir: &Path, tx: mpsc::Sender<CaptureEvent>) -> Result<WatchGuard, UserMsg> {
    if !dir.is_dir() {
        return Err(UserMsg::with(
            "folderMissing",
            dir.display(),
            format!("The folder {} does not exist.", dir.display()),
        ));
    }
    let handler = move |res: notify::Result<notify::Event>| match res {
        Ok(event) => handle(event, &tx),
        Err(e) => tracing::warn!("screenshot watcher error: {e}"),
    };
    let mut watcher = notify::recommended_watcher(handler)
        .map_err(|e| UserMsg::with("watchFailed", &e, format!("Could not start watching: {e}")))?;
    watcher
        .watch(dir, RecursiveMode::NonRecursive)
        .map_err(|e| UserMsg::with("watchFailed", &e, format!("Could not watch {}: {e}", dir.display())))?;
    Ok(WatchGuard { _watcher: watcher })
}

fn handle(event: notify::Event, tx: &mpsc::Sender<CaptureEvent>) {
    // Creación o renombrado: el juego escribe el archivo con su nombre definitivo.
    if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(notify::event::ModifyKind::Name(_))) {
        return;
    }
    for path in event.paths {
        if let Some(ev) = to_event(&path) {
            // Si la cola está llena se descarta: llegará otra captura.
            if tx.try_send(ev).is_err() {
                tracing::warn!("screenshot queue full; dropping one");
            }
        }
    }
}

fn to_event(path: &Path) -> Option<CaptureEvent> {
    let file_name = path.file_name()?.to_str()?.to_owned();
    match parser::parse(&file_name) {
        Some(capture) => Some(CaptureEvent { file_name, capture }),
        None => {
            tracing::debug!("file ignored (not a screenshot with coordinates)");
            None
        }
    }
}
