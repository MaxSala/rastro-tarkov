//! Vigila la carpeta de capturas. Solo mira nombres de archivo: no lee, mueve ni borra nada.

use super::parser::{self, Capture};
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

pub fn spawn(dir: &Path, tx: mpsc::Sender<CaptureEvent>) -> Result<WatchGuard, String> {
    if !dir.is_dir() {
        return Err(format!("La carpeta {} no existe.", dir.display()));
    }
    let handler = move |res: notify::Result<notify::Event>| match res {
        Ok(event) => handle(event, &tx),
        Err(e) => tracing::warn!("error del vigilante de capturas: {e}"),
    };
    let mut watcher =
        notify::recommended_watcher(handler).map_err(|e| format!("No se pudo crear el vigilante: {e}"))?;
    watcher
        .watch(dir, RecursiveMode::NonRecursive)
        .map_err(|e| format!("No se pudo vigilar {}: {e}", dir.display()))?;
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
                tracing::warn!("cola de capturas llena; se descarta una");
            }
        }
    }
}

fn to_event(path: &Path) -> Option<CaptureEvent> {
    let file_name = path.file_name()?.to_str()?.to_owned();
    match parser::parse(&file_name) {
        Some(capture) => Some(CaptureEvent { file_name, capture }),
        None => {
            tracing::debug!("archivo ignorado (no es una captura con coordenadas)");
            None
        }
    }
}
