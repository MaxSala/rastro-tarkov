use super::*;
use serde_json::json;

const GEN: &str = "5d3b396e33c48f02b81cd9f3";
const VENTS: &str = "5d388e97081959000a123acf";
const BOLTS: &str = "57347c5b245977448d35f6e1";
const WIRES: &str = "5c06779c86f77426e00dd782";
const PRAPOR: &str = "54cb50c76803fa8b248b4571";

fn parse(v: Value) -> Vec<Station> {
    let raw: Vec<Option<RawStation>> = serde_json::from_value(v).expect("stations");
    stations(&raw)
}

/// json.tarkov.dev's hideout file, as tarkov.dev's own site reads it: stations keyed by id,
/// bare ids for items, stations and traders, texts as translation keys.
fn hideout_doc() -> Value {
    json!({ "data": {
        GEN: { "name": "generator Name", "normalizedName": "generator", "levels": [
            { "level": 2, "constructionTime": 7200,
              "itemRequirements": [{ "item": WIRES, "count": 2, "attributes": { "foundInRaid": "true" } }],
              "stationLevelRequirements": [{ "station": VENTS, "level": 1 }],
              "traderRequirements": [{ "trader": PRAPOR, "requirementType": "loyaltyLevel", "value": 2 }],
              "skillRequirements": [{ "skill": "HideoutManagement", "level": 3 }] },
            { "level": 1, "constructionTime": 0,
              "itemRequirements": [{ "item": BOLTS, "count": 5, "attributes": { "foundInRaid": "false" } }],
              "stationLevelRequirements": [], "traderRequirements": [], "skillRequirements": [] }
        ] },
        VENTS: { "id": VENTS, "name": "vents Name", "normalizedName": "vents", "levels": [] }
    }, "translations": [] })
}

#[test]
fn reads_the_static_file() {
    let loc: Locale =
        [("generator Name", "Generator"), ("vents Name", "Vents"), ("HideoutManagement", "Hideout management")]
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
    let items: ItemNames = [(BOLTS.to_owned(), "Bolts".to_owned()), (WIRES.to_owned(), "Wires".to_owned())].into();
    let traders: HashMap<String, String> = [(PRAPOR.to_owned(), "Prapor".to_owned())].into();
    let all = parse(from_json(&hideout_doc(), &loc, &items, &traders));
    assert_eq!(all.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["Generator", "Vents"], "by name");
    let generator = &all[0];
    assert_eq!(generator.id, GEN, "the id may only be the key");
    assert_eq!(generator.slug, "generator");
    assert_eq!(generator.levels.iter().map(|l| l.level).collect::<Vec<_>>(), [1, 2], "level 1 first");
    let two = &generator.levels[1];
    assert_eq!(two.seconds, 7200);
    assert_eq!(two.items, vec![Need { id: WIRES.into(), name: "Wires".into(), count: 2, fir: true }]);
    assert_eq!(two.stations, vec![(VENTS.to_owned(), 1)]);
    assert_eq!(two.traders, vec![("Prapor".to_owned(), 2)]);
    assert_eq!(two.skills, vec![("Hideout management".to_owned(), 3)]);
    assert!(!generator.levels[0].items[0].fir);
}

#[test]
fn reads_the_graphql_answer() {
    let all = parse(json!([{ "id": GEN, "name": "Generator", "normalizedName": "generator", "levels": [
        { "level": 1, "constructionTime": 60,
          "itemRequirements": [{ "item": { "id": BOLTS, "name": "Bolts" }, "count": 1,
                                 "attributes": [{ "type": "foundInRaid", "name": "foundInRaid", "value": "true" }] }],
          "stationLevelRequirements": [{ "station": { "id": VENTS }, "level": 1 }],
          "traderRequirements": [{ "trader": { "name": "Prapor" }, "requirementType": "level", "value": 1 }],
          "skillRequirements": [] }
    ] }]));
    assert_eq!(all.len(), 1);
    let l = &all[0].levels[0];
    assert!(l.items[0].fir);
    assert_eq!(l.traders, vec![("Prapor".to_owned(), 1)]);
}

#[test]
fn bad_parts_are_skipped_not_fatal() {
    let all = parse(json!([
        null,
        { "id": "not-an-id", "name": "Bad", "levels": [] },
        { "id": GEN, "name": "", "levels": [] },
        { "id": VENTS, "name": "Vents", "normalizedName": "../x", "levels": [
            null,
            { "level": 0 },
            { "level": 99 },
            { "level": 1, "constructionTime": -5, "itemRequirements": [
                { "item": { "id": BOLTS, "name": "Bolts" }, "count": 0 },
                { "item": { "id": "zz", "name": "?" }, "count": 1 },
                { "item": { "id": WIRES, "name": "Wires" }, "count": 3 }
              ], "traderRequirements": [{ "trader": { "name": "Prapor" }, "requirementType": "reputation", "value": 1 }] },
            { "level": 1, "constructionTime": 10 }
        ] },
        { "id": VENTS, "name": "Again", "levels": [] }
    ]));
    assert_eq!(all.len(), 1, "invalid ids, empty names and repeats are dropped");
    let s = &all[0];
    assert_eq!(s.slug, "", "an unsafe slug is dropped");
    assert_eq!(s.levels.len(), 1, "levels out of range and repeated levels are dropped");
    assert_eq!(s.levels[0].seconds, 0);
    assert_eq!(s.levels[0].items.len(), 1);
    assert_eq!(s.levels[0].items[0].count, 3);
    assert!(s.levels[0].traders.is_empty(), "only loyalty levels count");
}
