//! Datos de mapa de la API GraphQL de tarkov.dev (esquema: the-hideout/tarkov-api, schema-static).
//! Todo campo es opcional y se sanea: textos acotados, ids válidos y coordenadas finitas.

use crate::maps::manifest::{is_unsafe_char, Extract, Faction, Marker, MarkerKind, MAX_EXTRACTS, MAX_MARKERS};
use serde::Deserialize;
use std::collections::HashSet;

/// Solo campos de lectura pública; sin autenticación. `{lang}`: es o en; `{mode}`: regular o pve.
pub const QUERY: &str = "{ maps(lang: {lang}, gameMode: {mode}) { normalizedName name nameId raidDuration \
extracts { id name faction switches { name } position { x y z } } \
transits { description position { x y z } } \
switches { name position { x y z } } \
lootContainers { lootContainer { name normalizedName } position { x y z } } \
locks { lockType key { id name } needsPower position { x y z } } } }";

#[derive(Deserialize, Clone, Copy)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Deserialize)]
struct Named {
    name: Option<String>,
    #[serde(rename = "normalizedName", default)]
    normalized_name: Option<String>,
}

#[derive(Deserialize)]
struct ApiExtract {
    id: Option<String>,
    name: Option<String>,
    faction: Option<String>,
    #[serde(default)]
    switches: Option<Vec<Named>>,
    position: Option<Pos>,
}

#[derive(Deserialize)]
struct ApiTransit {
    description: Option<String>,
    position: Option<Pos>,
}

#[derive(Deserialize)]
struct ApiSwitch {
    name: Option<String>,
    position: Option<Pos>,
}

#[derive(Deserialize)]
struct ApiLoot {
    #[serde(rename = "lootContainer")]
    loot_container: Option<Named>,
    position: Option<Pos>,
}

#[derive(Deserialize)]
pub struct ApiMap {
    #[serde(rename = "normalizedName")]
    pub normalized_name: String,
    pub name: Option<String>,
    #[serde(rename = "nameId", default)]
    name_id: Option<String>,
    #[serde(rename = "raidDuration", default)]
    raid_duration: Option<f64>,
    extracts: Option<Vec<ApiExtract>>,
    transits: Option<Vec<ApiTransit>>,
    switches: Option<Vec<ApiSwitch>>,
    #[serde(rename = "lootContainers")]
    loot_containers: Option<Vec<ApiLoot>>,
    locks: Option<Vec<super::locks::ApiLock>>,
}

#[derive(Deserialize)]
struct Data {
    maps: Vec<ApiMap>,
}

#[derive(Deserialize)]
struct Response {
    data: Option<Data>,
}

pub fn parse(bytes: &[u8]) -> Result<Vec<ApiMap>, String> {
    let r: Response = serde_json::from_slice(bytes).map_err(|e| format!("invalid API response: {e}"))?;
    r.data.map(|d| d.maps).ok_or_else(|| "the API returned no maps".to_string())
}

/// Texto visible saneado: sin caracteres de control/bidi, recortado a `max` caracteres.
pub fn clean(raw: Option<&str>, max: usize, fallback: &str) -> String {
    let s: String = raw.unwrap_or("").chars().filter(|c| !is_unsafe_char(*c)).collect();
    let s = s.trim();
    let s: String = s.chars().take(max).collect();
    let s = s.trim().to_owned();
    if s.is_empty() {
        fallback.to_owned()
    } else {
        s
    }
}

fn ok_pos(p: Option<Pos>) -> Option<Pos> {
    p.filter(|p| [p.x, p.y, p.z].iter().all(|v| v.is_finite() && v.abs() <= 100_000.0))
}

fn unique_id(raw: &str, used: &mut HashSet<String>) -> String {
    let base: String =
        raw.to_ascii_lowercase().chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-').take(34).collect();
    let base = if base.is_empty() { "ex".to_owned() } else { base };
    let mut id = base.clone();
    let mut n = 2;
    while !used.insert(id.clone()) {
        id = format!("{base}-{n}");
        n += 1;
    }
    id
}

/// Datos de un mapa ya saneados, vengan de GraphQL o de json.tarkov.dev.
#[derive(Debug, Clone)]
pub struct MapData {
    pub normalized_name: String,
    pub name: Option<String>,
    pub extracts: Vec<Extract>,
    pub markers: Vec<Marker>,
    /// Nombre interno del juego (el que aparece en los logs).
    pub name_id: Option<String>,
    pub raid_minutes: Option<u32>,
}

/// Duración en minutos, solo si es razonable.
pub fn minutes(v: Option<f64>) -> Option<u32> {
    v.filter(|m| m.is_finite() && (1.0..=600.0).contains(m)).map(|m| m.round() as u32)
}

pub fn name_id(v: Option<&str>) -> Option<String> {
    v.filter(|s| crate::gamelog::parse::valid_location(s)).map(str::to_ascii_lowercase)
}

