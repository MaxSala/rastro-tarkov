use super::*;
use crate::maps::manifest::RefPoint;

fn cal(rotation: u16, invert_x: bool, invert_z: bool) -> Calibration {
    Calibration {
        points: [
            RefPoint { game: [-500.0, -300.0], pixel: [100.0, 900.0] },
            RefPoint { game: [400.0, 250.0], pixel: [1900.0, 150.0] },
        ],
        invert_x,
        invert_z,
        rotation,
    }
}

#[test]
fn reference_points_map_exactly() {
    for rot in [0, 90, 180, 270] {
        let c = cal(rot, false, false);
        let p = Projection::new(&c).expect("válida");
        for pt in c.points {
            let (px, py) = p.project(pt.game[0], pt.game[1]);
            assert!((px - pt.pixel[0]).abs() < 1e-9 && (py - pt.pixel[1]).abs() < 1e-9);
        }
    }
}

#[test]
fn round_trip_all_orientations() {
    for rot in [0, 90, 180, 270] {
        for (ix, iz) in [(false, false), (true, false), (false, true), (true, true)] {
            let p = Projection::new(&cal(rot, ix, iz)).expect("válida");
            for (x, z) in [(0.0, 0.0), (-720.1, 430.51), (123.4, -987.6)] {
                let (px, py) = p.project(x, z);
                let (bx, bz) = p.unproject(px, py);
                assert!((bx - x).abs() < 1e-6 && (bz - z).abs() < 1e-6, "rot {rot}");
            }
        }
    }
}

#[test]
fn rejects_degenerate_and_bad_rotation() {
    let mut c = cal(0, false, false);
    c.points[1].game = [-500.0, 100.0];
    assert!(Projection::new(&c).is_err());
    assert!(Projection::new(&cal(45, false, false)).is_err());
}

#[test]
fn screen_angle_north_is_up_when_z_grows_up() {
    // Punto 2 tiene z mayor y py menor: +Z del juego apunta hacia arriba en pantalla.
    let p = Projection::new(&cal(0, false, false)).expect("válida");
    assert!(p.screen_angle(0.0, 1.0).abs() < 1e-6);
    assert!((p.screen_angle(1.0, 0.0) - 90.0).abs() < 1e-6);
}
