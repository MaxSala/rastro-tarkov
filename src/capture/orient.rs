//! Orientación de la cámara a partir del cuaternión de Unity (Y arriba, mano izquierda).

use super::parser::Quat;

/// Vector unitario resultante de rotar `v` con el cuaternión `q` (q · v · q⁻¹).
fn rotate(q: Quat, v: [f64; 3]) -> [f64; 3] {
    let [vx, vy, vz] = v;
    // t = 2 · (q.xyz × v)
    let tx = 2.0 * (q.y * vz - q.z * vy);
    let ty = 2.0 * (q.z * vx - q.x * vz);
    let tz = 2.0 * (q.x * vy - q.y * vx);
    // v' = v + w·t + q.xyz × t
    [
        vx + q.w * tx + (q.y * tz - q.z * ty),
        vy + q.w * ty + (q.z * tx - q.x * tz),
        vz + q.w * tz + (q.x * ty - q.y * tx),
    ]
}

/// Vector "hacia delante" de la cámara: Unity mira hacia +Z local.
pub fn forward(q: Quat) -> [f64; 3] {
    rotate(q, [0.0, 0.0, 1.0])
}

/// Dirección horizontal (x, z) normalizada hacia donde mira la cámara.
/// Si la cámara mira casi en vertical, usa su vector "arriba" (o el opuesto),
/// que en ese caso apunta en horizontal hacia donde mira el jugador.
pub fn horizontal_dir(q: Quat) -> (f64, f64) {
    let f = forward(q);
    let (mut hx, mut hz) = (f[0], f[2]);
    if (hx * hx + hz * hz).sqrt() < 1e-3 {
        let up = rotate(q, [0.0, 1.0, 0.0]);
        // Mirando abajo, "arriba" apunta adelante; mirando arriba, apunta atrás.
        let sign = if f[1] < 0.0 { 1.0 } else { -1.0 };
        hx = up[0] * sign;
        hz = up[2] * sign;
    }
    let len = (hx * hx + hz * hz).sqrt();
    if len < 1e-9 {
        return (0.0, 1.0);
    }
    (hx / len, hz / len)
}

/// Rumbo en grados [0, 360): 0 = +Z del juego, creciendo hacia +X (horario visto desde arriba).
pub fn heading_deg(q: Quat) -> f64 {
    let (hx, hz) = horizontal_dir(q);
    normalize_360(hx.atan2(hz).to_degrees())
}

pub fn normalize_360(deg: f64) -> f64 {
    let d = deg.rem_euclid(360.0);
    if d >= 360.0 {
        0.0
    } else {
        d
    }
}

/// Normaliza a (-180, 180].
pub fn normalize_180(deg: f64) -> f64 {
    let d = normalize_360(deg);
    if d > 180.0 {
        d - 360.0
    } else {
        d
    }
}

#[cfg(test)]
#[path = "orient_tests.rs"]
mod tests;
