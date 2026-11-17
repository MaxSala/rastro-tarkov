use super::floor_at;
use crate::maps::manifest::Manifest;

fn manifest(base_layer: Option<&str>) -> Manifest {
    let mut v = serde_json::json!({
        "schema_version": 2,
        "id": "floors",
        "name": "Floors",
        "image": { "file": "floors.svg", "width": 100, "height": 100 },
        "calibration": { "points": [ { "game": [0, 0], "pixel": [0, 0] }, { "game": [100, 100], "pixel": [100, 100] } ] },
        "levels": [
            { "id": "ground", "name": "Ground floor", "y_min": -100, "y_max": 100, "svg_layer": "Ground_Level" },
            { "id": "second", "name": "2nd floor", "y_min": -100, "y_max": 100, "svg_layer": "Second_Floor",
              "zones": [ { "x": [10, 20], "z": [10, 20], "y": [4, 8] } ] }
        ],
        "extracts": [],
        "credits": { "author": "test", "license": "CC0-1.0", "source": "" }
    });
    if let (Some(b), Some(img)) = (base_layer, v.get_mut("image")) {
        img["base_layer"] = serde_json::json!(b);
    }
    serde_json::from_value(v).expect("manifest")
}

#[test]
fn a_position_inside_a_zone_is_on_that_floor() {
    let m = manifest(Some("Ground_Level"));
    assert_eq!(floor_at(&m, 15.0, 5.0, 15.0).as_deref(), Some("second"));
}

#[test]
fn outside_every_zone_is_the_base_floor() {
    let m = manifest(Some("Ground_Level"));
    assert_eq!(floor_at(&m, 15.0, 0.0, 15.0).as_deref(), Some("ground"), "below the zone");
    assert_eq!(floor_at(&m, 50.0, 5.0, 50.0).as_deref(), Some("ground"), "beside the zone");
}

#[test]
fn maps_without_floors_have_no_floor() {
    let m = manifest(None);
    assert_eq!(floor_at(&m, 15.0, 5.0, 15.0), None);
}
