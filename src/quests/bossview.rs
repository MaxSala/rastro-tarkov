//! Bosses on the map shown, for every snapshot: one marker per zone, at its most central spawn
//! point (a real spot, never the middle of a wall), with the zone's other points for the hover;
//! and the list of the map's bosses, with or without zones, for the raid card.

use super::bosses::Boss;
use crate::maps::pack::Pack;
use crate::party::lock;
use crate::state::App;
use crate::view_extra::floor_at;
use serde::Serialize;

const MAX_MARKERS: usize = 200;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BossMarker {
    pub id: String,
    pub boss: String,
    /// Zone name; empty when tarkov.dev has none.
    pub zone: String,
    /// Chance of the boss on this map, and of this zone when it comes (0 to 1).
    pub chance: f64,
    pub zone_chance: f64,
    pub px: f64,
    pub py: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    /// Every spawn point of the zone, in map pixels.
    pub points: Vec<[f64; 2]>,
    pub escorts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u32>,
    pub triggered: bool,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BossLine {
    pub name: String,
    pub chance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u32>,
    pub triggered: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BossesView {
    pub markers: Vec<BossMarker>,
    pub list: Vec<BossLine>,
}

/// The point with the smallest total distance to the others: the heart of the zone.
fn central(points: &[[f64; 3]]) -> Option<[f64; 3]> {
    let cost = |p: &[f64; 3]| points.iter().map(|q| (p[0] - q[0]).hypot(p[2] - q[2])).sum::<f64>();
    points.iter().copied().min_by(|a, b| cost(a).total_cmp(&cost(b)))
}

/// One line per boss name: several entries of the same one (Rogues at each building) read as
/// one, as likely as the likeliest and as early as the earliest.
pub fn lines(bosses: &[&Boss]) -> Vec<BossLine> {
    let mut out: Vec<BossLine> = Vec::new();
    for b in bosses {
        if let Some(l) = out.iter_mut().find(|l| l.name == b.name) {
            l.chance = l.chance.max(b.chance);
            l.time = l.time.zip(b.time).map(|(x, y)| x.min(y));
            l.triggered &= b.triggered;
        } else {
            out.push(BossLine { name: b.name.clone(), chance: b.chance, time: b.time, triggered: b.triggered });
        }
    }
    out.sort_by(|a, b| b.chance.total_cmp(&a.chance).then_with(|| a.name.cmp(&b.name)));
    out
}

pub fn view(app: &App, pack: &Pack) -> BossesView {
    let Some(db) = lock(&app.quests).db.clone() else { return BossesView::default() };
    let m = &pack.manifest;
    let here: Vec<&Boss> = db.bosses.iter().filter(|b| b.map == m.id).collect();
    let (w, h) = (f64::from(m.image.width), f64::from(m.image.height));
    let inside = |px: f64, py: f64| (-0.02 * w..=1.02 * w).contains(&px) && (-0.02 * h..=1.02 * h).contains(&py);
    let mut markers = Vec::new();
    for (i, b) in here.iter().enumerate() {
        for (n, z) in b.zones.iter().enumerate() {
            let Some(c) = central(&z.points) else { continue };
            let (px, py) = pack.projection.project(c[0], c[2]);
            if !inside(px, py) || markers.len() >= MAX_MARKERS {
                continue;
            }
            let points = z.points.iter().map(|p| pack.projection.project(p[0], p[2])).map(|(x, y)| [x, y]).collect();
            markers.push(BossMarker {
                id: format!("bz-{i}-{n}"),
                boss: b.name.clone(),
                zone: z.name.clone(),
                chance: b.chance,
                zone_chance: z.chance,
                px,
                py,
                floor: floor_at(m, c[0], c[1], c[2]),
                points,
                escorts: b.escorts.clone(),
                time: b.time,
                triggered: b.triggered,
            });
        }
    }
    BossesView { markers, list: lines(&here) }
}

#[cfg(test)]
#[path = "bossview_tests.rs"]
mod tests;
