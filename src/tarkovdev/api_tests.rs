use super::*;

const SAMPLE: &str = r#"{"data":{"maps":[{"normalizedName":"customs","name":"Aduanas",
  "extracts":[
    {"id":"d201ff0c1b0ee85652fd0ab8adb5cbf52772aa62","name":"Puerta de la gasolinera vieja","faction":"scav","switches":[],"position":{"x":300.51,"y":3.47,"z":-198.52}},
    {"id":"d201ff0c1b0ee85652fd0ab8adb5cbf52772aa62","name":"Duplicado\u202e","faction":"pmc","switches":[{"name":"Generador"}],"position":{"x":1,"y":2,"z":3}},
    {"id":null,"name":null,"faction":null,"position":null}
  ],
  "transits":[{"description":"Hacia Factory","position":{"x":5,"y":0,"z":5}}],
  "switches":null,
  "lootContainers":[{"lootContainer":{"name":"Caja de armas"},"position":{"x":1e300,"y":0,"z":0}},{"lootContainer":null,"position":{"x":9,"y":0,"z":9}}]
}]}}"#;

#[test]
fn parses_and_sanitizes() {
    let maps = parse(SAMPLE.as_bytes()).expect("ok");
    let m = maps.first().expect("mapa");
    assert_eq!(m.display_name().as_deref(), Some("Aduanas"));
    let ex = m.extracts();
    assert_eq!(ex.len(), 2, "la extracción sin posición se descarta");
    let ids: Vec<&str> = ex.iter().map(|e| e.id.as_str()).collect();
    assert_ne!(ids.first(), ids.get(1), "ids únicos");
    assert!(ids.iter().all(|id| crate::maps::manifest::is_valid_id(id)));
    let dup = ex.get(1).expect("2");
    assert_eq!(dup.name, "Duplicado");
    assert_eq!(dup.conditions, "Requiere activar: Generador");
    let mk = m.markers();
    assert_eq!(mk.len(), 2, "coordenada infinita descartada");
    assert!(mk.iter().any(|k| k.kind == MarkerKind::Loot && k.name == "Contenedor"));
}

#[test]
fn rejects_garbage() {
    assert!(parse(b"{}").is_err());
    assert!(parse(b"not json").is_err());
}
