use super::super::parser::{parse, Quat};
use super::*;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (normalize_180(a - b)).abs() < tol
}

/// Cuaternión de Unity para Euler (pitch alrededor de X, yaw alrededor de Y), orden Y·X.
fn from_yaw_pitch(yaw_deg: f64, pitch_deg: f64) -> Quat {
    let (hy, hp) = (yaw_deg.to_radians() / 2.0, pitch_deg.to_radians() / 2.0);
    let (sy, cy, sp, cp) = (hy.sin(), hy.cos(), hp.sin(), hp.cos());
    // q = qYaw * qPitch
    Quat { x: cy * sp, y: sy * cp, z: -sy * sp, w: cy * cp }
}

#[test]
fn sample_order_is_xyzw_and_heading_matches() {
    // Si el orden fuera w,x,y,z la cámara estaría casi tumbada (|x| ≈ 0.66): no plausible.
    // Con x,y,z,w la rotación es casi pura alrededor de Y y el morro baja ~5°.
    let c = parse("2026-08-07[19-31]_-720.10, -48.62, 430.51_-0.03195, 0.66069, -0.02987, -0.74938_11.67 (0).png")
        .expect("parsea");
    let f = forward(c.rot);
    assert!(f[1] < 0.0 && f[1] > -0.15, "pitch leve hacia abajo, f.y = {}", f[1]);
    assert!(approx(heading_deg(c.rot), 277.2, 0.5), "rumbo = {}", heading_deg(c.rot));
}

#[test]
fn identity_looks_along_plus_z() {
    let q = Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    assert!(approx(heading_deg(q), 0.0, 1e-6));
}

#[test]
fn yaw_ninety_looks_along_plus_x() {
    let q = from_yaw_pitch(90.0, 0.0);
    let f = forward(q);
    assert!((f[0] - 1.0).abs() < 1e-9 && f[2].abs() < 1e-9);
    assert!(approx(heading_deg(q), 90.0, 1e-6));
}

#[test]
fn pitch_up_and_down_keep_heading() {
    for yaw in [0.0, 45.0, 135.0, 200.0, 315.0] {
        for pitch in [-80.0, -30.0, 30.0, 80.0] {
            let q = from_yaw_pitch(yaw, pitch);
            assert!(approx(heading_deg(q), yaw, 1e-6), "yaw {yaw} pitch {pitch}");
        }
    }
}

#[test]
fn looking_straight_up_or_down_still_has_heading() {
    for yaw in [10.0, 100.0, 250.0] {
        assert!(approx(heading_deg(from_yaw_pitch(yaw, 90.0)), yaw, 1e-3));
        assert!(approx(heading_deg(from_yaw_pitch(yaw, -90.0)), yaw, 1e-3));
    }
}
