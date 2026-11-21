//! Bosses and other named AI: where each one can appear on each map, and how likely it is.
//!
//! They come with the quests, from the same per-mode download (`{mode}/maps`), so PvP and PvE
//! each keep their own chances. json.tarkov.dev gives the exact spawn points of every zone; the
//! GraphQL API only names the zones, so their points are looked up in the map's `spawns`.
//! Both arrive in the GraphQL shape ([`from_json`] converts the static file) and go through the
//! same checks: unknown parts are skipped, texts cleaned and every list capped.

use super::jsondoc::{entries, tr};
use super::pack_id_for;
use crate::maps::manifest::is_valid_id;
use crate::market::raw::Locale;
use crate::tarkovdev::api::clean;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};

const MAX_BOSSES: usize = 400;
const MAX_ZONES: usize = 16;
const MAX_POINTS: usize = 64;
const MAX_ESCORTS: usize = 6;
/// Spawn times past this are placeholders (events use 9999), not a time to show.
const MAX_TIME: f64 = 7200.0;
/// Variants of a map that are a raid of their own, with their own bosses: never merged into the
/// pack they share (night Factory is not day Factory).
const OWN_RAID: &[&str] = &["night-factory", "ground-zero-tutorial"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Boss {
    /// Mob id, e.g. `bossBully`.
    pub id: String,
    pub name: String,
    /// Pack id of the map.
    pub map: String,
    /// Chance of it being in a raid on this map, from 0 to 1.
    pub chance: f64,
    /// Seconds into the raid it arrives, when it doesn't come at the start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<u32>,
    /// Something on the map has to be activated first (a switch, a lever…).
    #[serde(default)]
    pub triggered: bool,
    #[serde(default)]
    pub escorts: Vec<String>,
    pub zones: Vec<BossZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BossZone {
    pub key: String,
    /// Empty when tarkov.dev has no name for it.
    pub name: String,
    /// Chance of this zone when it comes, from 0 to 1.
    pub chance: f64,
    /// Spawn points in game coordinates. Empty: the zone is known only by name.
    pub points: Vec<[f64; 3]>,
}

/// A list that may come as `null`.
pub(super) fn list<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Vec<T>, D::Error> {
    Ok(Option::<Vec<T>>::deserialize(d)?.unwrap_or_default())
}

/// A number, or nothing if the value is anything else: one odd value never sinks the download.
pub(super) fn num<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    Ok(Value::deserialize(d)?.as_f64())
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default)]
pub struct Pos {
    #[serde(deserialize_with = "num")]
    x: Option<f64>,
    #[serde(deserialize_with = "num")]
    y: Option<f64>,
    #[serde(deserialize_with = "num")]
    z: Option<f64>,
}

