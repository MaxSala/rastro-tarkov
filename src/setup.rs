//! Primer arranque: carpeta de capturas y elección de cómo cargar los mapas.

use crate::capture::parser;
use crate::config;
use crate::error::{AppError, AppResult};
use crate::party::lock;
use crate::state::App;
use serde::Serialize;
use std::path::PathBuf;

const MAX_LISTED: usize = 20_000;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderCheck {
    pub path: String,
    pub exists: bool,
    /// Capturas con coordenadas encontradas (solo se leen los nombres).
    pub captures: usize,
}

pub fn check_folder(raw: &str) -> AppResult<FolderCheck> {
    let path = PathBuf::from(raw.trim());
    if raw.len() > 1024 || !path.is_absolute() {
        return Err(AppError::bad(
            "enter a full path, for example C:\\Users\\you\\Documents\\Escape from Tarkov\\Screenshots",
        ));
    }
    let mut captures = 0;
    let exists = path.is_dir();
    if exists {
        for entry in std::fs::read_dir(&path)?.take(MAX_LISTED).flatten() {
            if entry.file_name().to_str().is_some_and(|n| parser::parse(n).is_some()) {
                captures += 1;
            }
        }
    }
    Ok(FolderCheck { path: path.display().to_string(), exists, captures })
}

impl App {
    pub fn set_screenshots_dir(&self, raw: &str) -> AppResult<()> {
        let check = check_folder(raw)?;
        if !check.exists {
            return Err(AppError::user_with(
                "folderMissing",
                &check.path,
                format!("The folder {} does not exist.", check.path),
            ));
        }
        let mut inner = lock(&self.inner);
        let mut next = inner.config.clone();
        next.screenshots_dir = Some(PathBuf::from(raw.trim()));
        next.validate()?;
        config::save(&self.paths.config_file, &next)?;
        inner.config = next;
        drop(inner);
        self.restart_watcher();
        Ok(())
    }

    pub fn set_open_in_window(&self, value: bool) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.config.open_in_window = value;
        config::save(&self.paths.config_file, &inner.config)?;
        drop(inner);
        self.notify();
        Ok(())
    }

    pub fn set_language(&self, lang: crate::config::Language) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        if inner.config.language == lang {
            return Ok(());
        }
        inner.config.language = lang;
        config::save(&self.paths.config_file, &inner.config)?;
        drop(inner);
        self.notify();
        Ok(())
    }

    pub fn finish_onboarding(&self) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.config.onboarding_done = true;
        config::save(&self.paths.config_file, &inner.config)?;
        drop(inner);
        self.notify();
        Ok(())
    }
}
