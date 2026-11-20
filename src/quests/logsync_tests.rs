use super::*;
use crate::quests::progress::{ProgressStore, Source};
use crate::quests::{Faction, Quest, QuestDb, Requirement};

fn quest(id: &str, level: u8, requires: &[&str]) -> Quest {
    Quest {
        id: id.into(),
        name: id.into(),
        slug: String::new(),
        trader: "prapor".into(),
        map: None,
        level,
        kappa: false,
        lightkeeper: false,
        faction: Faction::Any,
        wiki: None,
        requires: requires
            .iter()
            .map(|q| Requirement { quest: (*q).into(), status: vec!["complete".into()] })
            .collect(),
        loyalty: vec![],
        objectives: vec![],
        rewards: Default::default(),
        image: false,
    }
}

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbb";
const C: &str = "cccccccccccccccccccccccc";

fn db() -> QuestDb {
    QuestDb {
        mode: "regular".into(),
        lang: "en".into(),
        fetched_at: 0,
        traders: vec![],
        quests: vec![quest(A, 1, &[]), quest(B, 5, &[A]), quest(C, 12, &[B])],
    }
}

fn ev(q: &str, status: Status, at: u64) -> QuestLogEvent {
    QuestLogEvent { quest: q.into(), status, at, local: None }
}

#[test]
fn accepting_a_quest_completes_what_it_needs_and_sets_the_level() {
    let (list, level) = plan(Some(&db()), &[ev(C, Status::Active, 100)]);
    assert_eq!(level, 12);
    assert!(list.contains(&(A.into(), Status::Completed, 100)));
    assert!(list.contains(&(B.into(), Status::Completed, 100)));
    assert!(list.contains(&(C.into(), Status::Active, 100)));
}

#[test]
fn a_failed_quest_implies_nothing() {
    let (list, level) = plan(Some(&db()), &[ev(C, Status::Failed, 100)]);
    assert_eq!((list.len(), level), (1, 1));
}

#[test]
fn without_quest_data_only_the_states_themselves_apply() {
    let (list, level) = plan(None, &[ev(C, Status::Active, 100)]);
    assert_eq!((list.len(), level), (1, 1));
}

#[test]
fn a_later_change_by_hand_is_never_overwritten() {
    let path =
        std::env::temp_dir().join(format!("rastro-logsync-{}.json", crate::maps::import::random_hex(6).expect("rng")));
    let mut store = ProgressStore::load(&path);
    let mode = GameMode::Regular;
    store
        .apply(
            mode,
            crate::quests::progress::Event::SetStatus {
                quest: B.into(),
                status: Some(Status::Active),
                source: Source::Manual,
            },
        )
        .expect("ok");
    // The logs say B was completed a long time ago: the player's later choice stays.
    let changed = store
        .apply(mode, Event::FromLogs(vec![(B.into(), Status::Completed, 10), (A.into(), Status::Completed, 10)]))
        .expect("ok");
    assert!(changed);
    assert_eq!(store.status(mode, B), Some(Status::Active));
    assert_eq!(store.status(mode, A), Some(Status::Completed));
    assert_eq!(store.get(mode).quests.get(A).map(|r| r.source), Some(Source::Logs));
    // Applying the same again changes nothing.
    assert!(!store.apply(mode, Event::FromLogs(vec![(A.into(), Status::Completed, 10)])).expect("ok"));
    assert!(store.apply(mode, Event::RaiseLevel(20)).expect("ok"));
    assert!(!store.apply(mode, Event::RaiseLevel(5)).expect("ok"), "never lowers the level");
}
