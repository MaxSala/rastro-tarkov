//! Partes de la vista que no van en cada instantánea principal: party y capas del mapa.

use crate::maps::manifest::{Manifest, MarkerKind};
use crate::maps::pack::Pack;
use crate::party::{lock, Status, PING_TTL_SECS};
use crate::state::App;
use crate::view::{heading_dir, marker, Marker};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub screenshots_dir: String,
    pub screenshots_dir_is_default: bool,
    pub trail_length: usize,
    pub player_name: String,
    pub player_color: u8,
    pub party_port: u16,
    pub share_warning_ack: bool,
    pub open_in_window: bool,
    pub language: crate::config::Language,
    pub check_updates: Option<bool>,
}

pub(crate) fn settings(cfg: &crate::config::Config) -> Settings {
    Settings {
        screenshots_dir: cfg.effective_screenshots_dir().map(|p| p.display().to_string()).unwrap_or_default(),
        screenshots_dir_is_default: cfg.screenshots_dir.is_none(),
        trail_length: cfg.trail_length,
        player_name: cfg.player_name.clone(),
        player_color: cfg.player_color,
        party_port: cfg.party_port,
        share_warning_ack: cfg.share_warning_ack,
        open_in_window: cfg.open_in_window,
        language: cfg.language,
        check_updates: cfg.check_updates,
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyView {
    pub status: &'static str,
    pub error: Option<crate::msg::UserMsg>,
    pub is_host: bool,
    pub code: Option<String>,
    pub peers: Vec<PeerView>,
    pub pings: Vec<PingView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerView {
    pub id: u8,
    pub name: String,
    pub color: u8,
    pub on_this_map: bool,
    pub marker: Option<Marker>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingView {
    pub color: u8,
    pub px: f64,
    pub py: f64,
    pub remaining_ms: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PointView {
    pub id: String,
    pub name: String,
    pub kind: crate::points::PointKind,
    pub note: String,
    pub px: f64,
    pub py: f64,
    pub distance_m: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guide {
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub distance_m: f64,
    pub height_diff_m: f64,
    /// Rumbo relativo a hacia donde miras: negativo = izquierda, positivo = derecha.
    pub relative_deg: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerView {
    pub kind: MarkerKind,
    /// Tipo de contenedor (para filtrar).
    pub group: Option<String>,
    /// Icono de tarkov.dev, si está descargado.
    pub icon: Option<&'static str>,
    pub name: String,
    pub px: f64,
    pub py: f64,
    /// Cerraduras: id de la llave y si necesita corriente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub power: bool,
    /// Floor the marker is on, for maps with floors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

/// Floor (level id) a game position is on, for maps with selectable floors: the first level
/// whose zones contain it, otherwise the base floor. `None` on maps without floors.
pub fn floor_at(m: &Manifest, x: f64, y: f64, z: f64) -> Option<String> {
    let base = m.image.base_layer.as_deref()?;
    let hit =
        m.levels.iter().filter(|lv| lv.svg_layer.is_some()).find(|lv| lv.zones.iter().any(|zn| zn.contains(x, y, z)));
    hit.or_else(|| m.levels.iter().find(|lv| lv.svg_layer.as_deref() == Some(base))).map(|lv| lv.id.clone())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyLocation {
    pub map_id: String,
    pub map_name: String,
    pub count: usize,
}

/// Mapas instalados con cerraduras que abre esta llave (para «Ver en el mapa» desde el Mercado).
pub fn key_locations(app: &App, key: &str) -> Vec<KeyLocation> {
    let inner = lock(&app.inner);
    let mut out: Vec<KeyLocation> = inner
        .store
        .all()
        .filter_map(|p| {
            let count = p.manifest.markers.iter().filter(|m| m.key.as_deref() == Some(key)).count();
            (count > 0).then(|| KeyLocation { map_id: p.manifest.id.clone(), map_name: p.manifest.name.clone(), count })
        })
        .collect();
    out.sort_by(|a, b| a.map_name.cmp(&b.map_name));
    out
}

/// Capas del mapa activo, ya proyectadas, y los iconos disponibles. Se envían al cambiar de mapa.
pub fn map_markers(app: &App) -> Option<(String, Vec<MarkerView>, Vec<String>)> {
    use crate::tarkovdev::icons;
    let available = icons::available(&app.paths.icons_dir);
    let has = |n: &str| available.iter().any(|a| a == n);
    let inner = lock(&app.inner);
    let pack = App::active_pack(&inner)?;
    drop(inner);
    let list = pack
        .manifest
        .markers
        .iter()
        .map(|m| {
            let (px, py) = pack.projection.project(m.x, m.z);
            let icon = match m.kind {
                MarkerKind::Loot => m.group.as_deref().and_then(icons::container_icon),
                MarkerKind::Transit => Some("extract_transit"),
                MarkerKind::Switch => Some("switch"),
                MarkerKind::Lock => Some("lock"),
            }
            .filter(|i| has(i));
            let (key, power) = (m.key.clone(), m.power);
            let floor = floor_at(&pack.manifest, m.x, m.y, m.z);
            MarkerView { kind: m.kind, group: m.group.clone(), icon, name: m.name.clone(), px, py, key, power, floor }
        })
        .collect();
    Some((pack.manifest.id.clone(), list, available))
}

pub(crate) fn party_view(app: &App, pack: Option<&Pack>) -> PartyView {
    let party = lock(&app.party);
    let r = lock(&party.roster);
    let (status, error) = match &r.status {
        Status::Idle => ("idle", None),
        Status::Hosting => ("hosting", None),
        Status::Connecting => ("connecting", None),
        Status::Connected => ("connected", None),
        Status::Error(e) => ("error", Some(e.clone())),
    };
    let map_id = pack.map(|p| p.manifest.id.as_str());
    let peers = r
        .peers
        .iter()
        .map(|(id, p)| {
            let here = p.pos.as_ref().filter(|q| Some(q.map.as_str()) == map_id);
            PeerView {
                id: *id,
                name: p.name.clone(),
                color: p.color,
                on_this_map: here.is_some(),
                marker: here.zip(pack).map(|(q, pk)| marker(pk, q.x, q.y, q.z, heading_dir(q.heading))),
            }
        })
        .collect();
    let ttl_ms = PING_TTL_SECS * 1000;
    let pings = r
        .pings
        .iter()
        .filter(|p| Some(p.map.as_str()) == map_id)
        .filter_map(|p| {
            let age = u64::try_from(p.at.elapsed().as_millis()).unwrap_or(u64::MAX);
            let (px, py) = pack?.projection.project(p.x, p.z);
            (age < ttl_ms).then(|| PingView { color: p.color, px, py, remaining_ms: ttl_ms.saturating_sub(age) })
        })
        .collect();
    PartyView { status, error, is_host: r.is_host, code: r.code.as_ref().map(|c| c.to_string()), peers, pings }
}

#[cfg(test)]
#[path = "floor_tests.rs"]
mod floor_tests;