pub fn to_map_data(maps: Vec<ApiMap>) -> Vec<MapData> {
    maps.into_iter()
        .map(|m| MapData {
            name: m.display_name(),
            extracts: m.extracts(),
            markers: m.markers(),
            name_id: name_id(m.name_id.as_deref()),
            raid_minutes: minutes(m.raid_duration),
            normalized_name: m.normalized_name,
        })
        .collect()
}

/// Construye una extracción saneada (compartido con json.tarkov.dev).
pub fn make_extract(
    id: &str,
    name: &str,
    faction: &str,
    switches: &[String],
    (x, y, z): (f64, f64, f64),
    used: &mut HashSet<String>,
) -> Option<Extract> {
    ok_pos(Some(Pos { x, y, z }))?;
    let faction = match faction {
        "pmc" => Faction::Pmc,
        "scav" => Faction::Scav,
        _ => Faction::Shared,
    };
    Some(Extract {
        id: unique_id(id, used),
        name: clean(Some(name), 60, "Extract"),
        faction,
        x,
        y,
        z,
        conditions: String::new(),
        switches: clean_switches(switches),
    })
}

/// Switch names for "Requires: …", cleaned and bounded like the manifest expects.
pub fn clean_switches(names: &[String]) -> Vec<String> {
    names.iter().map(|n| clean(Some(n), 60, "")).filter(|n| !n.is_empty()).take(8).collect()
}

/// normalizedName al estilo de tarkov.dev: minúsculas, sin apóstrofos, el resto de símbolos a guiones.
pub fn normalize_name(name: &str) -> String {
    let mut out = String::new();
    for c in name.to_lowercase().chars().filter(|c| *c != '\'' && *c != '’') {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').chars().take(40).collect()
}

fn group_of(raw: Option<&str>) -> Option<String> {
    raw.map(normalize_name).filter(|g| crate::maps::manifest::is_valid_id(g))
}

pub fn make_marker(kind: MarkerKind, name: &str, fallback: &str, x: f64, y: f64, z: f64) -> Option<Marker> {
    make_marker_in(kind, None, name, fallback, (x, y, z))
}

pub fn make_marker_in(
    kind: MarkerKind,
    group: Option<&str>,
    name: &str,
    fallback: &str,
    (x, y, z): (f64, f64, f64),
) -> Option<Marker> {
    let group = group_of(group);
    ok_pos(Some(Pos { x, y, z })).map(|p| Marker {
        group,
        kind,
        key: None,
        power: false,
        name: clean(Some(name), 80, fallback),
        x: p.x,
        y: p.y,
        z: p.z,
    })
}

impl ApiMap {
    pub fn display_name(&self) -> Option<String> {
        self.name.as_deref().map(|n| clean(Some(n), 60, "Map"))
    }

    pub fn extracts(&self) -> Vec<Extract> {
        let mut used = HashSet::new();
        let list = self.extracts.as_deref().unwrap_or_default();
        list.iter()
            .filter_map(|e| {
                let p = ok_pos(e.position)?;
                let faction = match e.faction.as_deref() {
                    Some("pmc") => Faction::Pmc,
                    Some("scav") => Faction::Scav,
                    _ => Faction::Shared,
                };
                let switches: Vec<String> =
                    e.switches.as_deref().unwrap_or_default().iter().filter_map(|s| s.name.clone()).collect();
                Some(Extract {
                    id: unique_id(e.id.as_deref().unwrap_or("ex"), &mut used),
                    name: clean(e.name.as_deref(), 60, "Extract"),
                    faction,
                    x: p.x,
                    y: p.y,
                    z: p.z,
                    conditions: String::new(),
                    switches: clean_switches(&switches),
                })
            })
            .take(MAX_EXTRACTS)
            .collect()
    }

    pub fn markers(&self) -> Vec<Marker> {
        let mk = |kind, name: Option<&str>, fallback: &str, p: Option<Pos>| {
            let name = clean(name, 80, fallback);
            ok_pos(p).map(|p| Marker { kind, group: None, name, x: p.x, y: p.y, z: p.z, key: None, power: false })
        };
        let transits = self.transits.as_deref().unwrap_or_default().iter();
        let switches = self.switches.as_deref().unwrap_or_default().iter();
        let loot = self.loot_containers.as_deref().unwrap_or_default().iter();
        transits
            .filter_map(|t| mk(MarkerKind::Transit, t.description.as_deref(), "Transit", t.position))
            .chain(switches.filter_map(|s| mk(MarkerKind::Switch, s.name.as_deref(), "Switch", s.position)))
            .chain(self.locks.as_deref().unwrap_or_default().iter().filter_map(super::locks::ApiLock::marker))
            .chain(loot.filter_map(|l| {
                let c = l.loot_container.as_ref();
                let name = c.and_then(|c| c.name.as_deref()).unwrap_or("");
                let group = c.and_then(|c| c.normalized_name.as_deref().or(c.name.as_deref()));
                let p = ok_pos(l.position)?;
                make_marker_in(MarkerKind::Loot, group, name, "Container", (p.x, p.y, p.z))
            }))
            .take(MAX_MARKERS)
            .collect()
    }
}

#[cfg(test)]
#[path = "api_tests.rs"]
mod tests;