impl Pos {
    fn point(self) -> Option<[f64; 3]> {
        let p = [self.x?, self.y.unwrap_or(0.0), self.z?];
        p.iter().all(|v| v.is_finite() && v.abs() <= 100_000.0).then_some(p)
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawMap {
    pub normalized_name: Option<String>,
    #[serde(deserialize_with = "list")]
    pub bosses: Vec<Option<RawBoss>>,
    #[serde(deserialize_with = "list")]
    pub spawns: Vec<Option<RawSpawn>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawBoss {
    boss: Option<RawMob>,
    #[serde(deserialize_with = "num")]
    spawn_chance: Option<f64>,
    #[serde(deserialize_with = "num")]
    spawn_time: Option<f64>,
    spawn_trigger: Option<String>,
    #[serde(deserialize_with = "list")]
    escorts: Vec<Option<RawEscort>>,
    #[serde(deserialize_with = "list")]
    spawn_locations: Vec<Option<RawLocation>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawMob {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawEscort {
    boss: Option<RawMob>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawLocation {
    spawn_key: Option<String>,
    name: Option<String>,
    #[serde(deserialize_with = "num")]
    chance: Option<f64>,
    /// Only in json.tarkov.dev's file.
    #[serde(deserialize_with = "list")]
    positions: Vec<Option<Pos>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawSpawn {
    zone_name: Option<String>,
    position: Option<Pos>,
    #[serde(deserialize_with = "list")]
    categories: Vec<Option<String>>,
}

/// Mob ids and zone keys are short identifiers (`bossBully`, `ZoneDormitory`).
fn ident(s: &str) -> bool {
    (1..=60).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn chance(v: Option<f64>) -> Option<f64> {
    v.filter(|c| c.is_finite() && *c > 0.0).map(|c| c.min(1.0))
}

fn zone(l: &RawLocation, spawns: &[Option<RawSpawn>]) -> Option<BossZone> {
    let key = l.spawn_key.as_deref().filter(|k| ident(k))?.to_owned();
    let mut points: Vec<[f64; 3]> = l.positions.iter().flatten().filter_map(|p| p.point()).collect();
    if points.is_empty() {
        points = spawns
            .iter()
            .flatten()
            .filter(|s| s.zone_name.as_deref() == Some(key.as_str()))
            .filter(|s| s.categories.iter().flatten().any(|c| c == "boss"))
            .filter_map(|s| s.position.and_then(Pos::point))
            .collect();
    }
    points.truncate(MAX_POINTS);
    // An untranslated zone only has its internal key (`1BD1PortAmbush1`): no name to show.
    let name = clean(l.name.as_deref(), 60, "");
    let name = if name == key { String::new() } else { name };
    Some(BossZone { name, chance: chance(l.chance)?, points, key })
}

/// Whether the wave has a placeholder time rather than a real one.
fn late(b: &RawBoss) -> bool {
    b.spawn_time.is_some_and(|t| t >= MAX_TIME)
}

fn boss(b: &RawBoss, map: &str, spawns: &[Option<RawSpawn>]) -> Option<Boss> {
    let mob = b.boss.as_ref()?;
    let id = mob.id.as_deref().filter(|s| ident(s))?.to_owned();
    let name = clean(mob.name.as_deref(), 60, &id);
    let mut escorts: Vec<String> = Vec::new();
    for e in b.escorts.iter().flatten().filter_map(|e| e.boss.as_ref()) {
        let n = clean(e.name.as_deref(), 60, "");
        if !n.is_empty() && n != name && !escorts.contains(&n) && escorts.len() < MAX_ESCORTS {
            escorts.push(n);
        }
    }
    Some(Boss {
        chance: chance(b.spawn_chance)?,
        time: b.spawn_time.filter(|t| t.is_finite() && *t > 0.0 && *t < MAX_TIME).map(|t| t.round() as u32),
        triggered: b.spawn_trigger.as_deref().is_some_and(|t| !t.trim().is_empty()),
        escorts,
        zones: b.spawn_locations.iter().flatten().filter_map(|l| zone(l, spawns)).take(MAX_ZONES).collect(),
        map: map.to_owned(),
        name,
        id,
    })
}

/// Another wave of a boss already listed (same zones): one entry, as likely as the likeliest
/// wave, coming as early as the earliest. Waves with a placeholder time say nothing of when.
fn merge(into: &mut (Boss, bool), b: (Boss, bool)) {
    let ((into, into_late), (b, b_late)) = (into, b);
    into.chance = into.chance.max(b.chance);
    match (*into_late, b_late) {
        (true, false) => (into.time, into.triggered, *into_late) = (b.time, b.triggered, false),
        (false, false) => {
            into.time = into.time.zip(b.time).map(|(a, c)| a.min(c));
            into.triggered &= b.triggered;
        }
        _ => {}
    }
    for e in b.escorts {
        if !into.escorts.contains(&e) && into.escorts.len() < MAX_ESCORTS {
            into.escorts.push(e);
        }
    }
}

fn same_zones(a: &Boss, b: &Boss) -> bool {
    a.zones.len() == b.zones.len() && a.zones.iter().all(|z| b.zones.iter().any(|y| y.key == z.key))
}

/// Every boss of every installed-map kind, from the `maps` part of the answer.
pub fn bosses(maps: &[Option<RawMap>]) -> Vec<Boss> {
    let mut out: Vec<(Boss, bool)> = Vec::new();
    for m in maps.iter().flatten() {
        let Some(raw) = m.normalized_name.as_deref().filter(|n| !OWN_RAID.contains(n)) else { continue };
        let map = pack_id_for(raw);
        if !is_valid_id(map) {
            continue;
        }
        for b in m.bosses.iter().flatten().filter_map(|b| Some((boss(b, map, &m.spawns)?, late(b)))) {
            if let Some(prev) = out.iter_mut().find(|(x, _)| x.map == b.0.map && x.id == b.0.id && same_zones(x, &b.0))
            {
                merge(prev, b);
            } else if out.len() < MAX_BOSSES {
                out.push(b);
            }
        }
    }
    out.into_iter().map(|(b, _)| b).collect()
}

fn mob_id(v: &Value) -> Option<&str> {
    v.get("mob").and_then(Value::as_str)
}

/// json.tarkov.dev's `{mode}/maps` and `maps_{lang}` → the `maps` part of a GraphQL answer.
/// Mobs are referred to by id and named through the `mobs` table.
pub fn from_json(doc: &Value, loc: &Locale) -> Value {
    let root = doc.get("data").unwrap_or(doc);
    let mob = |id: Option<&str>| {
        let id = id.unwrap_or("");
        let key = root.get("mobs").and_then(|m| m.get(id)).and_then(|m| m.get("name")).and_then(Value::as_str);
        json!({ "id": id, "name": tr(loc, Some(key.unwrap_or(id))) })
    };
    let arr = |v: &Value, k: &str| v.get(k).and_then(Value::as_array).cloned().unwrap_or_default();
    let maps: Vec<Value> = entries(doc, "maps")
        .into_iter()
        .map(|m| {
            let bosses: Vec<Value> = arr(m, "bosses")
                .iter()
                .map(|b| {
                    let escorts: Vec<Value> = arr(b, "escorts").iter().map(|e| json!({ "boss": mob(mob_id(e)) })).collect();
                    let locations: Vec<Value> = arr(b, "spawnLocations")
                        .iter()
                        .map(|l| {
                            let name = l.get("name").and_then(Value::as_str);
                            json!({ "spawnKey": l.get("spawnKey"), "name": tr(loc, name), "chance": l.get("chance"), "positions": l.get("positions") })
                        })
                        .collect();
                    json!({
                        "boss": mob(mob_id(b)),
                        "spawnChance": b.get("spawnChance"),
                        "spawnTime": b.get("spawnTime"),
                        "spawnTrigger": b.get("spawnTrigger"),
                        "escorts": escorts,
                        "spawnLocations": locations,
                    })
                })
                .collect();
            json!({ "normalizedName": m.get("normalizedName"), "bosses": bosses, "spawns": m.get("spawns") })
        })
        .collect();
    Value::Array(maps)
}

#[cfg(test)]
#[path = "bosses_tests.rs"]
mod tests;
