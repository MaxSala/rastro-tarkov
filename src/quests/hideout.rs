//! Hideout stations and what each level asks for: items, other stations, trader loyalty and
//! skills. They come with the quests, from the same per-mode download (`{mode}/hideout`), in the
//! GraphQL shape ([`from_json`] converts the static file), and go through the same checks.

use super::bosses::{list, num};
use super::jsondoc::{tr, ItemNames};
use super::query::id;
use crate::market::raw::Locale;
use crate::tarkovdev::api::clean;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

const MAX_STATIONS: usize = 64;
const MAX_LEVELS: usize = 10;
const MAX_NEEDS: usize = 24;
const MAX_COUNT: f64 = 100_000_000.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: String,
    pub name: String,
    pub slug: String,
    /// Level 1 first; a station with none listed can't be built.
    pub levels: Vec<StationLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StationLevel {
    pub level: u8,
    /// Construction time in seconds.
    pub seconds: u32,
    pub items: Vec<Need>,
    /// Other stations, by id, and the level they need.
    pub stations: Vec<(String, u8)>,
    /// Trader name and loyalty level.
    pub traders: Vec<(String, u8)>,
    /// Skill name and level.
    pub skills: Vec<(String, u8)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Need {
    pub id: String,
    pub name: String,
    pub count: u32,
    /// Has to be found in raid.
    pub fir: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawStation {
    id: Option<String>,
    name: Option<String>,
    normalized_name: Option<String>,
    #[serde(deserialize_with = "list")]
    levels: Vec<Option<RawLevel>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawLevel {
    #[serde(deserialize_with = "num")]
    level: Option<f64>,
    #[serde(deserialize_with = "num")]
    construction_time: Option<f64>,
    #[serde(deserialize_with = "list")]
    item_requirements: Vec<Option<RawNeed>>,
    #[serde(deserialize_with = "list")]
    station_level_requirements: Vec<Option<RawStationReq>>,
    #[serde(deserialize_with = "list")]
    trader_requirements: Vec<Option<RawTraderReq>>,
    #[serde(deserialize_with = "list")]
    skill_requirements: Vec<Option<RawSkill>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawRef {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawAttr {
    #[serde(rename = "type")]
    kind: Option<String>,
    name: Option<String>,
    value: Option<Value>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawNeed {
    item: Option<RawRef>,
    #[serde(deserialize_with = "num")]
    count: Option<f64>,
    #[serde(deserialize_with = "list")]
    attributes: Vec<Option<RawAttr>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawStationReq {
    station: Option<RawRef>,
    #[serde(deserialize_with = "num")]
    level: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawTraderReq {
    trader: Option<RawRef>,
    requirement_type: Option<String>,
    #[serde(deserialize_with = "num")]
    value: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawSkill {
    name: Option<String>,
    #[serde(deserialize_with = "num")]
    level: Option<f64>,
}

fn small(v: Option<f64>, max: f64) -> Option<u8> {
    v.filter(|l| l.is_finite() && (1.0..=max).contains(l)).map(|l| l as u8)
}

fn found_in_raid(a: &RawAttr) -> bool {
    let named = [&a.kind, &a.name].iter().any(|n| n.as_deref().is_some_and(|n| n.eq_ignore_ascii_case("foundInRaid")));
    let yes = match &a.value {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => s.eq_ignore_ascii_case("true"),
        _ => false,
    };
    named && yes
}

fn need(n: &RawNeed) -> Option<Need> {
    let item = n.item.as_ref()?;
    let count = n.count.filter(|c| c.is_finite() && (1.0..=MAX_COUNT).contains(c))? as u32;
    Some(Need {
        id: id(item.id.as_deref())?,
        name: clean(item.name.as_deref(), 120, ""),
        count,
        fir: n.attributes.iter().flatten().any(found_in_raid),
    })
}

fn level(l: &RawLevel) -> Option<StationLevel> {
    let named = |n: Option<&str>| Some(clean(n, 40, "")).filter(|s| !s.is_empty());
    Some(StationLevel {
        level: small(l.level, MAX_LEVELS as f64)?,
        seconds: l
            .construction_time
            .filter(|t| t.is_finite() && *t >= 0.0)
            .map_or(0, |t| t.min(f64::from(u32::MAX)) as u32),
        items: l.item_requirements.iter().flatten().filter_map(need).take(MAX_NEEDS).collect(),
        stations: l
            .station_level_requirements
            .iter()
            .flatten()
            .filter_map(|r| Some((id(r.station.as_ref()?.id.as_deref())?, small(r.level, MAX_LEVELS as f64)?)))
            .take(8)
            .collect(),
        traders: l
            .trader_requirements
            .iter()
            .flatten()
            .filter(|r| r.requirement_type.as_deref().is_none_or(|t| t == "loyaltyLevel" || t == "level"))
            .filter_map(|r| Some((named(r.trader.as_ref()?.name.as_deref())?, small(r.value, 10.0)?)))
            .take(8)
            .collect(),
        skills: l
            .skill_requirements
            .iter()
            .flatten()
            .filter_map(|r| Some((named(r.name.as_deref())?, small(r.level, 51.0)?)))
            .take(8)
            .collect(),
    })
}

/// Stations from the `hideoutStations` part of the answer.
pub fn stations(raw: &[Option<RawStation>]) -> Vec<Station> {
    let mut out: Vec<Station> = Vec::new();
    for s in raw.iter().flatten() {
        let (Some(sid), name) = (id(s.id.as_deref()), clean(s.name.as_deref(), 60, "")) else { continue };
        if name.is_empty() || out.iter().any(|x| x.id == sid) || out.len() >= MAX_STATIONS {
            continue;
        }
        let mut levels: Vec<StationLevel> = Vec::new();
        for l in s.levels.iter().flatten().filter_map(level) {
            if levels.iter().all(|x| x.level != l.level) {
                levels.push(l);
            }
        }
        levels.sort_by_key(|l| l.level);
        let slug = s.normalized_name.as_deref().filter(|n| crate::maps::manifest::is_valid_id(n)).unwrap_or("");
        out.push(Station { id: sid, name, slug: slug.to_owned(), levels });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// json.tarkov.dev's `{mode}/hideout` and `hideout_{lang}` → `hideoutStations` of a GraphQL
/// answer. Items, stations and traders come as bare ids, skills as `skill`, attributes as an
/// object (`{ "foundInRaid": "true" }`), as tarkov-data-manager writes them.
pub fn from_json(doc: &Value, loc: &Locale, items: &ItemNames, traders: &HashMap<String, String>) -> Value {
    let arr = |v: &Value, k: &str| v.get(k).and_then(Value::as_array).cloned().unwrap_or_default();
    let rid = |v: Option<&Value>| {
        v.and_then(|v| v.as_str().or_else(|| v.get("id").and_then(Value::as_str))).map(str::to_owned)
    };
    // Stations keyed by id (the id may only be the key) or, in other versions, a list.
    let root = doc.get("data").unwrap_or(doc);
    let root = root.get("hideout").unwrap_or(root);
    let all: Vec<(Option<&str>, &Value)> = match root {
        Value::Object(o) => o.iter().map(|(k, v)| (Some(k.as_str()), v)).collect(),
        Value::Array(a) => a.iter().map(|v| (None, v)).collect(),
        _ => Vec::new(),
    };
    let out: Vec<Value> = all
        .into_iter()
        .filter(|(_, s)| s.get("levels").is_some())
        .map(|(key, s)| {
            let levels: Vec<Value> = arr(s, "levels")
                .iter()
                .map(|l| {
                    let needs: Vec<Value> = arr(l, "itemRequirements")
                        .iter()
                        .map(|r| {
                            let item = rid(r.get("item"));
                            let name = item.as_ref().and_then(|i| items.get(i)).cloned().unwrap_or_default();
                            let attrs: Vec<Value> = match r.get("attributes") {
                                Some(Value::Object(o)) => o.iter().map(|(k, v)| json!({ "type": k, "name": k, "value": v })).collect(),
                                Some(Value::Array(a)) => a.clone(),
                                _ => Vec::new(),
                            };
                            json!({ "item": { "id": item, "name": name }, "count": r.get("count"), "attributes": attrs })
                        })
                        .collect();
                    let stations: Vec<Value> = arr(l, "stationLevelRequirements")
                        .iter()
                        .map(|r| json!({ "station": { "id": rid(r.get("station")) }, "level": r.get("level") }))
                        .collect();
                    let trader_reqs: Vec<Value> = arr(l, "traderRequirements")
                        .iter()
                        .map(|r| {
                            let name = rid(r.get("trader")).and_then(|t| traders.get(&t).cloned());
                            json!({ "trader": { "name": name }, "requirementType": r.get("requirementType"), "value": r.get("value").or(r.get("level")) })
                        })
                        .collect();
                    let skills: Vec<Value> = arr(l, "skillRequirements")
                        .iter()
                        .map(|r| json!({ "name": tr(loc, r.get("skill").or(r.get("name")).and_then(Value::as_str)), "level": r.get("level") }))
                        .collect();
                    json!({
                        "level": l.get("level"),
                        "constructionTime": l.get("constructionTime"),
                        "itemRequirements": needs,
                        "stationLevelRequirements": stations,
                        "traderRequirements": trader_reqs,
                        "skillRequirements": skills,
                    })
                })
                .collect();
            let name = tr(loc, s.get("name").and_then(Value::as_str));
            let sid = s.get("id").and_then(Value::as_str).or(key);
            json!({ "id": sid, "name": name, "normalizedName": s.get("normalizedName"), "levels": levels })
        })
        .collect();
    Value::Array(out)
}

#[cfg(test)]
#[path = "hideout_tests.rs"]
mod tests;
