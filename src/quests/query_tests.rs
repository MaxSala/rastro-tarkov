use super::*;
use crate::quests::SpotKind;

const PRAPOR: &str = r#"{"normalizedName":"prapor","name":"Prapor"}"#;

fn answer(tasks: &str) -> Vec<u8> {
    format!(r#"{{"data":{{"tasks":[{tasks}]}}}}"#).into_bytes()
}

fn checking() -> String {
    format!(
        r#"{{"id":"5979ed3886f77431307dc512","name":"Checking","normalizedName":"checking",
        "wikiLink":"https://escapefromtarkov.fandom.com/wiki/Checking","minPlayerLevel":3,
        "kappaRequired":true,"lightkeeperRequired":false,"factionName":"Any","trader":{PRAPOR},
        "map":{{"normalizedName":"customs"}},
        "taskRequirements":[{{"task":{{"id":"5936d90786f7742b1420ba5b"}},"status":["complete"]}}],
        "traderRequirements":[{{"trader":{{"name":"Prapor"}},"requirementType":"level","value":2}}],
        "objectives":[
          {{"id":"5979ed3886f77431307dc513","type":"findQuestItem","description":"Find the bronze pocket watch",
            "optional":false,"maps":[{{"normalizedName":"customs"}}],"count":1,
            "questItem":{{"id":"5937fd0086f7742bf33fc198","name":"Bronze pocket watch"}},
            "possibleLocations":[{{"map":{{"normalizedName":"customs"}},"positions":[{{"x":10.5,"y":1,"z":-20}},{{"x":10.6,"y":1.2,"z":-20.1}}]}}],
            "zones":[],"requiredKeys":[[{{"id":"5ad5d7d286f77450166e0a89","name":"Key to the truck"}}]]}},
          {{"id":"5979ee2c86f77461b7404323","type":"giveItem","description":"Hand over 3 Salewa",
            "optional":false,"maps":[],"count":3,"foundInRaid":true,
            "items":[{{"id":"544fb45d4bdc2dee738b4568","name":"Salewa"}}]}},
          {{"id":"5979ee2c86f77461b7404324","type":"visit","description":"Survey the night factory",
            "maps":[{{"normalizedName":"night-factory"}}],
            "zones":[{{"map":{{"normalizedName":"night-factory"}},"position":{{"x":1,"y":2,"z":3}}}}]}},
          {{"id":"not-an-id","type":"visit","description":"Broken"}},
          {{"id":"5979ee2c86f77461b7404325","type":"visit","description":""}}
        ]}}"#
    )
}

#[test]
fn reads_a_full_quest() {
    let db = parse(&answer(&checking()), "regular", "en", 7).expect("ok");
    assert_eq!((db.mode.as_str(), db.lang.as_str(), db.fetched_at), ("regular", "en", 7));
    assert_eq!(db.traders, vec![TraderRef { id: "prapor".into(), name: "Prapor".into(), image: None }]);
    let q = db.quests.first().expect("quest");
    assert_eq!((q.name.as_str(), q.slug.as_str(), q.trader.as_str()), ("Checking", "checking", "prapor"));
    assert_eq!((q.map.as_deref(), q.level, q.kappa, q.lightkeeper), (Some("customs"), 3, true, false));
    assert_eq!(q.faction, Faction::Any);
    assert_eq!(
        q.requires,
        vec![Requirement { quest: "5936d90786f7742b1420ba5b".into(), status: vec!["complete".into()] }]
    );
    assert_eq!(q.loyalty, vec![("Prapor".to_owned(), 2)]);
    assert_eq!(q.objectives.len(), 3, "objectives without id or text are skipped");
}

#[test]
fn objectives_keep_spots_items_and_keys() {
    let db = parse(&answer(&checking()), "regular", "en", 0).expect("ok");
    let q = db.quests.first().expect("quest");
    let watch = q.objectives.first().expect("watch");
    assert_eq!(watch.kind, "findQuestItem");
    assert_eq!(watch.positions.len(), 1, "two spots within a metre are one");
    assert_eq!(watch.positions.first().map(|s| s.map.as_str()), Some("customs"));
    assert_eq!(watch.items.first().map(|i| i.name.as_str()), Some("Bronze pocket watch"));
    assert_eq!(watch.keys.first().and_then(|g| g.first()).map(|k| k.name.as_str()), Some("Key to the truck"));
    let salewa = q.objectives.get(1).expect("salewa");
    assert_eq!((salewa.count, salewa.fir, salewa.maps.len()), (3, true, 0));
    let night = q.objectives.get(2).expect("night");
    assert_eq!(night.maps, vec!["factory".to_owned()], "night factory is the factory pack");
    assert_eq!(night.positions.first().map(|s| s.map.as_str()), Some("factory"));
}

