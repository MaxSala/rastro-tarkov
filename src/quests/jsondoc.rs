//! Quests from json.tarkov.dev's static files (`{mode}/tasks`, `traders`, `maps` and their
//! `_{lang}` translations), the same files tarkov.dev's own site uses. They are served from a
//! CDN, so they keep working when the GraphQL API is down.
//!
//! In these files tasks refer to traders, maps and items by id and to texts by translation key.
//! [`to_graphql`] resolves all that into the shape of the GraphQL answer, so both sources go
//! through the same validation in [`super::query`].

use crate::market::raw::{parse_locale, Locale};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

/// The downloaded documents, still untrusted.
pub struct Docs {
    pub tasks: Vec<u8>,
    pub tasks_locale: Vec<u8>,
    pub traders: Vec<u8>,
    pub traders_locale: Vec<u8>,
    pub maps: Vec<u8>,
}

/// Item id → name in the player's language, from the market catalog or `items_{lang}`.
pub type ItemNames = HashMap<String, String>;

/// `data.<key>` (or `<key>`, or the root) as a list of entries, whether an object or an array.
fn entries<'a>(doc: &'a Value, key: &str) -> Vec<&'a Value> {
    let root = doc.get("data").unwrap_or(doc);
    match root.get(key).unwrap_or(root) {
        Value::Object(o) => o.values().collect(),
        Value::Array(a) => a.iter().collect(),
        _ => Vec::new(),
    }
}

fn s<'a>(v: &'a Value, k: &str) -> Option<&'a str> {
    v.get(k).and_then(Value::as_str)
}

/// A reference written as an id, or as an object with an `id`.
fn rid(v: &Value) -> Option<&str> {
    v.as_str().or_else(|| s(v, "id"))
}

fn list<'a>(v: &'a Value, k: &str) -> impl Iterator<Item = &'a Value> {
    v.get(k).and_then(Value::as_array).into_iter().flatten()
}

/// Translated text for a key; a key with no translation is only kept if it reads like a text.
fn tr(loc: &Locale, key: Option<&str>) -> String {
    let Some(key) = key else { return String::new() };
    if let Some(t) = loc.get(key) {
        return t.clone();
    }
    let looks_like_key =
        key.split(' ').next().is_some_and(|w| w.len() == 24 && w.bytes().all(|b| b.is_ascii_hexdigit()));
    if looks_like_key {
        String::new()
    } else {
        key.to_owned()
    }
}

struct Ctx<'a> {
    maps: HashMap<String, String>,
    traders: HashMap<String, (String, String)>,
    loc: &'a Locale,
    items: &'a ItemNames,
    quest_items: HashMap<String, String>,
}

