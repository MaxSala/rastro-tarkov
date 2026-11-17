// Líneas reales de EFT 1.3.0.2.48779 (14/11/2026), con identificadores de perfil y raid anonimizados.
use super::*;

const SESSION: &[&str] = &[
    "2026-11-14 15:11:26.504|1.3.0.2.48779|Info|application|Session mode: Pve",
    "2026-11-14 15:13:04.970|1.3.0.2.48779|Info|application|PrepareSelectedProfileLocally ProfileId:aaaaaaaaaaaaaaaaaaaaaaaa AccountId:1",
    "2026-11-14 15:48:13.896|1.3.0.2.48779|Info|application|MatchingCompleted:0 real:0 diff:0",
    "2026-11-14 15:48:17.631|1.3.0.2.48779|Info|application|scene preset path:maps/laboratory_preset.bundle rcid:laboratory.ScenesPreset.asset",
    "2026-11-14 15:48:34.674|1.3.0.2.48779|Info|application|LocationLoaded:15.98 real:20.77 diff:4.79",
    "2026-11-14 15:48:40.603|1.3.0.2.48779|Info|application|[Transit] Flag:Common, RaidId:bbbbbbbbbbbbbbbbbbbbbbbb, Count:0, Locations:laboratory -> ",
    "2026-11-14 15:49:14.188|1.3.0.2.48779|Info|application|GameStarted:44.08(0) real:60.29(0) diff:16.2",
    "2026-11-14 16:00:02.675|1.3.0.2.48779|Info|backend|---> Request HTTPS, id [218]: URL: https://gw-pve.escapefromtarkov.com/client/match/local/end. ",
    "2026-11-14 16:00:03.479|1.3.0.2.48779|Info|backend|<--- Response HTTPS, id [218]: URL: https://gw-pve.escapefromtarkov.com/client/match/local/end, DownloadSeconds: 0.799, ParseSeconds: 0.005, SumSeconds: 0.804, responseText: ",
    "2026-11-14 16:00:06.629|1.3.0.2.48779|Info|application|PrepareSelectedProfileLocally ProfileId:aaaaaaaaaaaaaaaaaaaaaaaa AccountId:1",
];

#[test]
fn parses_real_lines() {
    let events: Vec<LogEvent> = SESSION.iter().filter_map(|l| parse_line(l)).collect();
    assert_eq!(
        events,
        vec![
            LogEvent::Mode(GameMode::Pve),
            LogEvent::ProfileReady,
            LogEvent::MapLoading("laboratory".into()),
            LogEvent::MapLoading("laboratory".into()),
            LogEvent::RaidStarted("2026-11-14T15:49:14.188".into()),
            LogEvent::RaidEnded,
            LogEvent::ProfileReady,
        ]
    );
}

#[test]
fn full_raid_cycle_acts_once_per_raid() {
    let mut st = GameState::default();
    let mut enters = 0;
    let mut in_raid_seen = false;
    for line in SESSION {
        if let Some(ev) = parse_line(line) {
            if let Some(Action::EnterMap(loc)) = st.apply(ev) {
                assert_eq!(loc, "laboratory");
                enters += 1;
            }
            in_raid_seen |= st.phase == Phase::Raid;
        }
    }
    assert_eq!(enters, 1, "preset + [Transit] = una sola raid");
    assert!(in_raid_seen);
    assert_eq!(st.phase, Phase::Menu);
    assert_eq!(st.mode, Some(GameMode::Pve));
    assert!(st.started_at.is_none());
}

#[test]
fn transit_chain_takes_last_location_and_rejects_garbage() {
    let l = "2026-11-14 15:48:40.603|1.3|Info|application|[Transit] Flag:Common, RaidId:x, Count:1, Locations:bigmap -> Woods -> ";
    assert_eq!(parse_line(l), Some(LogEvent::MapLoading("Woods".into())));
    let bad =
        "2026-11-14 15:48:40.603|1.3|Info|application|[Transit] Flag:Common, Locations:<script>alert(1)</script> -> ";
    assert_eq!(parse_line(bad), None);
    assert_eq!(parse_line("  \"Language\": \"en\","), None, "líneas de continuación");
    assert_eq!(parse_line(""), None);
    assert_eq!(parse_line(&"9".repeat(5000)), None);
}

#[test]
fn raid_start_without_map_is_ignored() {
    let mut st = GameState::default();
    assert_eq!(st.apply(LogEvent::RaidStarted("2026-11-14T15:49:14.188".into())), None);
    assert_eq!(st.phase, Phase::Menu);
}
