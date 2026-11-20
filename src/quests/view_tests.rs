use super::super::progress::Record;
use super::super::{Objective, Quest, Spot};
use super::*;

const Q: &str = "5979ed3886f77431307dc512";
const O1: &str = "5979ee2c86f77461b7404323";
const O2: &str = "5979ee2c86f77461b7404324";

fn spot(map: &str, x: f64) -> Spot {
    Spot { map: map.into(), x, y: 0.0, z: 0.0, outline: Vec::new(), kind: Default::default() }
}

fn objective(id: &str, count: u32, positions: Vec<Spot>) -> Objective {
    Objective {
        id: id.into(),
        kind: "visit".into(),
        text: "Go".into(),
        optional: false,
        count,
        fir: false,
        maps: vec![],
        items: vec![],
        keys: vec![],
        positions,
    }
}

fn db() -> QuestDb {
    QuestDb {
        mode: "regular".into(),
        lang: "en".into(),
        fetched_at: 0,
        traders: vec![],
        quests: vec![Quest {
            id: Q.into(),
            name: "Checking".into(),
            slug: "checking".into(),
            trader: "prapor".into(),
            map: Some("customs".into()),
            level: 1,
            kappa: false,
            lightkeeper: false,
            faction: Faction::Any,
            wiki: None,
            requires: vec![],
            loyalty: vec![],
            objectives: vec![
                objective(O1, 1, vec![spot("customs", 1.0), spot("woods", 2.0), spot("customs", 3.0)]),
                objective(O2, 3, vec![spot("customs", 4.0)]),
            ],
            rewards: Default::default(),
            image: false,
        }],
    }
}

fn active() -> ModeProgress {
    let mut p = ModeProgress::default();
    p.quests.insert(Q.into(), Record { status: Status::Active, at: 0, source: Default::default() });
    p
}

#[test]
fn only_open_objectives_of_active_quests_on_this_map() {
    let db = db();
    let none = ModeProgress::default();
    assert_eq!(open_spots(&db, &none, "customs", None).count(), 0, "not started: nothing on the map");
    let mut p = active();
    let ids: Vec<(&str, usize)> = open_spots(&db, &p, "customs", None).map(|(_, o, n, _, _)| (o, n)).collect();
    assert_eq!(ids, vec![(O1, 0), (O1, 2), (O2, 0)], "spot numbers stay those of the objective");
    p.objectives.insert(O1.into(), 1);
    p.objectives.insert(O2.into(), 2);
    let ids: Vec<&str> = open_spots(&db, &p, "customs", None).map(|(_, o, _, _, _)| o).collect();
    assert_eq!(ids, vec![O2], "done objectives disappear, partial ones stay");
    p.quests.insert(Q.into(), Record { status: Status::Completed, at: 0, source: Default::default() });
    assert_eq!(open_spots(&db, &p, "customs", None).count(), 0);
}

#[test]
fn a_previewed_quest_shows_all_its_spots() {
    let db = db();
    let mut p = ModeProgress::default();
    p.objectives.insert(O1.into(), 1);
    let spots: Vec<(&str, bool)> = open_spots(&db, &p, "customs", Some(Q)).map(|(_, o, _, _, f)| (o, f)).collect();
    assert_eq!(spots, vec![(O1, true), (O1, true), (O2, true)], "done objectives too, all flagged");
    assert_eq!(open_spots(&db, &p, "customs", Some(O1)).count(), 0, "an unknown id previews nothing");
    let mut p = active();
    p.objectives.insert(O1.into(), 1);
    let spots: Vec<(&str, bool)> = open_spots(&db, &p, "customs", Some(Q)).map(|(_, o, _, _, f)| (o, f)).collect();
    assert_eq!(spots, vec![(O2, false)], "an active quest stays as it is while previewed");
}

#[test]
fn guide_targets_are_parsed_strictly() {
    assert_eq!(parse_target(&format!("qo-{O1}-2")), Some((O1, 2)));
    assert_eq!(parse_target("qo-nothex-1"), None);
    assert_eq!(parse_target(&format!("qo-{O1}-99")), None);
    assert_eq!(parse_target(&format!("pt-{O1}-1")), None);
    assert_eq!(parse_target(&format!("qo-{O1}")), None);
}
