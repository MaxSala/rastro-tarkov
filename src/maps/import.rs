//! Importación en dos pasos: preparar (validar en zona temporal) y confirmar (instalar).

use super::pack::{validate_dir, PackStore};
use super::zipx;
use crate::error::{AppError, AppResult};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MAX_PENDING: usize = 4;
const DEMO_MANIFEST: &[u8] = include_bytes!("../../assets/demo/manifest.json");
const DEMO_IMAGE: &[u8] = include_bytes!("../../assets/demo/demo.svg");

/// Lo que el usuario debe ver y aceptar antes de instalar.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub staging_id: String,
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub source: String,
    pub extracts: usize,
    pub width: u32,
    pub height: u32,
    pub sha256: String,
    /// None = el usuario no pegó hash; Some(true) = coincide.
    pub sha256_verified: Option<bool>,
    pub exists: bool,
}

pub struct Importer {
    staging_root: PathBuf,
    pending: HashMap<String, PathBuf>,
    order: Vec<String>,
}

pub fn random_hex(bytes: usize) -> AppResult<String> {
    let mut buf = vec![0u8; bytes];
    getrandom::fill(&mut buf).map_err(|e| AppError::Internal(format!("aleatoriedad: {e}")))?;
    Ok(data_encoding::HEXLOWER.encode(&buf))
}

fn normalize_sha(raw: &str) -> AppResult<String> {
    let s = raw.trim().to_ascii_lowercase();
    if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(AppError::bad("el SHA-256 debe tener 64 caracteres hexadecimales"));
    }
    Ok(s)
}

impl Importer {
    pub fn new(staging_root: &Path) -> Self {
        Self { staging_root: staging_root.to_path_buf(), pending: HashMap::new(), order: Vec::new() }
    }

    fn new_slot(&mut self) -> AppResult<(String, PathBuf)> {
        if self.order.len() >= MAX_PENDING {
            let oldest = self.order.remove(0);
            self.cancel(&oldest);
        }
        let id = random_hex(16)?;
        let dir = self.staging_root.join(&id);
        std::fs::create_dir(&dir)?;
        Ok((id, dir))
    }

    fn finish(
        &mut self,
        id: String,
        dir: PathBuf,
        sha: String,
        verified: Option<bool>,
        store: &PackStore,
    ) -> AppResult<Preview> {
        let pack = match validate_dir(&dir) {
            Ok(p) => p,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&dir);
                return Err(e);
            }
        };
        let m = &pack.manifest;
        let preview = Preview {
            staging_id: id.clone(),
            id: m.id.clone(),
            name: m.name.clone(),
            author: m.credits.author.clone(),
            license: m.credits.license.clone(),
            source: m.credits.source.clone(),
            extracts: m.extracts.len(),
            width: m.image.width,
            height: m.image.height,
            sha256: sha,
            sha256_verified: verified,
            exists: store.contains(&m.id),
        };
        self.pending.insert(id.clone(), dir);
        self.order.push(id);
        Ok(preview)
    }

    /// Prepara un .zip (subido o descargado). Si hay hash esperado y no coincide, se rechaza.
    pub fn stage_zip(&mut self, bytes: &[u8], expected_sha: Option<&str>, store: &PackStore) -> AppResult<Preview> {
        let sha = data_encoding::HEXLOWER.encode(&Sha256::digest(bytes));
        let verified = match expected_sha.filter(|s| !s.trim().is_empty()) {
            Some(raw) if normalize_sha(raw)? != sha => {
                return Err(AppError::pack("el SHA-256 no coincide: el archivo no es el esperado"));
            }
            Some(_) => Some(true),
            None => None,
        };
        let (id, dir) = self.new_slot()?;
        if let Err(e) = zipx::extract(bytes, &dir) {
            let _ = std::fs::remove_dir_all(&dir);
            return Err(e);
        }
        self.finish(id, dir, sha, verified, store)
    }

    /// Prepara el paquete de ejemplo embebido; pasa por la misma validación.
    pub fn stage_demo(&mut self, store: &PackStore) -> AppResult<Preview> {
        self.stage_files(&[("manifest.json", DEMO_MANIFEST), ("demo.svg", DEMO_IMAGE)], store)
    }

    /// Escribe archivos generados en una zona temporal nueva y los valida como un paquete.
    /// Los nombres deben ser planos (se comprueba igual que en un zip).
    pub fn stage_files(&mut self, files: &[(&str, &[u8])], store: &PackStore) -> AppResult<Preview> {
        let (id, dir) = self.new_slot()?;
        let mut hasher = Sha256::new();
        for (name, bytes) in files {
            let ok = *name == "manifest.json" || super::image::Kind::from_file_name(name).is_some();
            if !ok {
                let _ = std::fs::remove_dir_all(&dir);
                return Err(AppError::pack(format!("archivo no permitido: {name}")));
            }
            hasher.update(bytes);
            std::fs::write(dir.join(name), bytes)?;
        }
        let sha = data_encoding::HEXLOWER.encode(&hasher.finalize());
        self.finish(id, dir, sha, None, store)
    }

    pub fn commit(&mut self, staging_id: &str, overwrite: bool, store: &mut PackStore) -> AppResult<String> {
        let dir = self
            .pending
            .get(staging_id)
            .cloned()
            .ok_or_else(|| AppError::bad("la importación caducó; vuelve a empezar"))?;
        let id = store.install(&dir, overwrite)?;
        self.pending.remove(staging_id);
        self.order.retain(|s| s != staging_id);
        Ok(id)
    }

    pub fn cancel(&mut self, staging_id: &str) {
        if let Some(dir) = self.pending.remove(staging_id) {
            let _ = std::fs::remove_dir_all(dir);
        }
        self.order.retain(|s| s != staging_id);
    }
}

#[cfg(test)]
#[path = "import_tests.rs"]
mod tests;
