use super::*;
use crate::quests::query::parse_value;

const PRAPOR: &str = "54cb50c76803fa8b248b4571";
const CUSTOMS: &str = "56f40101d2720b2a4d8b45d6";
const TASK: &str = "5936d90786f7742b1420ba5b";
const PREV: &str = "657315df034d76585f032e01";
const OBJ: &str = "5967733e86f774602332fc84";
const SALEWA: &str = "544fb45d4bdc2dee738b4568";
const KEY: &str = "5780cf7f2459777de4559322";
const QITEM: &str = "590c62a386f77412b0130255";

fn docs() -> Docs {
    let tasks = serde_json::json!({ "data": {
        "tasks": { TASK: {
            "id": TASK, "name": format!("{TASK} name"), "normalizedName": "debut", "trader": PRAPOR,
            "map": CUSTOMS, "minPlayerLevel": 2, "kappaRequired": true, "lightkeeperRequired": false,
            "factionName": "Any", "wikiLink": "https://escapefromtarkov.fandom.com/wiki/Debut",
            "taskImageLink": format!("https://assets.tarkov.dev/{TASK}.webp"), "experience": 1700,
            "finishRewards": {
                "traderStanding": [{ "trader": PRAPOR, "standing": 0.02 }],
                "items": [{ "item": SALEWA, "count": 2 }, { "item": SALEWA, "count": 1 }],
                "offerUnlock": [{ "id": "x", "trader": PRAPOR, "level": 1, "item": KEY, "count": 1 }],
                "skillLevelReward": [{ "skill": "Endurance", "level": 1 }],
                "traderUnlock": [PRAPOR],
                "craftUnlock": [{ "station": "5d484fc0654e76006657e0ab", "level": 1, "item": SALEWA, "count": 1 }],
                "achievement": [], "customization": []
            },
            "taskRequirements": [{ "task": PREV, "status": ["complete"] }],
            "traderRequirements": [{ "trader": PRAPOR, "requirementType": "level", "compareMethod": ">=", "value": 2 }],
            "objectives": [
                { "id": OBJ, "type": "giveItem", "description": OBJ, "optional": false, "count": 3,
                  "foundInRaid": true, "items": [SALEWA], "maps": [] },
                { "id": "5967733e86f774602332fc85", "type": "findQuestItem", "description": "5967733e86f774602332fc85",
                  "optional": false, "count": 1, "questItem": QITEM, "maps": [CUSTOMS],
                  "possibleLocations": [{ "map": CUSTOMS, "positions": [{ "x": -100.5, "y": 2.0, "z": 50.0 }] }],
                  "requiredKeys": [[KEY]] },
                { "id": "5967733e86f774602332fc86", "type": "visit", "description": "5967733e86f774602332fc86",
                  "optional": true, "maps": [CUSTOMS],
                  "zones": [{ "id": "zone_1", "map": CUSTOMS, "position": { "x": 10.0, "y": 1.0, "z": 20.0 },
                    "outline": [{ "x": 5.0, "y": 1.0, "z": 15.0 }, { "x": 15.0, "y": 1.0, "z": 15.0 },
                                { "x": 15.0, "y": 1.0, "z": 25.0 }, { "x": 5.0, "y": 1.0, "z": 25.0 }] }] }
            ]
        }},
        "questItems": { QITEM: { "id": QITEM, "name": format!("{QITEM} Name") } }
    }});
    let loc = serde_json::json!({ "data": {
        format!("{TASK} name"): "Debut",
        OBJ: "Hand over 3 Salewa",
        "5967733e86f774602332fc85": "Find the package",
        "5967733e86f774602332fc86": "Visit the \u{202e}dorms",
        format!("{QITEM} Name"): "Package"
    }});
    let traders =
        serde_json::json!({ "data": { PRAPOR: { "name": format!("{PRAPOR} Nickname"), "normalizedName": "prapor" } } });
    let traders_loc = serde_json::json!({ "data": { format!("{PRAPOR} Nickname"): "Prapor" } });
    let maps = serde_json::json!({ "data": { "maps": { CUSTOMS: { "id": CUSTOMS, "normalizedName": "customs" } } } });
    let b = |v: serde_json::Value| serde_json::to_vec(&v).unwrap_or_default();
    Docs {
        tasks: b(tasks),
        tasks_locale: b(loc),
        traders: b(traders),
        traders_locale: b(traders_loc),
        maps: b(maps),
        maps_locale: vec![],
        hideout: vec![],
        hideout_locale: vec![],
    }
}