impl Ctx<'_> {
    fn map(&self, v: &Value) -> Value {
        match rid(v).and_then(|id| self.maps.get(id)) {
            Some(n) => json!({ "normalizedName": n }),
            None => Value::Null,
        }
    }

    fn item(&self, v: &Value) -> Value {
        let Some(id) = rid(v) else { return Value::Null };
        let name = self.items.get(id).or_else(|| self.quest_items.get(id)).cloned().unwrap_or_default();
        json!({ "id": id, "name": name })
    }

    fn items(&self, v: &Value, k: &str) -> Value {
        Value::Array(list(v, k).map(|i| self.item(i)).collect())
    }

    fn zones(&self, o: &Value) -> Value {
        let z = list(o, "zones").map(|z| {
            json!({
                "map": self.map(z.get("map").unwrap_or(&Value::Null)),
                "position": z.get("position"),
                "outline": z.get("outline"),
                "top": z.get("top"),
                "bottom": z.get("bottom"),
            })
        });
        Value::Array(z.collect())
    }

    fn locations(&self, o: &Value) -> Value {
        let l = list(o, "possibleLocations")
            .map(|l| json!({ "map": self.map(l.get("map").unwrap_or(&Value::Null)), "positions": l.get("positions") }));
        Value::Array(l.collect())
    }

    fn objective(&self, o: &Value) -> Value {
        let keys: Vec<Value> = list(o, "requiredKeys")
            .map(|g| Value::Array(g.as_array().into_iter().flatten().map(|k| self.item(k)).collect()))
            .collect();
        let pick = |k: &str| o.get(k).map_or(Value::Null, |v| self.item(v));
        json!({
            "id": s(o, "id"),
            "type": s(o, "type"),
            "description": tr(self.loc, s(o, "description")),
            "optional": o.get("optional"),
            "count": o.get("count"),
            "foundInRaid": o.get("foundInRaid"),
            "maps": list(o, "maps").map(|m| self.map(m)).collect::<Vec<_>>(),
            "items": self.items(o, "items"),
            "item": pick("item"),
            "markerItem": pick("markerItem"),
            "questItem": pick("questItem"),
            "zones": self.zones(o),
            "possibleLocations": self.locations(o),
            "requiredKeys": keys,
        })
    }

    fn trader(&self, v: &Value) -> Value {
        match rid(v).and_then(|id| self.traders.get(id)) {
            Some((slug, name)) => json!({ "id": rid(v), "normalizedName": slug, "name": name }),
            None => Value::Null,
        }
    }

    /// `finishRewards`: ids → names, as the API's `TaskRewards`.
    fn rewards(&self, r: &Value) -> Value {
        let trader = |x: &Value| self.trader(x.get("trader").unwrap_or(&Value::Null));
        let counted =
            |x: &Value| json!({ "item": self.item(x.get("item").unwrap_or(&Value::Null)), "count": x.get("count") });
        json!({
            "traderStanding": list(r, "traderStanding").map(|x| json!({ "trader": trader(x), "standing": x.get("standing") })).collect::<Vec<_>>(),
            "items": list(r, "items").map(counted).collect::<Vec<_>>(),
            "offerUnlock": list(r, "offerUnlock")
                .map(|x| json!({ "trader": trader(x), "level": x.get("level"), "item": self.item(x.get("item").unwrap_or(&Value::Null)) }))
                .collect::<Vec<_>>(),
            "skillLevelReward": list(r, "skillLevelReward")
                .map(|x| json!({ "name": tr(self.loc, s(x, "skill").or_else(|| s(x, "name"))), "level": x.get("level") }))
                .collect::<Vec<_>>(),
            "traderUnlock": list(r, "traderUnlock").map(|x| self.trader(x)).collect::<Vec<_>>(),
            "craftUnlock": list(r, "craftUnlock")
                .map(|x| json!({ "level": x.get("level"), "rewardItems": [counted(x)] }))
                .collect::<Vec<_>>(),
        })
    }

    fn task(&self, t: &Value) -> Value {
        let reqs: Vec<Value> = list(t, "taskRequirements")
            .map(|r| json!({ "task": { "id": r.get("task").and_then(rid) }, "status": r.get("status") }))
            .collect();
        let trader_reqs: Vec<Value> = list(t, "traderRequirements")
            .map(|r| json!({ "trader": self.trader(r.get("trader").unwrap_or(&Value::Null)), "requirementType": r.get("requirementType"), "value": r.get("value") }))
            .collect();
        json!({
            "id": s(t, "id"),
            "name": tr(self.loc, s(t, "name")),
            "normalizedName": s(t, "normalizedName"),
            "wikiLink": s(t, "wikiLink"),
            "taskImageLink": s(t, "taskImageLink"),
            "minPlayerLevel": t.get("minPlayerLevel"),
            "experience": t.get("experience"),
            "kappaRequired": t.get("kappaRequired"),
            "lightkeeperRequired": t.get("lightkeeperRequired"),
            "factionName": s(t, "factionName"),
            "trader": self.trader(t.get("trader").unwrap_or(&Value::Null)),
            "map": self.map(t.get("map").unwrap_or(&Value::Null)),
            "taskRequirements": reqs,
            "traderRequirements": trader_reqs,
            "objectives": list(t, "objectives").map(|o| self.objective(o)).collect::<Vec<_>>(),
            "finishRewards": t.get("finishRewards").map(|r| self.rewards(r)),
        })
    }
}

fn parse(bytes: &[u8], what: &str) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("invalid {what} document: {e}"))
}

/// The static documents → `{ "data": { "tasks": [...] } }`, as the GraphQL API would answer.
pub fn to_graphql(docs: &Docs, items: &ItemNames) -> Result<Value, String> {
    let tasks = parse(&docs.tasks, "tasks")?;
    let maps_doc = parse(&docs.maps, "maps")?;
    let traders_doc = parse(&docs.traders, "traders")?;
    let loc = parse_locale(&docs.tasks_locale);
    let traders_loc = parse_locale(&docs.traders_locale);
    let maps = entries(&maps_doc, "maps")
        .into_iter()
        .filter_map(|m| Some((s(m, "id")?.to_owned(), s(m, "normalizedName")?.to_owned())))
        .collect();
    let traders = match traders_doc.get("data").unwrap_or(&traders_doc) {
        Value::Object(o) => traders_from(o, &traders_loc),
        _ => HashMap::new(),
    };
    let quest_items = entries(&tasks, "questItems")
        .into_iter()
        .filter_map(|q| Some((s(q, "id")?.to_owned(), tr(&loc, s(q, "name")))))
        .collect();
    let ctx = Ctx { maps, traders, loc: &loc, items, quest_items };
    let out: Vec<Value> = entries(&tasks, "tasks").into_iter().map(|t| ctx.task(t)).collect();
    if out.is_empty() {
        return Err("the tasks document has no tasks".into());
    }
    Ok(json!({ "data": { "tasks": out } }))
}

/// Trader id → (normalizedName, translated name).
fn traders_from(o: &Map<String, Value>, loc: &Locale) -> HashMap<String, (String, String)> {
    o.iter()
        .filter_map(|(id, t)| {
            let slug = s(t, "normalizedName")?.to_owned();
            let name = tr(loc, s(t, "name"));
            Some((id.clone(), (slug.clone(), if name.is_empty() { slug } else { name })))
        })
        .collect()
}

#[cfg(test)]
#[path = "jsondoc_tests.rs"]
mod tests;
