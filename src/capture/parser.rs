//! Parser del nombre de archivo de las capturas. Nunca abre la imagen.

use regex::Regex;
use std::sync::OnceLock;

/// Nombre real de ejemplo:
/// `2026-08-07[19-31]_-720.10, -48.62, 430.51_-0.03195, 0.66069, -0.02987, -0.74938_11.67 (0).png`
///
/// Grupos: fecha (1-3), hora y minuto (4-5), segundos opcionales (6),
/// posición x, y, z (7-9), cuaternión x, y, z, w (10-13), tiempo de raid en horas (14),
/// sufijo de duplicado opcional (15) y extensión.
const CAPTURE_PATTERN: &str = concat!(
    r"^(\d{4})-(\d{2})-(\d{2})\[(\d{2})-(\d{2})(?:-(\d{2}))?\]_",
    r"(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?)_",
    r"(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?)_",
    r"(\d+(?:\.\d+)?)",
    r"(?:\s*\((\d{1,4})\))?",
    r"\.(?i:png|jpg|jpeg|bmp)$"
);

/// Límite razonable de coordenadas del juego (metros).
const MAX_COORD: f64 = 100_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Cuaternión de Unity en el orden del nombre: x, y, z, w.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Capture {
    /// Fecha y hora local del juego, normalizada a `AAAA-MM-DD HH:MM[:SS]`.
    pub taken_at: String,
    pub pos: Vec3,
    pub rot: Quat,
    pub raid_hours: f64,
}

fn regex() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(CAPTURE_PATTERN).ok()).as_ref()
}

/// Devuelve `None` si el nombre no es una captura válida (no es un error: hay otros archivos).
pub fn parse(file_name: &str) -> Option<Capture> {
    if file_name.len() > 255 {
        return None;
    }
    let caps = regex()?.captures(file_name)?;
    let num = |i: usize| -> Option<f64> {
        let v: f64 = caps.get(i)?.as_str().parse().ok()?;
        v.is_finite().then_some(v)
    };
    let txt = |i: usize| caps.get(i).map(|m| m.as_str());

    let pos = Vec3 { x: num(7)?, y: num(8)?, z: num(9)? };
    if [pos.x, pos.y, pos.z].iter().any(|c| c.abs() > MAX_COORD) {
        return None;
    }
    let rot = Quat { x: num(10)?, y: num(11)?, z: num(12)?, w: num(13)? };
    let norm = (rot.x * rot.x + rot.y * rot.y + rot.z * rot.z + rot.w * rot.w).sqrt();
    if !(0.9..=1.1).contains(&norm) {
        return None;
    }
    let raid_hours = num(14)?;
    if raid_hours > 48.0 {
        return None;
    }
    let mut taken_at = format!("{}-{}-{} {}:{}", txt(1)?, txt(2)?, txt(3)?, txt(4)?, txt(5)?);
    if let Some(s) = txt(6) {
        taken_at.push(':');
        taken_at.push_str(s);
    }
    Some(Capture { taken_at, pos, rot, raid_hours })
}

#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
