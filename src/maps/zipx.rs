//! Extracción defensiva de paquetes .zip: sin zip-slip, sin bombas, sin sorpresas.

use super::image::Kind;
use crate::error::{AppError, AppResult};
use std::collections::HashSet;
use std::io::{Cursor, Read, Write};
use std::path::Path;

pub const MAX_ZIP_BYTES: usize = 50 * 1024 * 1024;
pub const MAX_UNPACKED_BYTES: u64 = 100 * 1024 * 1024;
const MAX_RATIO: u64 = 100;
const MAX_ENTRIES: usize = 10;
const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;

/// Nombre de entrada aceptable: UTF-8, plano (en la raíz), sin control ni rutas.
fn check_entry_name(raw: &[u8]) -> AppResult<String> {
    let name = std::str::from_utf8(raw).map_err(|_| AppError::pack("nombre de archivo no UTF-8"))?;
    if name.is_empty() || name.len() > 64 || name.chars().any(char::is_control) {
        return Err(AppError::pack("nombre de archivo no válido en el zip"));
    }
    if name.contains(['/', '\\', ':']) || name.contains("..") {
        return Err(AppError::pack(
            "el zip solo puede contener manifest.json y la imagen en la raíz (sin carpetas ni rutas)",
        ));
    }
    if name != "manifest.json" && Kind::from_file_name(name).is_none() {
        return Err(AppError::pack(format!("archivo no permitido en el zip: {name}")));
    }
    Ok(name.to_owned())
}

/// Extrae `bytes` en `dest` (que debe existir y estar vacío). Valida estructura, no contenido.
pub fn extract(bytes: &[u8], dest: &Path) -> AppResult<()> {
    if bytes.len() > MAX_ZIP_BYTES {
        return Err(AppError::pack("el zip supera 50 MB"));
    }
    let mut archive =
        zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| AppError::pack(format!("no es un zip válido ({e})")))?;
    if archive.len() > MAX_ENTRIES {
        return Err(AppError::pack(format!("el zip tiene más de {MAX_ENTRIES} entradas")));
    }
    // Ratio sobre el tamaño real del zip, no sobre lo que declaran las cabeceras.
    let total_cap = MAX_UNPACKED_BYTES.min((bytes.len() as u64).saturating_mul(MAX_RATIO));
    let mut total: u64 = 0;
    let mut seen = HashSet::new();
    let (mut manifests, mut images) = (0, 0);

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| AppError::pack(format!("entrada ilegible ({e})")))?;
        if entry.is_symlink() || entry.is_dir() || !entry.is_file() {
            return Err(AppError::pack("el zip contiene carpetas o enlaces simbólicos"));
        }
        let name = check_entry_name(entry.name_raw())?;
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(AppError::pack(format!("entrada duplicada: {name}")));
        }
        if name == "manifest.json" {
            manifests += 1;
        } else {
            images += 1;
        }
        let entry_cap = if name == "manifest.json" { MAX_MANIFEST_BYTES } else { total_cap };
        let remaining = total_cap.saturating_sub(total).min(entry_cap);
        let mut out = std::fs::OpenOptions::new().write(true).create_new(true).open(dest.join(&name))?;
        let mut limited = (&mut entry).take(remaining.saturating_add(1));
        let mut buf = [0u8; 64 * 1024];
        let mut written: u64 = 0;
        loop {
            let n = limited.read(&mut buf).map_err(|e| AppError::pack(format!("datos corruptos ({e})")))?;
            if n == 0 {
                break;
            }
            written += n as u64;
            if written > remaining {
                return Err(AppError::pack("el contenido descomprimido es demasiado grande (posible bomba zip)"));
            }
            out.write_all(buf.get(..n).unwrap_or_default())?;
        }
        total += written;
    }
    if manifests != 1 || images != 1 {
        return Err(AppError::pack("el zip debe contener exactamente manifest.json y una imagen"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "zipx_tests.rs"]
mod tests;
