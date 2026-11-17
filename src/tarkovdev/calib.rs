//! Calibración a partir de maps.json de tarkov.dev.
//!
//! Regla (verificada con las extracciones conocidas de los 10 mapas y con los recuadros de
//! edificios por piso): con θ = coordinateRotation, u = x·cosθ − z·sinθ y v = x·sinθ + z·cosθ;
//! en el SVG, px crece con u y py decrece con v. `svgBounds`, si existe, manda sobre `bounds`.

use crate::maps::manifest::{is_valid_id, is_valid_layer_id, Calibration, RefPoint, Zone};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Group {
    #[serde(rename = "normalizedName")]
    normalized_name: String,
    #[serde(default)]
    maps: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "svgPath")]
    svg_path: Option<String>,
    #[serde(rename = "coordinateRotation", default)]
    rotation: f64,
    bounds: Option<[[f64; 2]; 2]>,
    #[serde(rename = "svgBounds")]
    svg_bounds: Option<[[f64; 2]; 2]>,
    author: Option<String>,
    #[serde(rename = "svgLayer")]
    svg_layer: Option<String>,
    #[serde(default)]
    layers: Vec<Value>,
}

/// Un piso del SVG: nombre, grupo `<g>` y zonas donde se aplica.
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub name: String,
    pub svg_layer: String,
    pub zones: Vec<Zone>,
}

#[derive(Debug, Clone)]
pub struct SvgMap {
    pub id: String,
    pub file: String,
    pub rotation: u16,
    pub bounds: [[f64; 2]; 2],
    pub author: String,
    pub base_layer: Option<String>,
    pub layers: Vec<LayerInfo>,
}

/// Nombres de pisos habituales de tarkov.dev en español.
pub fn floor_name(en: &str) -> String {
    let lower = en.to_lowercase();
    let known = [
        ("underground", "Subterráneo"),
        ("basement", "Sótano"),
        ("tunnel", "Túneles"),
        ("bunker", "Búnkeres"),
        ("roof", "Tejado"),
        ("garage", "Garaje"),
        ("1st", "1.ª planta"),
        ("2nd", "2.ª planta"),
        ("3rd", "3.ª planta"),
        ("4th", "4.ª planta"),
        ("5th", "5.ª planta"),
        ("ground", "Planta baja"),
    ];
    known
        .iter()
        .find(|(k, _)| lower.contains(k))
        .map_or_else(|| en.chars().take(40).collect(), |(_, v)| (*v).to_owned())
}

fn parse_layer(v: &Value) -> Option<LayerInfo> {
    let svg_layer = v.get("svgLayer")?.as_str()?.to_owned();
    if !is_valid_layer_id(&svg_layer) {
        return None;
    }
    let name = floor_name(v.get("name").and_then(Value::as_str).unwrap_or(&svg_layer));
    let num = |x: &Value| x.as_f64().filter(|n| n.is_finite() && n.abs() <= 100_000.0);
    let mut zones = Vec::new();
    for ext in v.get("extents").and_then(Value::as_array).into_iter().flatten() {
        let h = ext.get("height").and_then(Value::as_array);
        let (Some(y0), Some(y1)) = (h.and_then(|h| h.first()).and_then(num), h.and_then(|h| h.get(1)).and_then(num))
        else {
            continue;
        };
        for b in ext.get("bounds").and_then(Value::as_array).into_iter().flatten() {
            let corner = |i: usize| -> Option<[f64; 2]> {
                let c = b.get(i)?.as_array()?;
                Some([num(c.first()?)?, num(c.get(1)?)?])
            };
            if let (Some(a), Some(c)) = (corner(0), corner(1)) {
                zones.push(Zone { x: [a[0], c[0]], z: [a[1], c[1]], y: [y0, y1] });
            }
        }
    }
    zones.truncate(64);
    Some(LayerInfo { name, svg_layer, zones })
}

fn valid_file(f: &str) -> bool {
    f.len() <= 64
        && f.ends_with(".svg")
        && f.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
}

/// Mapas con SVG. Datos externos: se filtra todo lo que no encaje.
pub fn parse_maps(bytes: &[u8]) -> Result<Vec<SvgMap>, String> {
    let groups: Vec<Group> = serde_json::from_slice(bytes).map_err(|e| format!("maps.json no válido: {e}"))?;
    let mut out = Vec::new();
    for g in groups {
        if !is_valid_id(&g.normalized_name) {
            continue;
        }
        let Some(e) = g.maps.into_iter().find(|e| e.svg_path.is_some()) else { continue };
        let file = e.svg_path.as_deref().and_then(|p| p.rsplit('/').next()).unwrap_or_default().to_owned();
        let rot = e.rotation.rem_euclid(360.0);
        let rotation = [0u16, 90, 180, 270].into_iter().find(|r| (f64::from(*r) - rot).abs() < 1e-6);
        let bounds = e.svg_bounds.or(e.bounds);
        let finite = bounds.is_some_and(|b| b.iter().flatten().all(|v| v.is_finite() && v.abs() < 100_000.0));
        if let (true, Some(rotation), Some(bounds), true) = (valid_file(&file), rotation, bounds, finite) {
            let author = e.author.unwrap_or_default();
            let base_layer = e.svg_layer.filter(|l| is_valid_layer_id(l));
            let layers = e.layers.iter().filter_map(parse_layer).take(15).collect();
            out.push(SvgMap { id: g.normalized_name, file, rotation, bounds, author, base_layer, layers });
        }
    }
    Ok(out)
}

/// Ancho y alto del SVG (viewBox, redondeado). None si no se puede leer.
pub fn svg_size(svg: &str) -> Option<(u32, u32)> {
    let start = svg.find("viewBox=\"")? + 9;
    let rest = svg.get(start..)?;
    let vb = rest.get(..rest.find('"')?)?;
    let nums: Vec<f64> = vb.split([' ', ',']).filter(|s| !s.is_empty()).filter_map(|s| s.parse().ok()).collect();
    let (w, h) = (*nums.get(2)?, *nums.get(3)?);
    let ok = |v: f64| v.is_finite() && (1.0..=8192.0).contains(&v.round());
    (ok(w) && ok(h)).then(|| (w.round() as u32, h.round() as u32))
}

pub fn calibration(map: &SvgMap, w: u32, h: u32) -> Calibration {
    let th = f64::from(map.rotation).to_radians();
    let (c, s) = (th.cos().round(), th.sin().round());
    let uv = |p: [f64; 2]| (p[0] * c - p[1] * s, p[0] * s + p[1] * c);
    let [b0, b1] = map.bounds;
    let ((u0, v0), (u1, v1)) = (uv(b0), uv(b1));
    let (umin, umax, vmin, vmax) = (u0.min(u1), u0.max(u1), v0.min(v1), v0.max(v1));
    let px = |u: f64| (u - umin) / (umax - umin) * f64::from(w);
    let py = |v: f64| (vmax - v) / (vmax - vmin) * f64::from(h);
    Calibration {
        points: [RefPoint { game: b0, pixel: [px(u0), py(v0)] }, RefPoint { game: b1, pixel: [px(u1), py(v1)] }],
        invert_x: false,
        invert_z: false,
        // Con 90/270 el eje horizontal del mapa depende de z: se usa la rotación de Rastro.
        rotation: if map.rotation % 180 == 0 { 0 } else { 90 },
    }
}

#[cfg(test)]
#[path = "calib_tests.rs"]
mod tests;
