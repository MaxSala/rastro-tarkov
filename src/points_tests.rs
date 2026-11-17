use super::*;

fn tmp() -> PathBuf {
    std::env::temp_dir().join(format!("rastro-pt-{}.json", crate::maps::import::random_hex(6).expect("rng")))
}

#[test]
fn load_drops_invalid_entries() {
    let path = tmp();
    let json = r#"{
      "customs": [
        {"id":"pt-abc123","name":"Alijo","kind":"stash","note":"","x":1,"y":2,"z":3},
        {"id":"evil","name":"x","kind":"stash","x":1,"y":2,"z":3},
        {"id":"pt-def456","name":"bad\u202e","kind":"loot","x":1,"y":2,"z":3},
        {"id":"pt-aaa111","name":"lejos","kind":"loot","x":1e9,"y":2,"z":3}
      ],
      "../x": [{"id":"pt-bbb222","name":"x","kind":"key","x":1,"y":2,"z":3}]
    }"#;
    std::fs::write(&path, json).expect("w");
    let store = PointStore::load(&path);
    assert_eq!(store.by_map.len(), 1);
    assert_eq!(store.by_map.get("customs").map(Vec::len), Some(1));
}

#[test]
fn garbage_file_starts_empty() {
    let path = tmp();
    std::fs::write(&path, "no json").expect("w");
    assert!(PointStore::load(&path).by_map.is_empty());
    assert!(check_text(&"x".repeat(41), 40, "nombre", false).is_err());
    assert!(check_text("  ", 40, "nombre", false).is_err());
    assert!(check_text("Llave del 314", 40, "nombre", false).is_ok());
}
