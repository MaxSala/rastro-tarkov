//! Almacén de paquetes instalados en `maps/<id>/` e instalación atómica.

use super::image::{self, Kind};
use super::manifest::{is_valid_id, Manifest};
use super::project::Projection;
use super::zipx::MAX_UNPACKED_BYTES;
use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;

#[derive(Debug)]
pub struct Pack {
    pub manifest: Manifest,
    pub projection: Projection,
    pub kind: Kind,
    pub dir: PathBuf,
}

impl Pack {
    pub fn image_path(&self) -> PathBuf {
        self.dir.join(&self.manifest.image.file)
    }
}

/// Valida por completo una carpeta de paquete: manifest, imagen y que no haya nada más.
pub fn validate_dir(dir: &Path) -> AppResult<Pack> {
    let bytes = read_limited(&dir.join("manifest.json"), MAX_MANIFEST_BYTES)?
        .ok_or_else(|| AppError::pack("manifest.json is missing"))?;
    let manifest: Manifest =
        serde_json::from_slice(&bytes).map_err(|e| AppError::pack(format!("manifest.json: {e}")))?;
    let projection = manifest.validate().map_err(AppError::InvalidPack)?;
    let kind = Kind::from_file_name(&manifest.image.file).ok_or_else(|| AppError::pack("invalid image name"))?;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let ok = name == "manifest.json" || name.to_str() == Some(manifest.image.file.as_str());
        if !ok || !entry.file_type()?.is_file() {
            return Err(AppError::pack("the pack folder contains unexpected files"));
        }
    }
    let img = read_limited(&dir.join(&manifest.image.file), MAX_UNPACKED_BYTES)?
        .ok_or_else(|| AppError::pack(format!("the image {} is missing", manifest.image.file)))?;
    image::check(kind, &img, (manifest.image.width, manifest.image.height)).map_err(AppError::InvalidPack)?;
    Ok(Pack { manifest, projection, kind, dir: dir.to_path_buf() })
}

pub struct PackStore {
    root: PathBuf,
    packs: BTreeMap<String, Arc<Pack>>,
}

impl PackStore {
    pub fn load(root: &Path) -> Self {
        let mut packs = BTreeMap::new();
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let Some(id) = entry.file_name().to_str().map(str::to_owned) else {
                    continue;
                };
                if !is_valid_id(&id) {
                    continue;
                }
                match validate_dir(&entry.path()) {
                    Ok(p) if p.manifest.id == id => {
                        packs.insert(id, Arc::new(p));
                    }
                    Ok(_) => {
                        tracing::warn!("pack {id}: the manifest id does not match; ignored")
                    }
                    Err(e) => tracing::warn!("pack {id} ignored: {e}"),
                }
            }
        }
        Self { root: root.to_path_buf(), packs }
    }

    pub fn get(&self, id: &str) -> Option<Arc<Pack>> {
        self.packs.get(id).cloned()
    }

    pub fn all(&self) -> impl Iterator<Item = &Arc<Pack>> {
        self.packs.values()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.packs.contains_key(id)
    }

    /// Mueve una carpeta ya validada a `maps/<id>` con renombrados atómicos.
    pub fn install(&mut self, staged: &Path, overwrite: bool) -> AppResult<String> {
        let checked = validate_dir(staged)?;
        let id = checked.manifest.id.clone();
        let target = self.root.join(&id);
        let backup = self.root.join(format!(".old-{id}"));
        if target.exists() {
            if !overwrite {
                return Err(AppError::PackExists(id));
            }
            if backup.exists() {
                std::fs::remove_dir_all(&backup)?;
            }
            std::fs::rename(&target, &backup)?;
        }
        if let Err(e) = std::fs::rename(staged, &target) {
            if backup.exists() {
                let _ = std::fs::rename(&backup, &target);
            }
            return Err(e.into());
        }
        if backup.exists() {
            let _ = std::fs::remove_dir_all(&backup);
        }
        let pack = validate_dir(&target)?;
        self.packs.insert(id.clone(), Arc::new(pack));
        Ok(id)
    }

    pub fn remove(&mut self, id: &str) -> AppResult<()> {
        if !is_valid_id(id) || self.packs.remove(id).is_none() {
            return Err(AppError::NotFound);
        }
        std::fs::remove_dir_all(self.root.join(id))?;
        Ok(())
    }
}
