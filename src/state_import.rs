//! Acciones de paquetes de mapa sobre el estado (importar, instalar, borrar).

use crate::config;
use crate::error::AppResult;
use crate::maps::import::Preview;
use crate::maps::pack::Pack;
use crate::party::lock;
use crate::state::{App, Inner};
use std::sync::Arc;

impl App {
    pub fn stage_zip(&self, bytes: &[u8], sha: Option<&str>) -> AppResult<Preview> {
        let mut inner = lock(&self.inner);
        let Inner { importer, store, .. } = &mut *inner;
        importer.stage_zip(bytes, sha, store)
    }

    pub fn stage_demo(&self) -> AppResult<Preview> {
        let mut inner = lock(&self.inner);
        let Inner { importer, store, .. } = &mut *inner;
        importer.stage_demo(store)
    }

    pub fn commit_import(&self, staging_id: &str, overwrite: bool) -> AppResult<String> {
        let mut inner = lock(&self.inner);
        let Inner { importer, store, .. } = &mut *inner;
        let id = importer.commit(staging_id, overwrite, store)?;
        if inner.config.last_map.is_none() {
            inner.config.last_map = Some(id.clone());
            config::save(&self.paths.config_file, &inner.config)?;
        }
        drop(inner);
        self.notify();
        Ok(id)
    }

    pub fn cancel_import(&self, staging_id: &str) {
        lock(&self.inner).importer.cancel(staging_id);
    }

    pub fn remove_pack(&self, id: &str) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.store.remove(id)?;
        if inner.config.last_map.as_deref() == Some(id) {
            let first = inner.store.all().next().map(|p| p.manifest.id.clone());
            inner.config.last_map = first;
            config::save(&self.paths.config_file, &inner.config)?;
        }
        drop(inner);
        self.notify();
        Ok(())
    }

    pub fn pack(&self, id: &str) -> Option<Arc<Pack>> {
        lock(&self.inner).store.get(id)
    }

    /// Prepara archivos ya generados (tarkov.dev) y los instala, reemplazando si existían.
    pub fn install_files(&self, files: &[(&str, &[u8])]) -> AppResult<String> {
        let mut inner = lock(&self.inner);
        let Inner { importer, store, .. } = &mut *inner;
        let preview = importer.stage_files(files, store)?;
        drop(inner);
        self.commit_import(&preview.staging_id, true)
    }
}
