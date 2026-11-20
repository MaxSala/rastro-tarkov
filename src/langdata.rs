//! Downloaded data in another language than the interface: the tarkov.dev maps (their names are
//! written into the installed packs), the price catalog and the quests. After a language change
//! the interface offers to bring them over, or does it on its own if the player chose that.
//! Nothing here connects anywhere until the player (or that choice) asks for it.

use crate::config::{self, Language};
use crate::error::AppResult;
use crate::party::lock;
use crate::state::App;
use crate::tarkovdev::sync::{self, Job, SVG_SOURCE};
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataKind {
    Maps,
    Prices,
    Quests,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Outdated {
    pub kind: DataKind,
    /// The language it's in, when known (maps installed by an earlier build don't say).
    pub lang: Option<Language>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageData {
    pub outdated: Vec<Outdated>,
    /// Update on its own when the language changes. None: the player hasn't been asked.
    pub auto: Option<bool>,
}

/// The newest `<mode>-<lang>.json` cache in `dir` in another language, if there's none in `lang`.
fn cached_in_other_language(dir: &Path, mode: &str, lang: Language) -> Option<Language> {
    if dir.join(format!("{mode}-{}.json", lang.code())).is_file() {
        return None;
    }
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            let code = name.strip_prefix(mode)?.strip_prefix('-')?.strip_suffix(".json")?;
            let other = Language::from_code(code)?;
            Some((e.metadata().and_then(|m| m.modified()).ok()?, other))
        })
        .max_by_key(|(at, _)| *at)
        .map(|(_, l)| l)
}

impl App {
    fn has_tarkovdev_maps(&self) -> bool {
        lock(&self.inner).store.all().any(|p| p.manifest.credits.source == SVG_SOURCE)
    }

    /// For the snapshot. Takes `inner` briefly; call it without holding it.
    pub fn language_data(&self) -> LanguageData {
        let mode = self.effective_mode().api_name();
        let (lang, maps_lang, auto) = {
            let inner = lock(&self.inner);
            (inner.config.language, inner.config.maps_language, inner.config.auto_language_data)
        };
        let mut outdated = Vec::new();
        if maps_lang != Some(lang) && self.has_tarkovdev_maps() {
            outdated.push(Outdated { kind: DataKind::Maps, lang: maps_lang });
        }
        for (kind, dir) in [(DataKind::Prices, "market"), (DataKind::Quests, "quests")] {
            if let Some(other) = cached_in_other_language(&self.paths.base.join(dir), mode, lang) {
                outdated.push(Outdated { kind, lang: Some(other) });
            }
        }
        LanguageData { outdated, auto }
    }

    /// Brings everything that's in another language over to the current one. `flea_name` is the
    /// flea market's name in the interface language, as for a normal price download.
    pub fn update_language_data(self: &Arc<Self>, flea_name: String) -> AppResult<()> {
        for o in self.language_data().outdated {
            let started = match o.kind {
                DataKind::Maps => sync::start_job(self, Job::Relocalize),
                DataKind::Prices => self.market_refresh(flea_name.clone()),
                DataKind::Quests => self.quests_refresh(),
            };
            // Already running (the player started it by hand): that download will do.
            if let Err(e) = started {
                tracing::info!("Language data: {:?} not started: {e}", o.kind);
            }
        }
        Ok(())
    }

    pub fn set_auto_language_data(&self, enabled: bool) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.config.auto_language_data = Some(enabled);
        config::save(&self.paths.config_file, &inner.config)?;
        drop(inner);
        self.notify();
        Ok(())
    }

    /// Called by the map sync once maps are installed with data in `lang`.
    pub fn set_maps_language(&self, lang: Language) {
        let mut inner = lock(&self.inner);
        if inner.config.maps_language == Some(lang) {
            return;
        }
        inner.config.maps_language = Some(lang);
        if let Err(e) = config::save(&self.paths.config_file, &inner.config) {
            tracing::warn!("Could not save the maps language: {e}");
        }
    }
}

#[cfg(test)]
#[path = "langdata_tests.rs"]
mod tests;
