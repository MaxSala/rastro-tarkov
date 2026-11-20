use super::*;
use crate::quests::progress::Status;
use std::time::{Duration, SystemTime};

fn tmp() -> PathBuf {
    let d = std::env::temp_dir().join(format!("rastro-history-{}", crate::maps::import::random_hex(6).expect("rng")));
    std::fs::create_dir_all(&d).expect("mkdir");
    d
}

fn session(root: &Path, name: &str, age_hours: u64, application: &[&str], notes: &[String]) -> PathBuf {
    let s = root.join(format!("log_{name}"));
    std::fs::create_dir_all(&s).expect("mkdir");
    std::fs::write(s.join(format!("{name} application_000.log")), application.join("\n")).expect("w");
    std::fs::write(s.join(format!("{name} push-notifications_000.log")), notes.join("\n")).expect("w");
    let when = SystemTime::now() - Duration::from_secs(age_hours * 3600);
    if let Ok(f) = File::open(&s) {
        let _ = f.set_modified(when);
    }
    s
}

fn note(time: &str, dt: u64, template: &str) -> String {
    format!(
        "{time}|1.3|Info|push-notifications|Got notification | ChatMessageReceived\n{{\n  \"message\": {{\n    \"dt\": {dt},\n    \"templateId\": \"{template}\"\n  }}\n}}"
    )
}

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbb";
const Q1: &str = "5936d90786f7742b1420ba5b";
const Q2: &str = "5979ed3886f77431307dc512";
const Q3: &str = "59674eb386f774539f14813a";

#[test]
fn rebuilds_each_mode_from_every_launch() {
    let root = tmp();
    // Named so that sorting by name would put them in the wrong order (unpadded hour).
    session(
        &root,
        "2026.8.9_9-00-00",
        1,
        &[
            "2026-08-09 09:00:00.000|1.3|Info|application|Session mode: Regular",
            &format!(
                "2026-08-09 09:00:01.000|1.3|Info|application|PrepareSelectedProfileLocally ProfileId:{B} AccountId:1"
            ),
            &format!(
                "2026-08-09 10:00:00.000|1.3|Info|application|PrepareSelectedProfileLocally ProfileId:{A} AccountId:1"
            ),
        ],
        &[
            note("2026-08-09 09:30:00.000", 300, &format!("{Q3} description")),
            note("2026-08-09 10:30:00.000", 400, &format!("{Q1} successMessageText")),
        ],
    );
    session(
        &root,
        "2026.8.9_19-00-00",
        5,
        &[
            "2026-08-08 19:00:00.000|1.3|Info|application|Session mode: Pve",
            &format!(
                "2026-08-08 19:00:01.000|1.3|Info|application|PrepareSelectedProfileLocally ProfileId:{A} AccountId:1"
            ),
        ],
        &[
            note("2026-08-08 19:10:00.000", 100, &format!("{Q1} description")),
            note("2026-08-08 19:20:00.000", 200, &format!("{Q2} description")),
        ],
    );
    let h = scan(&root);
    assert_eq!(h.sessions, 2);
    let state = |mode: GameMode, q: &str| {
        h.events.iter().find(|(m, e)| *m == Some(mode) && e.quest == q).map(|(_, e)| e.status)
    };
    assert_eq!(state(GameMode::Pve, Q2), Some(Status::Active));
    // Q1 was accepted in PvE and handed in later, after switching back to the PvE character.
    assert_eq!(state(GameMode::Pve, Q1), Some(Status::Completed));
    assert_eq!(state(GameMode::Regular, Q3), Some(Status::Active));
    assert_eq!(state(GameMode::Regular, Q1), None, "the PvP character never touched it");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_empty_or_missing_folder_has_no_history() {
    let root = tmp();
    assert!(scan(&root).events.is_empty());
    assert!(scan(&root.join("missing")).events.is_empty());
    let _ = std::fs::remove_dir_all(&root);
}
