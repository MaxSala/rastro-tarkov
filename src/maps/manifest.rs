//! Formato del paquete de mapa (ver docs/MAP_PACK.md) y su validación estricta.

use super::manifest_check::{check_coord, check_text};
use super::project::Projection;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Versión que escribe esta app. Se aceptan la 1 (sin marcadores) y la 2.
pub const SCHEMA_VERSION: u32 = 2;
pub const MAX_MARKERS: usize = 3000;
pub const MAX_EXTRACTS: usize = 500;
pub const MAX_LEVELS: usize = 16;
pub const MAX_IMAGE_SIDE: u32 = 8192;
pub(super) const MAX_COORD: f64 = 100_000.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub image: ImageRef,
    pub calibration: Calibration,
    #[serde(default)]
    pub levels: Vec<Level>,
    #[serde(default)]
    pub extracts: Vec<Extract>,
    /// Capas opcionales (v2): contenedores de loot, transiciones e interruptores.
    #[serde(default)]
    pub markers: Vec<Marker>,
    /// Nombres internos del mapa en el juego (logs), p. ej. «bigmap»: detección automática.
    #[serde(default)]
    pub game_ids: Vec<String>,
    /// Duración de la raid en minutos (temporizador).
    #[serde(default)]
    pub raid_minutes: Option<u32>,
    pub credits: Credits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerKind {
    Loot,
    Transit,
    Switch,
    Lock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Marker {
    pub kind: MarkerKind,
    /// Tipo de contenedor (normalizedName de tarkov.dev) para filtrar y elegir icono.
    #[serde(default)]
    pub group: Option<String>,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Solo en `lock`: id (24 hex) del objeto llave en tarkov.dev y si necesita corriente.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub power: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageRef {
    pub file: String,
    pub width: u32,
    pub height: u32,
    /// Solo SVG con pisos: id del grupo `<g>` que siempre se ve (planta base).
    #[serde(default)]
    pub base_layer: Option<String>,
}

/// Dos puntos de referencia: coordenadas del juego (x, z) → píxel (x, y).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Calibration {
    pub points: [RefPoint; 2],
    #[serde(default)]
    pub invert_x: bool,
    #[serde(default)]
    pub invert_z: bool,
    /// Rotación en grados, sentido horario: 0, 90, 180 o 270.
    #[serde(default)]
    pub rotation: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefPoint {
    pub game: [f64; 2],
    pub pixel: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub id: String,
    pub name: String,
    pub y_min: f64,
    pub y_max: f64,
    /// Grupo `<g>` del SVG que corresponde a este piso (si el mapa es un SVG por capas).
    #[serde(default)]
    pub svg_layer: Option<String>,
    /// Zonas (rectángulo x/z + rango de altura) donde este piso es el que toca: selección automática.
    #[serde(default)]
    pub zones: Vec<Zone>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zone {
    pub x: [f64; 2],
    pub z: [f64; 2],
    pub y: [f64; 2],
}

impl Zone {
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        let inside = |v: f64, r: [f64; 2]| v >= r[0].min(r[1]) && v <= r[0].max(r[1]);
        inside(x, self.x) && inside(z, self.z) && inside(y, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Faction {
    Pmc,
    Scav,
    Shared,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extract {
    pub id: String,
    pub name: String,
    pub faction: Faction,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default)]
    pub conditions: String,
    /// Switches to activate first; the interface writes "Requires: …" in its language.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub switches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credits {
    pub author: String,
    pub license: String,
    pub source: String,
}

fn id_regex() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9-]{1,40}$").ok()).as_ref()
}

/// Id de grupo SVG: letras, dígitos, `_` y `-`, empezando por letra (seguro en un selector CSS).
pub fn is_valid_layer_id(id: &str) -> bool {
    let mut chars = id.chars();
    id.len() <= 40
        && chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

pub fn is_valid_id(id: &str) -> bool {
    id_regex().is_some_and(|re| re.is_match(id))
}

/// Caracteres de control y de formato bidireccional/invisibles (evitan suplantación visual).
pub fn is_unsafe_char(c: char) -> bool {
    c.is_control()
        || matches!(c, '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{FEFF}')
}

impl Manifest {
    pub fn validate(&self) -> Result<Projection, String> {
        if !(1..=SCHEMA_VERSION).contains(&self.schema_version) {
            return Err(format!("schema version {} is not supported (expected {SCHEMA_VERSION})", self.schema_version));
        }
        if !is_valid_id(&self.id) {
            return Err("the id must match ^[a-z0-9-]{1,40}$".into());
        }
        check_text("name", &self.name, 60, false)?;
        super::image::Kind::from_file_name(&self.image.file)
            .ok_or("the image name is not valid (svg, webp, png or jpg)")?;
        let side = 1..=MAX_IMAGE_SIDE;
        if !side.contains(&self.image.width) || !side.contains(&self.image.height) {
            return Err(format!("the image must be between 1 and {MAX_IMAGE_SIDE} px per side"));
        }
        for p in &self.calibration.points {
            for v in p.game.iter().chain(p.pixel.iter()) {
                check_coord("calibration", *v)?;
            }
        }
        if self.levels.len() > MAX_LEVELS {
            return Err(format!("at most {MAX_LEVELS} levels"));
        }
        for l in &self.levels {
            if !is_valid_id(&l.id) {
                return Err("invalid level id".into());
            }
            check_text("levels.name", &l.name, 40, false)?;
            check_coord("levels.y_min", l.y_min)?;
            check_coord("levels.y_max", l.y_max)?;
            if l.svg_layer.as_deref().is_some_and(|g| !is_valid_layer_id(g)) {
                return Err("invalid levels.svg_layer".into());
            }
            if l.zones.len() > 64 {
                return Err("at most 64 zones per floor".into());
            }
            for z in &l.zones {
                for v in z.x.iter().chain(z.z.iter()).chain(z.y.iter()) {
                    check_coord("levels.zones", *v)?;
                }
            }
        }
        if self.image.base_layer.as_deref().is_some_and(|g| !is_valid_layer_id(g)) {
            return Err("invalid image.base_layer".into());
        }
        if self.game_ids.len() > 8 || !self.game_ids.iter().all(|g| crate::gamelog::parse::valid_location(g)) {
            return Err("invalid game_ids".into());
        }
        if self.raid_minutes.is_some_and(|m| !(1..=600).contains(&m)) {
            return Err("raid_minutes out of range".into());
        }
        self.validate_extracts()?;
        if self.schema_version < 2 && !self.markers.is_empty() {
            return Err("markers require schema_version 2".into());
        }
        if self.markers.len() > MAX_MARKERS {
            return Err(format!("at most {MAX_MARKERS} markers"));
        }
        for mk in &self.markers {
            // Las cerraduras pueden no tener nombre: se toma del catálogo por el id de la llave.
            check_text("markers.name", &mk.name, 80, mk.kind == MarkerKind::Lock)?;
            let bad_key = mk.key.as_deref().is_some_and(|k| !crate::market::icons::valid_id(k));
            if mk.group.as_deref().is_some_and(|g| !is_valid_id(g)) || bad_key {
                return Err("invalid markers.group or markers.key".into());
            }
            for v in [mk.x, mk.y, mk.z] {
                check_coord("markers.x/y/z", v)?;
            }
        }
        check_text("credits.author", &self.credits.author, 100, false)?;
        check_text("credits.license", &self.credits.license, 100, false)?;
        check_text("credits.source", &self.credits.source, 300, true)?;
        Projection::new(&self.calibration)
    }
}