#[test]
fn rejects_unsafe_links_and_bad_values() {
    let task = r#"{"id":"5936d90786f7742b1420ba5b","name":"Debut\u202e","normalizedName":"Debut!",
        "wikiLink":"https://evil.example/wiki","minPlayerLevel":1e9,"factionName":"USEC",
        "trader":{"normalizedName":"prapor","name":"Prapor"},"objectives":[
        {"id":"5936da9e86f7742d65037edf","type":"shoot","description":"Kill Scavs","count":-5,
         "zones":[{"map":{"normalizedName":"../etc"},"position":{"x":1,"y":1,"z":1}},
                  {"map":{"normalizedName":"customs"},"position":{"x":1e12,"y":1,"z":1}}]}]}"#;
    let db = parse(&answer(task), "pve", "es", 0).expect("ok");
    let q = db.quests.first().expect("quest");
    assert_eq!(q.name, "Debut", "control characters are removed");
    assert_eq!((q.slug.as_str(), q.wiki.as_deref(), q.level), ("", None, 100));
    assert_eq!(q.faction, Faction::Usec);
    let o = q.objectives.first().expect("objective");
    assert_eq!(o.count, 1, "a negative count means once");
    assert!(o.positions.is_empty(), "invalid maps and coordinates are dropped");
}

#[test]
fn needs_a_trader_an_id_and_at_least_one_quest() {
    assert!(parse(b"not json", "regular", "en", 0).is_err());
    assert!(parse(br#"{"data":null,"errors":[{"message":"boom"}]}"#, "regular", "en", 0).is_err());
    let no_trader = r#"{"id":"5936d90786f7742b1420ba5b","name":"Debut","objectives":[]}"#;
    assert!(parse(&answer(no_trader), "regular", "en", 0).is_err());
    let twice = format!("{},{}", checking(), checking());
    assert_eq!(parse(&answer(&twice), "regular", "en", 0).map(|d| d.quests.len()), Ok(1), "duplicates are skipped");
}

#[test]
fn query_names_every_field_the_parser_reads() {
    for field in ["possibleLocations", "requiredKeys", "foundInRaid", "taskRequirements", "traderRequirements"] {
        assert!(QUERY.contains(field), "{field}");
    }
    assert!(QUERY.contains("{lang}") && QUERY.contains("{mode}"));
}

#[test]
fn graphql_rewards_and_picture() {
    let task = r#"{"id":"5936d90786f7742b1420ba5b","name":"Debut","experience":1700,
        "taskImageLink":"https://assets.tarkov.dev/5936d90786f7742b1420ba5b.webp",
        "trader":{"id":"54cb50c76803fa8b248b4571","normalizedName":"prapor","name":"Prapor"},
        "objectives":[],
        "finishRewards":{"traderStanding":[{"trader":{"name":"Prapor"},"standing":0.02}],
          "items":[{"item":{"id":"5449016a4bdc2d6f028b456f","name":"Roubles"},"count":15000}],
          "offerUnlock":[],"skillLevelReward":[{"name":"Strength","level":-3}],"traderUnlock":[],
          "craftUnlock":[{"level":1,"rewardItems":[{"item":{"id":"bad"},"count":1}]}]}}"#;
    let db = parse(&answer(task), "regular", "en", 0).expect("ok");
    let q = db.quests.first().expect("quest");
    assert!(q.image);
    assert_eq!(db.traders.first().and_then(|t| t.image.as_deref()), Some("54cb50c76803fa8b248b4571"));
    assert_eq!(q.rewards.experience, 1700);
    assert_eq!(q.rewards.items.first().map(|i| i.count), Some(15000));
    assert!(q.rewards.skills.is_empty(), "negative skill levels are dropped");
    assert!(q.rewards.crafts.is_empty(), "invalid item ids are dropped");
}

#[test]
fn spots_say_how_precise_they_are() {
    let task = r#"{"id":"5936d90786f7742b1420ba5b","name":"Debut","trader":{"normalizedName":"prapor","name":"Prapor"},
        "objectives":[
        {"id":"5936da9e86f7742d65037ed1","type":"findQuestItem","description":"Find the package",
         "possibleLocations":[{"map":{"normalizedName":"customs"},"positions":[{"x":1,"y":1,"z":1},{"x":50,"y":1,"z":50}]}]},
        {"id":"5936da9e86f7742d65037ed2","type":"findQuestItem","description":"Find the watch",
         "possibleLocations":[{"map":{"normalizedName":"customs"},"positions":[{"x":7,"y":1,"z":7}]}]},
        {"id":"5936da9e86f7742d65037ed3","type":"shoot","description":"Kill in the area",
         "zones":[{"map":{"normalizedName":"customs"},"position":{"x":0,"y":10,"z":0},"top":20,"bottom":2,
           "outline":[{"x":-40,"z":-40},{"x":40,"z":-40},{"x":40,"z":40},{"x":-40,"z":40}]}]},
        {"id":"5936da9e86f7742d65037ed4","type":"mark","description":"Mark the tank",
         "zones":[{"map":{"normalizedName":"customs"},"position":{"x":3,"y":2,"z":3},
           "outline":[{"x":1,"z":1},{"x":5,"z":1},{"x":5,"z":5}]}]}]}"#;
    let db = parse(&answer(task), "regular", "en", 0).expect("ok");
    let o = &db.quests.first().expect("quest").objectives;
    let kinds = |i: usize| o.get(i).map(|o| o.positions.iter().map(|s| s.kind).collect::<Vec<_>>()).unwrap_or_default();
    assert_eq!(kinds(0), vec![SpotKind::Candidate, SpotKind::Candidate]);
    assert_eq!(kinds(1), vec![SpotKind::Exact], "a single possible spawn is the place");
    assert_eq!(kinds(2), vec![SpotKind::Area]);
    assert_eq!(o.get(2).and_then(|o| o.positions.first()).map(|s| s.y), Some(2.5), "height near the zone's floor");
    assert_eq!(kinds(3), vec![SpotKind::Exact]);
}
