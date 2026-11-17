use super::*;

const SAMPLE: &str = "2026-08-07[19-31]_-720.10, -48.62, 430.51_-0.03195, 0.66069, -0.02987, -0.74938_11.67 (0).png";

#[test]
fn parses_real_sample() {
    let c = parse(SAMPLE).expect("debe parsear");
    assert_eq!(c.taken_at, "2026-08-07 19:31");
    assert_eq!(c.pos, Vec3 { x: -720.10, y: -48.62, z: 430.51 });
    assert_eq!(c.rot, Quat { x: -0.03195, y: 0.66069, z: -0.02987, w: -0.74938 });
    assert!((c.raid_hours - 11.67).abs() < 1e-9);
}

#[test]
fn accepts_seconds_and_no_suffix() {
    let name = "2026-08-07[19-31-05]_1.5, 2, -3.25_0, 0, 0, 1_0.5.png";
    let c = parse(name).expect("debe parsear");
    assert_eq!(c.taken_at, "2026-08-07 19:31:05");
    assert_eq!(c.pos.z, -3.25);
}

#[test]
fn accepts_uppercase_extension_and_jpg() {
    assert!(parse("2026-08-07[19-31]_1, 2, 3_0, 0, 0, 1_1.PNG").is_some());
    assert!(parse("2026-08-07[19-31]_1, 2, 3_0, 0, 0, 1_1 (12).jpg").is_some());
}

#[test]
fn rejects_garbage() {
    for bad in [
        "",
        "captura.png",
        "2026-08-07[19-31]_1, 2_0, 0, 0, 1_1.png",
        "2026-08-07[19-31]_1, 2, 3_0, 0, 0, 1_1.png.exe",
        "2026-08-07[19-31]_1, 2, 3_0, 0, 0, 5_1.png", // cuaternión no unitario
        "2026-08-07[19-31]_999999, 2, 3_0, 0, 0, 1_1.png", // fuera de rango
    ] {
        assert!(parse(bad).is_none(), "debería rechazar {bad:?}");
    }
    assert!(parse(&"9".repeat(400)).is_none());
}
