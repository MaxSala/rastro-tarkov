// The shapes below follow real `push-notifications` and `application` logs (EFT 1.x), with
// made-up ids.
use super::*;

fn message(time: &str, dt: u64, template: &str) -> Vec<String> {
    vec![
        format!("{time}|1.0.6.5.46221|Info|push-notifications|Got notification | ChatMessageReceived"),
        "{".into(),
        "  \"type\": \"new_message\",".into(),
        "  \"dialogId\": \"54cb57776803fa99248b456e\",".into(),
        "  \"message\": {".into(),
        "    \"type\": 10,".into(),
        format!("    \"dt\": {dt},"),
        "    \"text\": \"quest started\",".into(),
        format!("    \"templateId\": \"{template}\","),
        "    \"hasRewards\": false".into(),
        "  }".into(),
        "}".into(),
    ]
}

fn read(lines: &[String]) -> Vec<QuestLogEvent> {
    let mut r = NotificationReader::default();
    lines.iter().filter_map(|l| r.line(l)).collect()
}

#[test]
fn reads_accepted_completed_and_failed() {
    let mut lines = message("2026-08-02 21:18:00.931", 1_785_719_880, "657315ddab5a49b71f098853 description");
    lines.extend(message("2026-08-02 21:40:00.000", 1_785_721_200, "5967725e86f774601a446662 successMessageText"));
    lines.extend(message(
        "2026-08-02 21:41:00.000",
        1_785_721_260,
        "5967733e86f774601a446663 successMessageText 58330581ace78e27b8b10cee 0",
    ));
    lines.extend(message("2026-08-02 21:42:00.000", 1_785_721_320, "5967530A86F77462BA22226B failMessageText"));
    let events = read(&lines);
    let got: Vec<(&str, Status, u64)> = events.iter().map(|e| (e.quest.as_str(), e.status, e.at)).collect();
    assert_eq!(
        got,
        vec![
            ("657315ddab5a49b71f098853", Status::Active, 1_785_719_880),
            ("5967725e86f774601a446662", Status::Completed, 1_785_721_200),
            ("5967733e86f774601a446663", Status::Completed, 1_785_721_260),
            ("5967530a86f77462ba22226b", Status::Failed, 1_785_721_320),
        ]
    );
    assert_eq!(events.first().and_then(|e| e.local.as_deref()), Some("2026-08-02T21:18:00.931"));
}

#[test]
fn other_messages_and_lines_are_ignored() {
    let mut lines = message("2026-08-02 21:18:00.931", 1, "5ac3475486f7741d6224abcd 0");
    lines
        .push("2026-08-02 20:48:20.433|1.0.6.5.46221|Info|push-notifications|Received notification: Type: Ping".into());
    lines.push("    \"templateId\": \"not-an-id description\",".into());
    lines.push(String::new());
    assert!(read(&lines).is_empty());
}

#[test]
fn a_message_without_its_own_time_does_not_borrow_the_previous_one() {
    let mut lines = message("2026-08-02 21:18:00.931", 1_000, "657315ddab5a49b71f098853 description");
    lines.push("2026-08-02 21:19:00.000|1.0|Info|push-notifications|Got notification | ChatMessageReceived".into());
    lines.push("    \"templateId\": \"5967725e86f774601a446662 successMessageText\",".into());
    let events = read(&lines);
    assert_eq!(events.len(), 2);
    assert!(events.get(1).is_some_and(|e| e.at > 1_000), "falls back to the current time, not the old dt");
}

#[test]
fn follows_the_mode_across_a_character_switch() {
    let mut t = ModeTracker::default();
    let line = |m: &str| format!("2026-08-09 10:00:00.000|1.3|Info|application|{m}");
    // As the game writes it: the mode first, then the character it loads.
    assert!(t.application_line(&line("Session mode: Pve")));
    t.application_line(&line("PrepareSelectedProfileLocally ProfileId:aaaaaaaaaaaaaaaaaaaaaaaa AccountId:1"));
    assert_eq!(t.current(), Some(GameMode::Pve));
    t.application_line(&line("Session mode: Regular"));
    t.application_line(&line("PrepareSelectedProfileLocally ProfileId:bbbbbbbbbbbbbbbbbbbbbbbb AccountId:1"));
    assert_eq!(t.current(), Some(GameMode::Regular));
    // Back to the first character without a mode line: it's still known to be the PvE one.
    t.application_line(&line("PrepareSelectedProfileLocally ProfileId:aaaaaaaaaaaaaaaaaaaaaaaa AccountId:1"));
    assert_eq!(t.current(), Some(GameMode::Pve));
    assert!(!format!("{t:?}").contains("aaaaaaaa"), "profile ids are never kept as text");
}

#[test]
fn the_timeline_answers_with_the_mode_loaded_then() {
    let mut tl = Timeline::default();
    tl.push("2026-08-09T10:00:00.000".into(), GameMode::Pve);
    tl.push("2026-08-09T14:00:00.000".into(), GameMode::Regular);
    assert_eq!(tl.at(Some("2026-08-09T12:00:00.000")), Some(GameMode::Pve));
    assert_eq!(tl.at(Some("2026-08-09T15:00:00.000")), Some(GameMode::Regular));
    assert_eq!(tl.at(Some("2026-08-09T09:00:00.000")), Some(GameMode::Pve), "before any: the first");
    assert_eq!(Timeline::default().at(Some("2026-08-09T09:00:00.000")), None);
}

#[test]
fn the_last_word_wins() {
    let e = |q: &str, status, at| QuestLogEvent { quest: q.into(), status, at, local: None };
    let folded = fold([
        e("a", Status::Active, 1),
        e("a", Status::Completed, 2),
        e("a", Status::Active, 3),
        e("c", Status::Active, 1),
        e("c", Status::Failed, 2),
    ]);
    let got: Vec<(&str, Status)> = folded.iter().map(|e| (e.quest.as_str(), e.status)).collect();
    assert_eq!(got, vec![("c", Status::Failed), ("a", Status::Active)]);
}
