//! Proyección entre coordenadas del juego (x, z) y píxeles del mapa, y su inversa.

use super::manifest::Calibration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection {
    sign_x: f64,
    sign_z: f64,
    rotation: u16,
    // px = ku · u + cu ; py = kv · v + cv
    ku: f64,
    cu: f64,
    kv: f64,
    cv: f64,
}

impl Projection {
    /// Ajusta escala y desplazamiento por eje con los dos puntos de referencia.
    pub fn new(cal: &Calibration) -> Result<Self, String> {
        if ![0, 90, 180, 270].contains(&cal.rotation) {
            return Err("rotation must be 0, 90, 180 or 270".into());
        }
        let mut p = Self {
            sign_x: if cal.invert_x { -1.0 } else { 1.0 },
            sign_z: if cal.invert_z { -1.0 } else { 1.0 },
            rotation: cal.rotation,
            ku: 1.0,
            cu: 0.0,
            kv: 1.0,
            cv: 0.0,
        };
        let [a, b] = cal.points;
        let (ua, va) = p.orient(a.game[0], a.game[1]);
        let (ub, vb) = p.orient(b.game[0], b.game[1]);
        let (du, dv) = (ub - ua, vb - va);
        if du.abs() < 1e-6 || dv.abs() < 1e-6 {
            return Err("the two calibration points must differ on both axes".into());
        }
        p.ku = (b.pixel[0] - a.pixel[0]) / du;
        p.kv = (b.pixel[1] - a.pixel[1]) / dv;
        if !p.ku.is_finite() || !p.kv.is_finite() || p.ku.abs() < 1e-9 || p.kv.abs() < 1e-9 {
            return Err("the calibration produces a zero or infinite scale".into());
        }
        p.cu = a.pixel[0] - p.ku * ua;
        p.cv = a.pixel[1] - p.kv * va;
        Ok(p)
    }

    /// Inversión y rotación horaria aplicadas a (x, z) antes del ajuste lineal.
    fn orient(&self, x: f64, z: f64) -> (f64, f64) {
        let (a, b) = (x * self.sign_x, z * self.sign_z);
        match self.rotation {
            90 => (b, -a),
            180 => (-a, -b),
            270 => (-b, a),
            _ => (a, b),
        }
    }

    fn unorient(&self, u: f64, v: f64) -> (f64, f64) {
        let (a, b) = match self.rotation {
            90 => (-v, u),
            180 => (-u, -v),
            270 => (v, -u),
            _ => (u, v),
        };
        (a * self.sign_x, b * self.sign_z)
    }

    pub fn project(&self, x: f64, z: f64) -> (f64, f64) {
        let (u, v) = self.orient(x, z);
        (self.ku * u + self.cu, self.kv * v + self.cv)
    }

    pub fn unproject(&self, px: f64, py: f64) -> (f64, f64) {
        self.unorient((px - self.cu) / self.ku, (py - self.cv) / self.kv)
    }

    /// Ángulo en pantalla (grados, horario desde "arriba") de una dirección horizontal del juego.
    pub fn screen_angle(&self, dir_x: f64, dir_z: f64) -> f64 {
        let (u, v) = self.orient(dir_x, dir_z);
        let (dx, dy) = (self.ku * u, self.kv * v);
        crate::capture::orient::normalize_360(dx.atan2(-dy).to_degrees())
    }
}

#[cfg(test)]
#[path = "project_tests.rs"]
mod tests;
