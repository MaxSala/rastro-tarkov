use super::*;

const A: &str = "5936d90786f7742b1420ba5b";
const B: &str = "5979ed3886f77431307dc512";
const OBJ: &str = "5979ee2c86f77461b7404323";

fn tmp() -> PathBuf {
    std::env::temp_dir().join(format!("rastro-q-{}.json", crate::maps::import::random_hex(6).expect("rng")))
}

fn store() -> (PathBuf, ProgressStore) {
    let path = tmp();
    let s = ProgressStore::load(&path);
    (path, s)
}

fn set(quest: &str, status: Option<Status>) -> Event {
    Event::SetStatus { quest: quest.into(), status, source: Source::Manual }
}

#[test]
fn starts_empty_at_level_one() {
    let (_d, s) = store();
    assert_eq!(s.get(GameMode::Regular), &ModeProgress::default());
    assert_eq!(s.get(GameMode::Regular).level, 1);
}

#[test]
fn modes_are_kept_apart_and_saved() {
    let (path, mut s) = store();
    assert_eq!(s.apply(GameMode::Pve, set(A, Some(Status::Active))), Ok(true));
    assert_eq!(s.apply(GameMode::Pve, Event::SetObjective { objective: OBJ.into(), count: 2 }), Ok(true));
    assert_eq!(s.apply(GameMode::Pve, Event::SetLevel(14)), Ok(true));
    assert_eq!(s.status(GameMode::Pve, A), Some(Status::Active));
    assert_eq!(s.status(GameMode::Regular, A), None);
    s.save().expect("save");
    let again = ProgressStore::load(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(again.get(GameMode::Pve), s.get(GameMode::Pve));
    assert_eq!(again.get(GameMode::Pve).objectives.get(OBJ), Some(&2));
}

#[test]
fn revision_moves_only_on_real_changes() {
    let (_d, mut s) = store();
    let r0 = s.revision;
    assert_eq!(s.apply(GameMode::Regular, set(A, Some(Status::Completed))), Ok(true));
    assert_eq!(s.apply(GameMode::Regular, set(A, Some(Status::Completed))), Ok(false));
    assert_eq!(s.revision, r0 + 1);
    assert_eq!(s.apply(GameMode::Regular, set(A, None)), Ok(true));
    assert_eq!(s.status(GameMode::Regular, A), None);
}

#[test]
fn complete_all_keeps_existing_records() {
    let (_d, mut s) = store();
    s.apply(GameMode::Regular, set(A, Some(Status::Completed))).expect("ok");
    let at = s.get(GameMode::Regular).quests.get(A).map(|r| r.at);
    s.apply(GameMode::Regular, Event::CompleteAll(vec![A.into(), B.into()])).expect("ok");
    assert_eq!(s.status(GameMode::Regular, B), Some(Status::Completed));
    assert_eq!(s.get(GameMode::Regular).quests.get(A).map(|r| r.at), at);
}

#[test]
fn refuses_bad_ids_and_counts() {
    let (_d, mut s) = store();
    assert!(s.apply(GameMode::Regular, set("../x", Some(Status::Active))).is_err());
    assert!(s.apply(GameMode::Regular, set(&A.to_uppercase(), Some(Status::Active))).is_err());
    assert!(s.apply(GameMode::Regular, Event::SetObjective { objective: OBJ.into(), count: 1_000_000 }).is_err());
    assert!(s.apply(GameMode::Regular, Event::CompleteAll(vec![A.into(), "nope".into()])).is_err());
    assert_eq!(s.apply(GameMode::Regular, Event::SetLevel(250)), Ok(true));
    assert_eq!(s.get(GameMode::Regular).level, MAX_LEVEL);
}

#[test]
fn objective_zero_clears_and_reset_empties() {
    let (_d, mut s) = store();
    s.apply(GameMode::Regular, Event::SetObjective { objective: OBJ.into(), count: 3 }).expect("ok");
    s.apply(GameMode::Regular, Event::SetObjective { objective: OBJ.into(), count: 0 }).expect("ok");
    assert!(s.get(GameMode::Regular).objectives.is_empty());
    s.apply(GameMode::Regular, set(A, Some(Status::Active))).expect("ok");
    assert_eq!(s.apply(GameMode::Regular, Event::Reset), Ok(true));
    assert_eq!(s.get(GameMode::Regular), &ModeProgress::default());
}

#[test]
fn a_broken_or_hostile_file_is_cleaned_up() {
    let path = tmp();
    std::fs::write(&path, b"{ not json").expect("write");
    assert_eq!(ProgressStore::load(&path).get(GameMode::Regular), &ModeProgress::default());
    let hostile = format!(
        r#"{{"regular":{{"level":0,"quests":{{"{A}":{{"status":"active","at":1}},"zz":{{"status":"active","at":1}}}},
        "objectives":{{"{OBJ}":0,"{B}":5}}}}}}"#
    );
    std::fs::write(&path, hostile).expect("write");
    let s = ProgressStore::load(&path);
    let p = s.get(GameMode::Regular);
    assert_eq!(p.level, 1);
    assert_eq!(p.quests.keys().collect::<Vec<_>>(), vec![A]);
    assert_eq!(p.objectives.get(B), Some(&5));
    assert_eq!(p.objectives.len(), 1);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn hideout_levels_are_kept_per_mode_and_checked() {
    let (_path, mut s) = store();
    let gen = "5d3b396e33c48f02b81cd9f3".to_owned();
    assert_eq!(s.apply(GameMode::Pve, Event::SetStation { station: gen.clone(), level: 2 }), Ok(true));
    assert_eq!(s.get(GameMode::Pve).hideout.get(&gen), Some(&2));
    assert!(s.get(GameMode::Regular).hideout.is_empty(), "each mode has its own hideout");
    assert!(s.apply(GameMode::Pve, Event::SetStation { station: "nope".into(), level: 1 }).is_err());
    assert!(s.apply(GameMode::Pve, Event::SetStation { station: gen.clone(), level: 11 }).is_err());
    assert_eq!(s.apply(GameMode::Pve, Event::SetStation { station: gen.clone(), level: 0 }), Ok(true));
    assert!(s.get(GameMode::Pve).hideout.is_empty(), "0 clears it");
}

#[test]
fn a_hand_edited_hideout_is_cleaned_on_load() {
    let path = tmp();
    let json = r#"{"pve":{"hideout":{"5d3b396e33c48f02b81cd9f3":3,"bad":1,"5d388e97081959000a123acf":99}}}"#;
    std::fs::write(&path, json).expect("write");
    let s = ProgressStore::load(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(s.get(GameMode::Pve).hideout.len(), 1);
}
