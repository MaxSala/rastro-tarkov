//! Vista para la web: todo proyectado ya a píxeles del mapa activo. La web solo dibuja.

use crate::capture::orient::{normalize_180, normalize_360};
use crate::maps::manifest::Faction;
use crate::maps::pack::Pack;
use crate::party::lock;
use crate::state::App;
use crate::view_extra::{floor_at, party_view, Guide, PartyView, PointView};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub version: &'static str,
    pub settings: crate::view_extra::Settings,
    pub watcher: Watcher,
    pub packs: Vec<PackInfo>,
    pub active_map: Option<ActiveMap>,
    pub player: Option<Marker>,
    pub trail: Vec<[f64; 2]>,
    pub extracts: Vec<ExtractView>,
    pub selected_extract: Option<String>,
    pub guide: Option<Guide>,
    pub points: Vec<PointView>,
    pub data_dir: String,
    pub party: PartyView,
    /// true mientras no se haya completado el asistente de primer arranque.
    pub onboarding: bool,
    pub tarkovdev: crate::tarkovdev::sync::SyncStatus,
    /// Detección por logs del juego (mapa, raid, modo).
    pub game: crate::gamelog::GameView,
    pub market: crate::market::MarketStatus,
    pub update: crate::update::UpdateStatus,
    pub quests: crate::quests::QuestsStatus,
    /// Downloaded data in another language than the interface.
    pub language_data: crate::langdata::LanguageData,
    /// Open objectives of your active quests on the map shown.
    pub quest_markers: Vec<crate::quests::view::QuestMarker>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Watcher {
    pub folder: Option<String>,
    pub error: Option<crate::msg::UserMsg>,
    pub last_file: Option<String>,
    pub last_taken_at: Option<String>,
    pub raid_hours: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackInfo {
    pub id: String,
    pub name: String,
    pub author: String,
    pub license: String,
    pub extracts: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMap {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub image_url: String,
    pub level: Option<String>,
    /// Floors of a layered SVG, lowest first (empty if the map has no floors).
    pub floors: Vec<FloorView>,
    /// Piso que corresponde a tu última posición según las zonas del mapa.
    pub auto_floor: Option<String>,
    /// The map's base floor: the one shown when no floor is picked and there's no position yet.
    pub base_floor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FloorView {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub px: f64,
    pub py: f64,
    /// Rotación en pantalla, grados horarios desde arriba.
    pub angle: f64,
    pub game_x: f64,
    pub game_y: f64,
    pub game_z: f64,
    /// Floor this position is on, for maps with floors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractView {
    pub id: String,
    pub name: String,
    pub faction: Faction,
    pub conditions: String,
    pub switches: Vec<String>,
    pub px: f64,
    pub py: f64,
    pub available: bool,
    pub dimmed: bool,
    /// Distancia horizontal desde la última posición, si la hay.
    pub distance_m: Option<f64>,
    /// Floor the extract is on, for maps with floors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

pub(crate) fn marker(pack: &Pack, x: f64, y: f64, z: f64, dir: (f64, f64)) -> Marker {
    let (px, py) = pack.projection.project(x, z);
    let angle = pack.projection.screen_angle(dir.0, dir.1);
    let floor = floor_at(&pack.manifest, x, y, z);
    Marker { px, py, angle, game_x: x, game_y: y, game_z: z, floor }
}

pub(crate) fn heading_dir(h: f64) -> (f64, f64) {
    let r = h.to_radians();
    (r.sin(), r.cos())
}

pub fn build(app: &App) -> View {
    // Read before taking `inner`: it takes `inner` and the quests lock one after the other.
    let quest_spot = crate::quests::view::selected_spot(app);
    let inner = lock(&app.inner);
    let cfg = &inner.config;
    let pack: Option<Arc<Pack>> = cfg.last_map.as_deref().and_then(|id| inner.store.get(id));
    let last = inner.raid.last.as_ref();
    let mut v = View {
        version: env!("CARGO_PKG_VERSION"),
        settings: crate::view_extra::settings(cfg),
        watcher: Watcher {
            folder: inner.watch_folder.as_ref().map(|p| p.display().to_string()),
            error: inner.watch_error.clone(),
            last_file: last.map(|l| l.file_name.clone()),
            last_taken_at: last.map(|l| l.taken_at.clone()),
            raid_hours: last.map(|l| l.raid_hours),
        },
        packs: inner
            .store
            .all()
            .map(|p| PackInfo {
                id: p.manifest.id.clone(),
                name: p.manifest.name.clone(),
                author: p.manifest.credits.author.clone(),
                license: p.manifest.credits.license.clone(),
                extracts: p.manifest.extracts.len(),
            })
            .collect(),
        active_map: None,
        player: None,
        trail: Vec::new(),
        extracts: Vec::new(),
        selected_extract: inner.raid.selected.clone(),
        points: Vec::new(),
        data_dir: app.paths.base.display().to_string(),
        guide: None,
        party: PartyView::default(),
        onboarding: !cfg.onboarding_done,
        tarkovdev: lock(&app.tarkovdev).clone(),
        game: Default::default(),
        market: Default::default(),
        update: lock(&app.update).clone(),
        quests: Default::default(),
        language_data: Default::default(),
        quest_markers: Vec::new(),
    };
    if let Some(pack) = &pack {
        let m = &pack.manifest;
        let level = last.and_then(|l| {
            m.levels.iter().filter(|lv| lv.svg_layer.is_none()).find(|lv| (lv.y_min..lv.y_max).contains(&l.pos.y))
        });
        // Floors from the lowest to the highest: the base floor counts as height 0, the others by
        // the bottom of their zones, so the interface can stack them like the building.
        let height = |lv: &crate::maps::manifest::Level| {
            lv.zones.iter().map(|z| z.y[0].min(z.y[1])).reduce(f64::min).unwrap_or(0.0)
        };
        let mut levels: Vec<_> = m.levels.iter().filter(|lv| lv.svg_layer.is_some()).collect();
        levels.sort_by(|a, b| height(a).total_cmp(&height(b)));
        let floors: Vec<FloorView> =
            levels.iter().map(|lv| FloorView { id: lv.id.clone(), name: lv.name.clone() }).collect();
        let auto_floor = last.and_then(|l| floor_at(m, l.pos.x, l.pos.y, l.pos.z));
        v.active_map = Some(ActiveMap {
            id: m.id.clone(),
            name: m.name.clone(),
            width: m.image.width,
            height: m.image.height,
            image_url: format!("/api/packs/{}/image", m.id),
            level: level.map(|lv| lv.name.clone()),
            floors,
            auto_floor,
            base_floor: m
                .image
                .base_layer
                .as_deref()
                .and_then(|b| m.levels.iter().find(|lv| lv.svg_layer.as_deref() == Some(b)))
                .map(|lv| lv.id.clone()),
        });
        v.player = last.map(|l| marker(pack, l.pos.x, l.pos.y, l.pos.z, l.dir));
        v.trail = inner.raid.trail.iter().map(|p| pack.projection.project(p.x, p.z)).map(|(a, b)| [a, b]).collect();
        let marked = inner.raid.available.get(&m.id);
        let any_marked = marked.is_some_and(|s| !s.is_empty());
        v.extracts = m
            .extracts
            .iter()
            .map(|e| {
                let (px, py) = pack.projection.project(e.x, e.z);
                let available = marked.is_some_and(|s| s.contains(&e.id));
                // TODO: suposición — sin ninguna marcada, ninguna se atenúa (aún no hay información).
                let dimmed = any_marked && !available;
                let distance_m = last.map(|l| (e.x - l.pos.x).hypot(e.z - l.pos.z));
                ExtractView {
                    id: e.id.clone(),
                    name: e.name.clone(),
                    faction: e.faction,
                    conditions: e.conditions.clone(),
                    switches: e.switches.clone(),
                    px,
                    py,
                    available,
                    dimmed,
                    distance_m,
                    floor: floor_at(m, e.x, e.y, e.z),
                }
            })
            .collect();
        let user_points: Vec<crate::points::UserPoint> =
            lock(&app.points).by_map.get(&m.id).cloned().unwrap_or_default();
        v.points = user_points
            .iter()
            .map(|p| {
                let (px, py) = pack.projection.project(p.x, p.z);
                PointView {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    kind: p.kind,
                    note: p.note.clone(),
                    px,
                    py,
                    distance_m: last.map(|l| (p.x - l.pos.x).hypot(p.z - l.pos.z)),
                    floor: floor_at(m, p.x, p.y, p.z),
                }
            })
            .collect();
        // El objetivo de la guía puede ser una extracción o un punto propio.
        let target = inner.raid.selected.as_ref().and_then(|id| {
            m.extracts
                .iter()
                .find(|e| &e.id == id)
                .map(|e| (e.x, e.y, e.z))
                .or_else(|| user_points.iter().find(|p| &p.id == id).map(|p| (p.x, p.y, p.z)))
                .or(quest_spot)
        });
        if let (Some(l), Some((ex, ey, ez))) = (last, target) {
            let e = crate::capture::parser::Vec3 { x: ex, y: ey, z: ez };
            let (dx, dz) = (e.x - l.pos.x, e.z - l.pos.z);
            let bearing = normalize_360(dx.atan2(dz).to_degrees());
            let (fx, fy) = pack.projection.project(l.pos.x, l.pos.z);
            let (tx, ty) = pack.projection.project(e.x, e.z);
            v.guide = Some(Guide {
                from: [fx, fy],
                to: [tx, ty],
                distance_m: dx.hypot(dz),
                height_diff_m: e.y - l.pos.y,
                relative_deg: normalize_180(bearing - l.heading),
            });
        }
    }
    let me = last.map(|l| l.pos);
    drop(inner);
    v.party = party_view(app, pack.as_deref());
    v.game = app.game_view();
    v.market = app.market_status();
    v.quests = app.quests_status();
    v.language_data = app.language_data();
    if let Some(p) = &pack {
        v.quest_markers = crate::quests::view::markers(app, p, me);
    }
    v
}
