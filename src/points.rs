//! Puntos propios del usuario: por mapa, en coordenadas del juego, guardados en `points.json`.

use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use crate::maps::manifest::{is_unsafe_char, is_valid_id};
use crate::party::lock;
use crate::state::App;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const MAX_PER_MAP: usize = 500;
const MAX_MAPS: usize = 64;
const MAX_FILE: u64 = 4 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointKind {
    Marker,
    Loot,
    Danger,
    Key,
    Quest,
    Stash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserPoint {
    pub id: String,
    pub name: String,
    pub kind: PointKind,
    #[serde(default)]
    pub note: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn check_text(v: &str, max: usize, field: &str, allow_empty: bool) -> AppResult<String> {
    let t = v.trim();
    if (!allow_empty && t.is_empty()) || t.chars().count() > max || t.chars().any(is_unsafe_char) {
        return Err(AppError::bad(format!("{field}: hasta {max} caracteres, sin caracteres de control")));
    }
    Ok(t.to_owned())
}

impl UserPoint {
    fn valid(&self) -> bool {
        let id_ok = self.id.starts_with("pt-") && self.id.len() <= 24 && is_valid_id(&self.id);
        let coords = [self.x, self.y, self.z].iter().all(|v| v.is_finite() && v.abs() <= 100_000.0);
        id_ok
            && coords
            && check_text(&self.name, 40, "", false).is_ok()
            && check_text(&self.note, 200, "", true).is_ok()
    }
}

pub struct PointStore {
    path: PathBuf,
    pub by_map: BTreeMap<String, Vec<UserPoint>>,
}

impl PointStore {
    /// Carga tolerante: lo que no valide se descarta con un aviso.
    pub fn load(path: &Path) -> Self {
        let mut by_map: BTreeMap<String, Vec<UserPoint>> = match read_limited(path, MAX_FILE) {
            Ok(Some(b)) => serde_json::from_slice(&b).unwrap_or_else(|_| {
                tracing::warn!("points.json no es válido; se empieza sin puntos");
                BTreeMap::new()
            }),
            _ => BTreeMap::new(),
        };
        by_map.retain(|map, _| is_valid_id(map));
        for list in by_map.values_mut() {
            list.retain(UserPoint::valid);
            list.truncate(MAX_PER_MAP);
        }
        Self { path: path.to_path_buf(), by_map }
    }

    fn save(&self) -> AppResult<()> {
        let json = serde_json::to_vec_pretty(&self.by_map).map_err(|e| AppError::Internal(e.to_string()))?;
        let tmp = self.path.with_extension("json.tmp");
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    fn find_mut(&mut self, id: &str) -> Option<&mut UserPoint> {
        self.by_map.values_mut().flatten().find(|p| p.id == id)
    }
}

impl App {
    pub fn add_point(&self, px: f64, py: f64, name: &str, kind: PointKind, note: &str) -> AppResult<()> {
        if !px.is_finite() || !py.is_finite() {
            return Err(AppError::bad("coordenadas no válidas"));
        }
        let (name, note) = (check_text(name, 40, "nombre", false)?, check_text(note, 200, "nota", true)?);
        let inner = lock(&self.inner);
        let pack = App::active_pack(&inner).ok_or_else(|| AppError::bad("no hay mapa activo"))?;
        let y = inner.raid.last.as_ref().map_or(0.0, |l| l.pos.y);
        drop(inner);
        let (x, z) = pack.projection.unproject(px, py);
        let id = format!("pt-{}", crate::maps::import::random_hex(6)?);
        let mut store = lock(&self.points);
        if !store.by_map.contains_key(&pack.manifest.id) && store.by_map.len() >= MAX_MAPS {
            return Err(AppError::bad("demasiados mapas con puntos"));
        }
        let list = store.by_map.entry(pack.manifest.id.clone()).or_default();
        if list.len() >= MAX_PER_MAP {
            return Err(AppError::bad(format!("máximo {MAX_PER_MAP} puntos por mapa")));
        }
        list.push(UserPoint { id, name, kind, note, x, y, z });
        store.save()?;
        drop(store);
        self.notify();
        Ok(())
    }

    pub fn update_point(&self, id: &str, name: &str, kind: PointKind, note: &str) -> AppResult<()> {
        let (name, note) = (check_text(name, 40, "nombre", false)?, check_text(note, 200, "nota", true)?);
        let mut store = lock(&self.points);
        let p = store.find_mut(id).ok_or(AppError::NotFound)?;
        p.name = name;
        p.kind = kind;
        p.note = note;
        store.save()?;
        drop(store);
        self.notify();
        Ok(())
    }

    pub fn delete_point(&self, id: &str) -> AppResult<()> {
        let mut store = lock(&self.points);
        let before: usize = store.by_map.values().map(Vec::len).sum();
        for list in store.by_map.values_mut() {
            list.retain(|p| p.id != id);
        }
        store.by_map.retain(|_, l| !l.is_empty());
        if store.by_map.values().map(Vec::len).sum::<usize>() == before {
            return Err(AppError::NotFound);
        }
        store.save()?;
        drop(store);
        {
            let mut inner = lock(&self.inner);
            if inner.raid.selected.as_deref() == Some(id) {
                inner.raid.selected = None;
            }
        }
        self.notify();
        Ok(())
    }
}

#[cfg(test)]
#[path = "points_tests.rs"]
mod tests;
