use super::*;

fn dir() -> std::path::PathBuf {
    let d =
        std::env::temp_dir().join(format!("rastro-langdata-{}", crate::maps::import::random_hex(8).expect("random")));
    std::fs::create_dir_all(&d).expect("dir");
    d
}

#[test]
fn finds_a_cache_in_another_language_only_when_the_current_one_is_missing() {
    let d = dir();
    assert_eq!(cached_in_other_language(&d, "regular", Language::En), None, "nothing downloaded");
    std::fs::write(d.join("regular-es.json"), b"{}").expect("write");
    std::fs::write(d.join("pve-de.json"), b"{}").expect("write");
    std::fs::write(d.join("regular-xx.json"), b"{}").expect("write");
    assert_eq!(cached_in_other_language(&d, "regular", Language::En), Some(Language::Es));
    assert_eq!(cached_in_other_language(&d, "pve", Language::En), Some(Language::De));
    std::fs::write(d.join("regular-en.json"), b"{}").expect("write");
    assert_eq!(cached_in_other_language(&d, "regular", Language::En), None, "already in English");
    let _ = std::fs::remove_dir_all(&d);
}

#[test]
fn language_codes_round_trip() {
    for l in Language::ALL {
        assert_eq!(Language::from_code(l.code()), Some(l));
    }
    assert_eq!(Language::from_code("../es"), None);
}
