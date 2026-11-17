//! Iconos de marcadores de tarkov.dev (`public/maps/interactive`, licencia MIT del proyecto
//! the-hideout/tarkov-dev, Copyright (c) 2019 Oskar Risberg). Se descargan bajo petición, se
//! validan como PNG pequeños y se sirven solo por nombre de una lista cerrada.

use crate::error::{AppError, AppResult};
use crate::maps::image::{check, Kind};
use std::path::{Path, PathBuf};

pub const BASE_URL: &str = "https://raw.githubusercontent.com/the-hideout/tarkov-dev/main/public/maps/interactive/";
const MAX_ICON_BYTES: usize = 64 * 1024;
const MAX_ICON_SIDE: u32 = 256;

/// Iconos que usa Rastro (archivo sin extensión `.png`).
pub const ICONS: &[&str] = &[
    "extract_pmc",
    "extract_scav",
    "extract_shared",
    "extract_transit",
    "switch",
    "lock",
    "container_buried-barrel-cache",
    "container_cash-register",
    "container_crate",
    "container_dead-scav",
    "container_drawer",
    "container_duffle-bag",
    "container_festive-airdrop-supply-crate",
    "container_grenade-box",
    "container_ground-cache",
    "container_jacket",
    "container_medbag",
    "container_medcase",
    "container_pc-block",
    "container_plastic-suitcase",
    "container_safe",
    "container_toolbox",
    "container_weapon-box",
    "container_wooden-ammo-box",
    "container_wooden-crate",
];

/// Tipo de contenedor (normalizedName) → icono. Tabla equivalente a la de tarkov.dev
/// (src/pages/map/map-images.mjs), para que los alias usen el mismo dibujo.
const CONTAINER_ALIASES: &[(&str, &str)] = &[
    ("bank-cash-register", "cash-register"),
    ("bank-safe", "safe"),
    ("cash-register-tar2-2", "cash-register"),
    ("dead-civilian", "dead-scav"),
    ("pmc-body", "dead-scav"),
    ("civilian-body", "dead-scav"),
    ("lab-technician-body", "dead-scav"),
    ("scav-body", "dead-scav"),
    ("medical-supply-crate", "crate"),
    ("ration-supply-crate", "crate"),
    ("technical-supply-crate", "crate"),
    ("shturmans-stash", "weapon-box"),
];

pub fn container_icon(group: &str) -> Option<&'static str> {
    let target = CONTAINER_ALIASES.iter().find(|(from, _)| *from == group).map_or(group, |(_, to)| *to);
    ICONS.iter().copied().find(|i| i.strip_prefix("container_") == Some(target))
}

pub fn is_known(name: &str) -> bool {
    ICONS.contains(&name)
}

pub fn path(dir: &Path, name: &str) -> Option<PathBuf> {
    is_known(name).then(|| dir.join(format!("{name}.png")))
}

/// Iconos ya descargados (para que la web sepa cuáles puede pedir).
pub fn available(dir: &Path) -> Vec<String> {
    ICONS.iter().filter(|n| dir.join(format!("{n}.png")).is_file()).map(|n| (*n).to_owned()).collect()
}

/// Valida un icono descargado: PNG real, pequeño y de lado razonable.
pub fn validate(bytes: &[u8]) -> AppResult<()> {
    if bytes.len() > MAX_ICON_BYTES {
        return Err(AppError::pack("icono demasiado grande"));
    }
    let png_size = |b: &[u8]| -> Option<(u32, u32)> {
        let w = u32::from_be_bytes(b.get(16..20)?.try_into().ok()?);
        let h = u32::from_be_bytes(b.get(20..24)?.try_into().ok()?);
        Some((w, h))
    };
    let (w, h) = png_size(bytes).ok_or_else(|| AppError::pack("icono no válido"))?;
    if w > MAX_ICON_SIDE || h > MAX_ICON_SIDE {
        return Err(AppError::pack("icono demasiado grande"));
    }
    check(Kind::Png, bytes, (w, h)).map(|_| ()).map_err(AppError::InvalidPack)
}

/// Guarda un icono con renombrado atómico.
pub fn store(dir: &Path, name: &str, bytes: &[u8]) -> AppResult<()> {
    let target = path(dir, name).ok_or(AppError::NotFound)?;
    validate(bytes)?;
    std::fs::create_dir_all(dir)?;
    let tmp = target.with_extension("png.tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, &target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_aliases_resolve_to_known_icons() {
        assert_eq!(container_icon("weapon-box"), Some("container_weapon-box"));
        assert_eq!(container_icon("scav-body"), Some("container_dead-scav"));
        assert_eq!(container_icon("shturmans-stash"), Some("container_weapon-box"));
        assert_eq!(container_icon("unknown-thing"), None);
        assert!(path(Path::new("/x"), "../config").is_none());
    }

    #[test]
    fn rejects_non_png_and_big_icons() {
        assert!(validate(b"<svg/>").is_err());
        let mut big = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
        big.extend_from_slice(&4000u32.to_be_bytes());
        big.extend_from_slice(&4000u32.to_be_bytes());
        assert!(validate(&big).is_err());
    }
}
