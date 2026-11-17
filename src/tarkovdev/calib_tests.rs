use super::*;
use crate::maps::project::Projection;

fn map(rotation: u16, bounds: [[f64; 2]; 2]) -> SvgMap {
    SvgMap {
        id: "x".into(),
        file: "X.svg".into(),
        rotation,
        bounds,
        author: String::new(),
        base_layer: None,
        layers: Vec::new(),
    }
}

#[test]
fn customs_top_left_is_first_bound() {
    // maps.json: customs, rotación 180, bounds [[698,-307],[-372,237]], SVG 1062x535.
    let m = map(180, [[698.0, -307.0], [-372.0, 237.0]]);
    let p = Projection::new(&calibration(&m, 1062, 535)).expect("válida");
    let (a, b) = p.project(698.0, -307.0);
    assert!(a.abs() < 1e-9 && b.abs() < 1e-9);
    let (a, b) = p.project(-372.0, 237.0);
    assert!((a - 1062.0).abs() < 1e-9 && (b - 535.0).abs() < 1e-9);
    // Extracción «Crossroads» (−334.8, −88): borde derecho, mitad superior.
    let (a, b) = p.project(-334.8, -87.98);
    assert!(a > 1000.0 && b < 268.0, "{a} {b}");
}

#[test]
fn factory_rotation_90_swaps_axes() {
    let m = map(90, [[77.0, -64.5], [-65.5, 67.4]]);
    let p = Projection::new(&calibration(&m, 131, 141)).expect("válida");
    // px depende de z (decrece), py de x (decrece).
    let (a1, b1) = p.project(0.0, -60.0);
    let (a2, b2) = p.project(0.0, 60.0);
    assert!(a1 > a2 && (b1 - b2).abs() < 1e-9);
    let (a3, b3) = p.project(70.0, 0.0);
    assert!(b3 < b1 && (a3 - p.project(0.0, 0.0).0).abs() < 1e-9);
}

#[test]
fn parses_only_usable_svg_maps() {
    let json = br#"[
      {"normalizedName":"customs","maps":[{"key":"customs","svgPath":"https://assets.tarkov.dev/maps/svg/Customs.svg","coordinateRotation":180,"bounds":[[698,-307],[-372,237]],"author":"Shebuka"},{"key":"c2"}]},
      {"normalizedName":"labs","maps":[{"key":"labs"}]},
      {"normalizedName":"../evil","maps":[{"svgPath":"x/E.svg","coordinateRotation":0,"bounds":[[1,1],[2,2]]}]},
      {"normalizedName":"odd","maps":[{"svgPath":"x/../../a.svg","coordinateRotation":45,"bounds":[[1,1],[2,2]]}]}
    ]"#;
    let maps = parse_maps(json).expect("ok");
    assert_eq!(maps.len(), 1);
    assert_eq!(
        maps.first().map(|m| (m.id.as_str(), m.file.as_str(), m.rotation)),
        Some(("customs", "Customs.svg", 180))
    );
}

#[test]
fn reads_viewbox() {
    assert_eq!(svg_size(r#"<svg viewBox="0 0 1062.4827 535.17401">"#), Some((1062, 535)));
    assert_eq!(svg_size(r#"<svg viewBox="0,0,99999,10">"#), None);
    assert_eq!(svg_size("<svg>"), None);
}

#[test]
fn parses_floor_layers_with_zones() {
    let json = br#"[{"normalizedName":"customs","maps":[{"svgPath":"x/Customs.svg","coordinateRotation":180,
      "bounds":[[698,-307],[-372,237]],"svgLayer":"Ground_Level",
      "layers":[{"name":"2nd Floor","svgLayer":"Second_Floor","extents":[{"height":[2.7,6.5],"bounds":[[[243,190],[165,125],"dorms"]]}]},
                {"name":"Evil","svgLayer":"x}{display:block"},
                {"name":"Underground","svgLayer":"Underground_Level","extents":[]}]}]}]"#;
    let m = parse_maps(json).expect("ok");
    let c = m.first().expect("mapa");
    assert_eq!(c.base_layer.as_deref(), Some("Ground_Level"));
    assert_eq!(c.layers.len(), 2, "el id con caracteres CSS se descarta");
    let second = c.layers.first().expect("piso");
    assert_eq!(second.name, "2nd floor");
    assert!(second.zones.first().expect("zona").contains(200.0, 4.0, 150.0));
    assert!(!second.zones.first().expect("zona").contains(200.0, 1.0, 150.0));
    assert_eq!(c.layers.get(1).map(|l| l.name.as_str()), Some("Underground"));
}
