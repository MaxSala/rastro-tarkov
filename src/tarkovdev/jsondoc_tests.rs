use super::*;

const MAPS: &str = r#"{"data":{"maps":{"56f40101d2720b2a4d8b45d6":{"id":"56f40101d2720b2a4d8b45d6","normalizedName":"customs","name":"56f40101d2720b2a4d8b45d6",
  "switches":[{"id":"sw1","name":"Generador principal","position":{"x":1,"y":0,"z":1}}],
  "extracts":[{"id":"ex1","name":"key_crossroads","faction":"shared","switches":["sw1"],"position":{"x":-334.8,"y":1,"z":-87.98}},
              {"id":"ex2","name":"key_sinpos","faction":"pmc"}],
  "transits":[{"id":"t1","description":"key_transit","position":{"x":5,"y":0,"z":5}}],
  "lootContainers":[{"lootContainer":"5d6d2bb386f774785b07a77a","position":{"x":9,"y":0,"z":9}}],
  "locks":[{"lockType":"door","key":"5780cf7f2459777de4559322","needsPower":false,"position":{"x":3,"y":1,"z":4}},
           {"lockType":"trunk","key":{"id":"5913877a86f774432f15d444"},"needsPower":true,"position":{"x":6,"y":0,"z":7}},
           {"lockType":"door","key":"no-es-un-id","position":{"x":0,"y":0,"z":0}}]}}}}"#;
const LOCALE: &str = r#"{"data":{"56f40101d2720b2a4d8b45d6":"Aduanas","key_crossroads":"Cruce","key_transit":"Hacia Factory","5d6d2bb386f774785b07a77a Name":"Caja de armas"}}"#;

#[test]
fn resolves_locale_keys_and_switches() {
    let en = br#"{"data":{"5d6d2bb386f774785b07a77a Name":"Weapon box"}}"#;
    let maps = parse(MAPS.as_bytes(), LOCALE.as_bytes(), en).expect("ok");
    let m = maps.first().expect("mapa");
    assert_eq!(m.normalized_name, "customs");
    assert_eq!(m.name.as_deref(), Some("Aduanas"));
    assert_eq!(m.extracts.len(), 1, "sin posición se descarta");
    let ex = m.extracts.first().expect("ex");
    assert_eq!(ex.name, "Cruce");
    assert_eq!(ex.conditions, "Requiere activar: Generador principal");
    let names: Vec<&str> = m.markers.iter().map(|k| k.name.as_str()).collect();
    let loot = m.markers.iter().find(|k| k.kind == MarkerKind::Loot).expect("loot");
    assert_eq!(loot.group.as_deref(), Some("weapon-box"), "tipo deducido del nombre inglés");
    assert!(
        names.contains(&"Hacia Factory") && names.contains(&"Caja de armas") && names.contains(&"Generador principal")
    );
}

#[test]
fn works_without_locale_and_rejects_garbage() {
    let maps = parse(MAPS.as_bytes(), b"no json", b"").expect("sin textos también vale");
    assert_eq!(maps.first().map(|m| m.extracts.len()), Some(1));
    assert!(parse(b"{}", b"{}", b"").is_err());
    assert!(parse(b"[]", b"{}", b"").is_err());
}

#[test]
fn normalizes_like_tarkov_dev() {
    use super::super::api::normalize_name;
    assert_eq!(normalize_name("Weapon box"), "weapon-box");
    assert_eq!(normalize_name("Shturman's Stash"), "shturmans-stash");
    assert_eq!(normalize_name("PC block"), "pc-block");
}

#[test]
fn locks_keep_the_key_id() {
    let maps =
        parse(MAPS.as_bytes(), br#"{"data":{"5780cf7f2459777de4559322 Name":"Llave de la 314"}}"#, b"").expect("ok");
    let locks: Vec<_> = maps.iter().flat_map(|m| &m.markers).filter(|k| k.kind == MarkerKind::Lock).collect();
    assert_eq!(locks.len(), 2, "la llave con id no válido se descarta");
    let door = locks.first().expect("puerta");
    assert_eq!(
        (door.key.as_deref(), door.group.as_deref(), door.name.as_str()),
        (Some("5780cf7f2459777de4559322"), Some("door"), "Llave de la 314")
    );
    let trunk = locks.get(1).expect("maletero");
    assert_eq!(
        (trunk.key.as_deref(), trunk.group.as_deref(), trunk.power),
        (Some("5913877a86f774432f15d444"), Some("trunk"), true)
    );
}
