//! Documentos JSON de json.tarkov.dev: `regular/maps` (datos) y `regular/maps_es` (textos).
//! Los nombres del documento de datos son claves que se resuelven en el de textos.
//! Formato tomado como referencia de herramientas comunitarias; todo se trata como no fiable.

use super::api::{make_extract, make_marker, make_marker_in, MapData};
use super::locks::make_lock;
use crate::maps::manifest::{MarkerKind, MAX_EXTRACTS, MAX_MARKERS};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

struct Locale<'a>(Option<&'a serde_json::Map<String, Value>>);

impl Locale<'_> {
    fn text(&self, key: &str) -> Option<String> {
        let map = self.0?;
        [key.to_owned(), format!("{key} name"), format!("{key} Name"), format!("{key} Description")]
            .iter()
            .find_map(|k| map.get(k).and_then(Value::as_str))
            .filter(|s| !s.trim().is_empty())
            .map(str::to_owned)
    }

    /// Resuelve una clave; si no hay traducción, usa la propia clave si parece un texto.
    fn resolve(&self, key: &str) -> String {
        self.text(key).unwrap_or_else(|| {
            if key.len() <= 60 && !key.chars().all(|c| c.is_ascii_hexdigit()) {
                key.to_owned()
            } else {
                String::new()
            }
        })
    }
}

fn pos(v: &Value) -> Option<(f64, f64, f64)> {
    let p = v.get("position")?;
    let n = |k: &str| p.get(k).and_then(Value::as_f64);
    Some((n("x")?, n("y").unwrap_or(0.0), n("z")?))
}

fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(Value::as_str).unwrap_or("")
}

fn list<'a>(v: &'a Value, k: &str) -> impl Iterator<Item = &'a Value> {
    v.get(k).and_then(Value::as_array).into_iter().flatten()
}

/// `locale_en` sirve para deducir el tipo de contenedor (normalizedName) a partir del nombre inglés.
pub fn parse(maps_doc: &[u8], locale_doc: &[u8], locale_en: &[u8]) -> Result<Vec<MapData>, String> {
    let data: Value = serde_json::from_slice(maps_doc).map_err(|e| format!("invalid maps document: {e}"))?;
    let loc: Value = serde_json::from_slice(locale_doc).unwrap_or(Value::Null);
    let locale = Locale(loc.get("data").unwrap_or(&loc).as_object());
    let loc_en: Value = serde_json::from_slice(locale_en).unwrap_or(Value::Null);
    let english = Locale(loc_en.get("data").unwrap_or(&loc_en).as_object());
    let root = data.get("data").unwrap_or(&data);
    let maps = root.get("maps").unwrap_or(root);
    let entries: Vec<&Value> = match maps {
        Value::Object(o) => o.values().collect(),
        Value::Array(a) => a.iter().collect(),
        _ => return Err("the maps document has no maps".into()),
    };
    let mut out = Vec::new();
    for m in entries {
        let normalized = s(m, "normalizedName").to_owned();
        if normalized.is_empty() {
            continue;
        }
        let switch_names: HashMap<&str, String> =
            list(m, "switches").map(|sw| (s(sw, "id"), locale.resolve(s(sw, "name")))).collect();
        let mut used = HashSet::new();
        let extracts = list(m, "extracts")
            .filter_map(|e| {
                let (x, y, z) = pos(e)?;
                let switches: Vec<String> = list(e, "switches")
                    .filter_map(|id| id.as_str().and_then(|id| switch_names.get(id)).filter(|n| !n.is_empty()).cloned())
                    .collect();
                make_extract(
                    s(e, "id"),
                    &locale.resolve(s(e, "name")),
                    s(e, "faction"),
                    &switches,
                    (x, y, z),
                    &mut used,
                )
            })
            .take(MAX_EXTRACTS)
            .collect();
        let transits = list(m, "transits").filter_map(|t| {
            let (x, y, z) = pos(t)?;
            make_marker(MarkerKind::Transit, &locale.resolve(s(t, "description")), "Transit", x, y, z)
        });
        let switches = list(m, "switches").filter_map(|sw| {
            let (x, y, z) = pos(sw)?;
            make_marker(MarkerKind::Switch, &locale.resolve(s(sw, "name")), "Switch", x, y, z)
        });
        let loot = list(m, "lootContainers").filter_map(|l| {
            let (x, y, z) = pos(l)?;
            // lootContainer puede ser un id (clave de texto) o un objeto con nombre.
            let c = l.get("lootContainer");
            let key = c
                .and_then(Value::as_str)
                .or_else(|| c.and_then(|c| c.get("name")).and_then(Value::as_str))
                .unwrap_or("");
            let normalized = c.and_then(|c| c.get("normalizedName")).and_then(Value::as_str).map(str::to_owned);
            let group = normalized.or_else(|| english.text(key)).or_else(|| locale.text(key));
            make_marker_in(MarkerKind::Loot, group.as_deref(), &locale.resolve(key), "Container", (x, y, z))
        });
        // `key` es el id del objeto (o un objeto con `id` en otras versiones del documento).
        let locks = list(m, "locks").filter_map(|l| {
            let k = l.get("key");
            let key = k.and_then(Value::as_str).or_else(|| k.and_then(|k| k.get("id")).and_then(Value::as_str))?;
            let power = l.get("needsPower").and_then(Value::as_bool).unwrap_or(false);
            let name = locale.text(&format!("{key} Name")).unwrap_or_default();
            make_lock(key, s(l, "lockType"), power, &name, pos(l)?)
        });
        // Las cerraduras van antes del botín: si hay que recortar, que sea por los contenedores.
        let markers = transits.chain(switches).chain(locks).chain(loot).take(MAX_MARKERS).collect();
        let name = locale
            .text(s(m, "id"))
            .or_else(|| locale.text(s(m, "name")))
            .or_else(|| Some(s(m, "name").to_owned()).filter(|n| !n.is_empty()));
        out.push(MapData {
            normalized_name: normalized,
            name: name.map(|n| super::api::clean(Some(&n), 60, "Map")),
            extracts,
            markers,
            name_id: super::api::name_id(m.get("nameId").and_then(Value::as_str)),
            raid_minutes: super::api::minutes(m.get("raidDuration").and_then(Value::as_f64)),
        });
    }
    if out.is_empty() {
        return Err("the document contains no recognisable maps".into());
    }
    Ok(out)
}

#[cfg(test)]
#[path = "jsondoc_tests.rs"]
mod tests;
