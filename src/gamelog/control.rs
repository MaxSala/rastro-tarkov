//! Acciones de la detección por logs sobre el estado de la app.

use super::parse::{GameState, Phase};
use super::{known_location, watcher, GameMode, GameModePref, GameView};
use crate::config;
use crate::error::{AppError, AppResult};
use crate::maps::pack::Pack;
use crate::party::lock;
use crate::state::App;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

impl App {
    /// Carpeta de logs efectiva: la configurada o la primera de las habituales que exista.
    fn logs_dir(&self) -> Option<PathBuf> {
        let configured = lock(&self.inner).config.logs_dir.clone();
        configured.or_else(|| super::default_logs_dirs().into_iter().find(|d| d.is_dir()))
    }

    /// (Re)arranca el vigilante según la configuración.
    pub fn start_game_logs(self: &Arc<Self>) {
        self.stop_game_logs();
        let enabled = lock(&self.inner).config.read_game_logs;
        let dir = self.logs_dir();
        let mut g = lock(&self.game);
        g.folder_found = dir.as_ref().is_some_and(|d| d.is_dir());
        g.folder = dir.clone();
        let (true, Some(dir)) = (enabled, dir) else { return };
        let stop = Arc::new(AtomicBool::new(false));
        g.stop = Some(stop.clone());
        drop(g);
        tracing::info!("Game log detection enabled");
        watcher::spawn(self.clone(), dir, stop);
    }

    pub fn stop_game_logs(&self) {
        let mut g = lock(&self.game);
        if let Some(stop) = g.stop.take() {
            stop.store(true, Ordering::Relaxed);
        }
        g.state = GameState::default();
    }

    pub fn set_game_logs(self: &Arc<Self>, enabled: bool, path: &str) -> AppResult<()> {
        let path = path.trim();
        let mut inner = lock(&self.inner);
        let mut next = inner.config.clone();
        next.read_game_logs = enabled;
        next.logs_dir = (!path.is_empty()).then(|| PathBuf::from(path));
        next.validate()?;
        config::save(&self.paths.config_file, &next)?;
        inner.config = next;
        drop(inner);
        self.start_game_logs();
        self.notify();
        Ok(())
    }

    pub fn set_game_mode(&self, pref: GameModePref) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.config.game_mode = pref;
        config::save(&self.paths.config_file, &inner.config).map_err(|e| AppError::Internal(e.to_string()))?;
        drop(inner);
        self.notify();
        Ok(())
    }

    /// Modo para los datos de tarkov.dev: el elegido o, en automático, el de los logs (PvP si no hay).
    pub fn effective_mode(&self) -> GameMode {
        match lock(&self.inner).config.game_mode {
            GameModePref::Regular => GameMode::Regular,
            GameModePref::Pve => GameMode::Pve,
            GameModePref::Auto => lock(&self.game).state.mode.unwrap_or(GameMode::Regular),
        }
    }

    /// Paquete instalado que corresponde a un nombre interno del juego.
    fn pack_for_location(&self, loc: &str) -> Option<Arc<Pack>> {
        let inner = lock(&self.inner);
        let by_manifest = inner.store.all().find(|p| p.manifest.game_ids.iter().any(|g| g.eq_ignore_ascii_case(loc)));
        by_manifest.cloned().or_else(|| known_location(loc).and_then(|(id, _)| inner.store.get(id)))
    }

    /// Llamada por el vigilante en cada vuelta. `enter`: mapa y si empieza una raid nueva.
    pub(super) fn game_tick(&self, state: GameState, found: bool, enter: Option<(String, bool)>, changed: bool) {
        let differs = {
            let mut g = lock(&self.game);
            let d = g.state != state || g.folder_found != found;
            g.state = state;
            g.folder_found = found;
            d
        };
        if let Some((loc, fresh)) = enter {
            tracing::info!("Game: loading map {loc}");
            if let Some(pack) = self.pack_for_location(&loc) {
                let active = lock(&self.inner).config.last_map.clone();
                if active.as_deref() != Some(pack.manifest.id.as_str()) {
                    let _ = self.set_active_map(&pack.manifest.id);
                }
            }
            if fresh {
                self.new_raid();
            }
        }
        if differs || changed {
            self.notify();
        }
    }

    pub fn game_view(&self) -> GameView {
        let (enabled, mode_pref) = {
            let inner = lock(&self.inner);
            (inner.config.read_game_logs, inner.config.game_mode)
        };
        let g = lock(&self.game);
        let (state, folder, found) = (g.state.clone(), g.folder.clone(), g.folder_found);
        drop(g);
        let pack = state.location.as_deref().and_then(|l| self.pack_for_location(l));
        let fallback =
            state.location.as_deref().map(|l| known_location(l).map_or_else(|| l.to_owned(), |(_, n)| n.to_owned()));
        GameView {
            enabled,
            folder: folder.map(|f| f.display().to_string()),
            folder_found: found,
            mode: state.mode,
            mode_pref,
            phase: enabled.then_some(state.phase),
            map_name: pack
                .as_ref()
                .map(|p| p.manifest.name.clone())
                .or(fallback)
                .filter(|_| state.phase != Phase::Menu),
            map_installed: pack.is_some(),
            started_at: state.started_at,
            raid_minutes: pack.and_then(|p| p.manifest.raid_minutes),
        }
    }
}