#[test]
fn static_files_become_the_same_quests_as_graphql() {
    let names: ItemNames =
        [(SALEWA.to_owned(), "Salewa first aid kit".to_owned()), (KEY.to_owned(), "Dorm room 114 key".to_owned())]
            .into();
    let v = to_graphql(&docs(), &names).map_err(|e| e.to_string());
    let db = v.and_then(|v| parse_value(v, "regular", "en", 1)).expect("ok");
    assert_eq!(db.traders.len(), 1);
    assert_eq!(db.traders.first().map(|t| t.name.as_str()), Some("Prapor"));
    let q = db.quests.first().expect("ok");
    assert_eq!((q.name.as_str(), q.slug.as_str(), q.trader.as_str()), ("Debut", "debut", "prapor"));
    assert_eq!(q.map.as_deref(), Some("customs"));
    assert_eq!((q.level, q.kappa), (2, true));
    assert_eq!(q.requires.first().map(|r| r.quest.as_str()), Some(PREV));
    assert_eq!(q.loyalty, vec![("Prapor".to_owned(), 2)]);
    let [give, find, visit] = q.objectives.as_slice() else { panic!("three objectives expected") };
    assert_eq!((give.count, give.fir), (3, true));
    assert_eq!(give.items.first().map(|i| i.name.as_str()), Some("Salewa first aid kit"));
    assert_eq!(find.items.first().map(|i| i.name.as_str()), Some("Package"));
    assert_eq!(find.keys.first().and_then(|g| g.first()).map(|k| k.name.as_str()), Some("Dorm room 114 key"));
    assert_eq!(find.positions.first().map(|p| (p.map.as_str(), p.x)), Some(("customs", -100.5)));
    assert!(visit.optional);
    assert!(!visit.text.contains('\u{202e}'));
    assert_eq!(visit.positions.first().map(|p| p.z), Some(20.0));
    assert_eq!(visit.positions.first().map(|p| p.outline.len()), Some(4), "zones keep their outline");
}

#[test]
fn rewards_trader_portrait_and_quest_picture() {
    let names: ItemNames =
        [(SALEWA.to_owned(), "Salewa".to_owned()), (KEY.to_owned(), "Dorm room 114 key".to_owned())].into();
    let db = to_graphql(&docs(), &names).and_then(|v| parse_value(v, "regular", "en", 1)).expect("ok");
    assert_eq!(db.traders.first().and_then(|t| t.image.as_deref()), Some(PRAPOR));
    let q = db.quests.first().expect("quest");
    assert!(q.image, "the picture is tarkov.dev's own for this quest");
    let r = &q.rewards;
    assert_eq!(r.experience, 1700);
    assert_eq!(r.standing, vec![("Prapor".to_owned(), 0.02)]);
    assert_eq!(r.items.iter().map(|i| (i.name.as_str(), i.count)).collect::<Vec<_>>(), vec![("Salewa", 3)]);
    assert_eq!(
        r.offers.first().map(|o| (o.name.as_str(), o.trader.as_str(), o.level)),
        Some(("Dorm room 114 key", "Prapor", 1))
    );
    assert_eq!(r.skills, vec![("Endurance".to_owned(), 1.0)]);
    assert_eq!(r.traders, vec!["Prapor".to_owned()]);
    assert_eq!(r.crafts.first().map(|c| c.name.as_str()), Some("Salewa"));
}

#[test]
fn placeholder_or_foreign_pictures_are_ignored() {
    for link in ["https://assets.tarkov.dev/unknown-task.webp", "https://evil.example/5936d90786f7742b1420ba5b.webp"] {
        let mut d = docs();
        let mut v: serde_json::Value = serde_json::from_slice(&d.tasks).expect("json");
        v["data"]["tasks"][TASK]["taskImageLink"] = link.into();
        d.tasks = serde_json::to_vec(&v).expect("json");
        let db = to_graphql(&d, &ItemNames::new()).and_then(|v| parse_value(v, "regular", "en", 1)).expect("ok");
        assert!(!db.quests.first().expect("quest").image, "{link}");
    }
}

#[test]
fn unknown_ids_and_keys_never_show_up_as_text() {
    let db = to_graphql(&docs(), &ItemNames::new()).and_then(|v| parse_value(v, "regular", "en", 1)).expect("ok");
    let give = db.quests.first().and_then(|q| q.objectives.first()).expect("ok");
    // No name for the item: it keeps its id (for the Market) but no made-up name.
    assert_eq!(give.items.first().map(|i| (i.id.as_str(), i.name.as_str())), Some((SALEWA, "")));
}

#[test]
fn broken_documents_fail_cleanly() {
    let mut d = docs();
    d.tasks = b"{\"data\":{\"tasks\":{}}}".to_vec();
    assert!(to_graphql(&d, &ItemNames::new()).is_err());
    let mut d = docs();
    d.maps = b"not json".to_vec();
    assert!(to_graphql(&d, &ItemNames::new()).is_err());
}
