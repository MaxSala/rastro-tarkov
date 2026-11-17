//! Cerraduras de tarkov.dev: puertas, contenedores y maleteros que se abren con una llave.
//! Solo se guarda el id de la llave; su nombre, icono y precio salen del catálogo del Mercado.

use super::api::clean;
use crate::maps::manifest::{Marker, MarkerKind};
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct LockPos {
    x: f64,
    #[serde(default)]
    y: f64,
    z: f64,
}

#[derive(Deserialize)]
pub struct ApiKey {
    id: Option<String>,
    name: Option<String>,
}

/// Forma de la API GraphQL: `locks { lockType key { id name } needsPower position { x y z } }`.
#[derive(Deserialize)]
pub struct ApiLock {
    #[serde(rename = "lockType")]
    lock_type: Option<String>,
    key: Option<ApiKey>,
    #[serde(rename = "needsPower")]
    needs_power: Option<bool>,
    position: Option<LockPos>,
}

impl ApiLock {
    pub fn marker(&self) -> Option<Marker> {
        let key = self.key.as_ref()?;
        let p = self.position?;
        make_lock(
            key.id.as_deref()?,
            self.lock_type.as_deref().unwrap_or(""),
            self.needs_power.unwrap_or(false),
            key.name.as_deref().unwrap_or(""),
            (p.x, p.y, p.z),
        )
    }
}

/// Tipo de cerradura normalizado; lo desconocido queda como `lock`.
fn lock_type(raw: &str) -> &'static str {
    match raw {
        "door" => "door",
        "container" => "container",
        "trunk" => "trunk",
        _ => "lock",
    }
}

pub fn make_lock(key: &str, kind: &str, power: bool, name: &str, (x, y, z): (f64, f64, f64)) -> Option<Marker> {
    if !crate::market::icons::valid_id(key) || ![x, y, z].iter().all(|v| v.is_finite() && v.abs() < 100_000.0) {
        return None;
    }
    Some(Marker {
        kind: MarkerKind::Lock,
        group: Some(lock_type(kind).to_owned()),
        name: clean(Some(name), 80, ""),
        x,
        y,
        z,
        key: Some(key.to_ascii_lowercase()),
        power,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locks_are_validated() {
        let ok = make_lock("5780cf7f2459777de4559322", "door", true, "Llave\u{202e} de la 314", (1.0, 2.0, 3.0))
            .expect("cerradura");
        assert_eq!((ok.group.as_deref(), ok.power), (Some("door"), true));
        assert!(!ok.name.contains('\u{202e}'));
        assert_eq!(
            make_lock("5780cf7f2459777de4559322", "raro", false, "", (0.0, 0.0, 0.0)).and_then(|m| m.group),
            Some("lock".into())
        );
        assert!(make_lock("../../etc", "door", false, "", (0.0, 0.0, 0.0)).is_none());
        // Sin nombre (lo normal en json.tarkov.dev) el paquete debe seguir siendo válido.
        let unnamed = make_lock("5780cf7f2459777de4559322", "door", false, "", (1.0, 1.0, 1.0)).expect("sin nombre");
        assert_eq!(unnamed.name, "");
        assert!(make_lock("5780cf7f2459777de4559322", "door", false, "", (f64::NAN, 0.0, 0.0)).is_none());
    }

    #[test]
    fn unnamed_locks_keep_the_pack_valid() {
        let raw = include_str!("../../assets/demo/manifest.json");
        let mut m: crate::maps::manifest::Manifest = serde_json::from_str(raw).expect("demo");
        m.schema_version = m.schema_version.max(2);
        m.markers.push(make_lock("5780cf7f2459777de4559322", "door", false, "", (1.0, 1.0, 1.0)).expect("cerradura"));
        assert!(m.validate().is_ok(), "{:?}", m.validate().err());
        if let Some(l) = m.markers.last_mut() {
            l.key = Some("no-hex".into());
        }
        assert!(m.validate().is_err(), "llave no válida en un paquete importado");
    }
}
