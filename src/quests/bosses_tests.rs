use super::*;
use serde_json::json;

fn locale() -> Locale {
    [
        ("bossBully", "Reshala"),
        ("followerBully", "Reshala Guard"),
        ("ZoneDormitory", "Dorms"),
        ("ZoneGasStation", "Gas station"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v.to_owned()))
    .collect()
}

/// json.tarkov.dev's maps file, cut down from the real one.
fn maps_doc() -> Value {
    let reshala = json!({
        "mob": "bossBully", "spawnChance": 0.45, "spawnTime": -1,
        "escorts": [{ "mob": "followerBully" }, { "mob": "followerBully" }],
        "spawnLocations": [
            { "name": "ZoneDormitory", "spawnKey": "ZoneDormitory", "chance": 0.33,
              "positions": [{ "x": 181.1, "y": -0.1, "z": 178.0 }, { "x": 174.6, "y": 2.8, "z": 152.7 }] },
            { "name": "ZoneGasStation", "spawnKey": "ZoneGasStation", "chance": 0.33,
              "positions": [{ "x": 420.4, "y": 1.1, "z": 22.5 }] }
        ]
    });
    let raiders = |chance: f64, time: f64| {
        json!({ "mob": "pmcBot", "spawnChance": chance, "spawnTime": time,
                "spawnLocations": [{ "name": "BotZoneMain", "spawnKey": "BotZoneMain", "chance": 1,
                                     "positions": [{ "x": 1.0, "y": 0.0, "z": 2.0 }] }] })
    };
    json!({ "data": {
        "maps": {
            "56f40101d2720b2a4d8b45d6": { "normalizedName": "customs", "bosses": [
                reshala,
                { "mob": "bossPartisan", "spawnChance": 0.1, "spawnTime": 900, "spawnLocations": [] }
            ], "spawns": [] },
            "5b0fc42d86f7744a585f9105": { "normalizedName": "the-lab", "bosses": [
                raiders(0.45, 450.0), raiders(0.6, 300.0), raiders(0.35, 9999.0)
            ] },
            "59fc81d786f774390775787e": { "normalizedName": "night-factory", "bosses": [
                { "mob": "bossTagilla", "spawnChance": 0.45, "spawnLocations": [] }
            ] }
        },
        "mobs": {
            "bossBully": { "id": "bossBully", "name": "bossBully", "normalizedName": "reshala" },
            "followerBully": { "id": "followerBully", "name": "followerBully" },
            "pmcBot": { "id": "pmcBot", "name": "pmcBot" }
        }
    }})
}

fn parse(v: Value) -> Vec<Boss> {
    let maps: Vec<Option<RawMap>> = serde_json::from_value(v).expect("maps");
    bosses(&maps)
}

#[test]
fn reads_the_static_file_with_exact_points() {
    let all = parse(from_json(&maps_doc(), &locale()));
    let reshala = all.iter().find(|b| b.id == "bossBully").expect("Reshala");
    assert_eq!(reshala.name, "Reshala");
    assert_eq!(reshala.map, "customs");
    assert!((reshala.chance - 0.45).abs() < 1e-9);
    assert_eq!(reshala.time, None, "-1 means at the start");
    assert_eq!(reshala.escorts, vec!["Reshala Guard".to_owned()], "escorts once, by name");
    assert_eq!(reshala.zones.len(), 2);
    assert_eq!(reshala.zones[0].name, "Dorms");
    assert_eq!(reshala.zones[0].points.len(), 2);
    assert_eq!(reshala.zones[0].points[1], [174.6, 2.8, 152.7]);
    // Known by name only: listed, with no zones to draw.
    let partisan = all.iter().find(|b| b.id == "bossPartisan").expect("Partisan");
    assert_eq!(partisan.time, Some(900));
    assert!(partisan.zones.is_empty());
    assert_eq!(partisan.name, "bossPartisan", "an untranslated name falls back to the id");
}

#[test]
fn waves_of_the_same_boss_become_one_entry() {
    let all = parse(from_json(&maps_doc(), &locale()));
    let raiders: Vec<&Boss> = all.iter().filter(|b| b.map == "the-lab").collect();
    assert_eq!(raiders.len(), 1);
    assert!((raiders[0].chance - 0.6).abs() < 1e-9, "as likely as the likeliest wave");
    assert_eq!(raiders[0].time, Some(300), "as early as the earliest; 9999 is no time");
}

#[test]
fn a_variant_that_is_its_own_raid_is_not_merged() {
    let all = parse(from_json(&maps_doc(), &locale()));
    assert!(all.iter().all(|b| b.id != "bossTagilla"), "night Factory's bosses are not day Factory's");
}

#[test]
fn graphql_zones_take_their_points_from_the_spawns() {
    let answer = json!([{ "normalizedName": "customs",
        "bosses": [{ "boss": { "id": "bossBully", "name": "Reshala" }, "spawnChance": 0.45, "spawnTime": -1,
                     "spawnTrigger": "", "escorts": [{ "boss": { "name": "Reshala Guard" } }],
                     "spawnLocations": [{ "spawnKey": "ZoneDormitory", "name": "Dorms", "chance": 0.33 }] }],
        "spawns": [
            { "zoneName": "ZoneDormitory", "position": { "x": 1, "y": 2, "z": 3 }, "categories": ["bot", "boss"] },
            { "zoneName": "ZoneDormitory", "position": { "x": 9, "y": 9, "z": 9 }, "categories": ["player"] },
            { "zoneName": "ZoneScavBase", "position": { "x": 5, "y": 5, "z": 5 }, "categories": ["boss"] }
        ] }]);
    let all = parse(answer);
    assert_eq!(all.len(), 1);
    assert!(!all[0].triggered, "an empty trigger is no trigger");
    assert_eq!(all[0].zones[0].points, vec![[1.0, 2.0, 3.0]], "only boss spawns of that zone");
}

#[test]
fn bad_parts_are_skipped_not_fatal() {
    let answer = json!([
        null,
        { "normalizedName": "customs", "bosses": null, "spawns": null },
        { "normalizedName": "../etc", "bosses": [{ "boss": { "id": "bossBully", "name": "x" }, "spawnChance": 1 }] },
        { "normalizedName": "woods", "bosses": [
            null,
            { "boss": { "id": "bad id!", "name": "x" }, "spawnChance": 0.5 },
            { "boss": { "id": "bossKojaniy", "name": "Shturman" }, "spawnChance": 0 },
            { "boss": { "id": "bossKojaniy", "name": "Shturman\u{202e}" }, "spawnChance": 7, "spawnTrigger": "Switch",
              "spawnLocations": [
                  { "spawnKey": "ZoneWoodCutter", "name": "Sawmill", "chance": 1, "positions": [{ "x": "a" }, { "x": 1, "z": 2 }, { "x": 1e9, "z": 0 }] },
                  { "spawnKey": "no spaces allowed", "chance": 1 },
                  { "spawnKey": "ZoneNoChance", "chance": 0 }
              ] }
        ] }
    ]);
    let all = parse(answer);
    assert_eq!(all.len(), 1);
    let b = &all[0];
    assert_eq!(b.name, "Shturman", "control characters are cleaned");
    assert!((b.chance - 1.0).abs() < 1e-9, "chances are capped at 100 %");
    assert!(b.triggered);
    assert_eq!(b.zones.len(), 1);
    assert_eq!(b.zones[0].points, vec![[1.0, 0.0, 2.0]], "only finite, sensible points");
}
