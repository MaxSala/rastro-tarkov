//! What the interface gets about quests besides the data itself: the progress (sent when it
//! changes) and, in every snapshot, the open objectives of your active quests on the map shown.

use super::progress::{ModeProgress, Status};
use super::{Faction, QuestDb};
use crate::capture::parser::Vec3;
use crate::gamelog::GameMode;
use crate::maps::pack::Pack;
use crate::party::lock;
use crate::state::App;
use crate::view_extra::floor_at;
use serde::Serialize;
use std::collections::BTreeMap;

/// One spot of an open objective, projected onto the map shown.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuestMarker {
    /// `qo-<objective id>-<n>`: also the guide target id.
    pub id: String,
    pub quest: String,
    pub objective: String,
    pub px: f64,
    pub py: f64,
    pub distance_m: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    /// Part of the quest being previewed from the Quests section.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub focus: bool,
    /// Outline of the zone in map pixels, when the objective is an area rather than a point.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outline: Vec<[f64; 2]>,
    pub kind: super::SpotKind,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressView {
    pub mode: GameMode,
    pub level: u8,
    pub faction: Faction,
    pub quests: BTreeMap<String, Status>,
    pub objectives: BTreeMap<String, u32>,
    pub hideout: BTreeMap<String, u8>,
    pub revision: u64,
}

const MAX_MARKERS: usize = 400;

/// Spots on `map` to show: the open objectives of active quests, and every objective of the
/// quest being previewed (`focus`) when it isn't active, flagged as a preview.
pub fn open_spots<'a>(
    db: &'a QuestDb,
    p: &'a ModeProgress,
    map: &'a str,
    focus: Option<&'a str>,
) -> impl Iterator<Item = (&'a str, &'a str, usize, &'a super::Spot, bool)> + 'a {
    db.quests
        .iter()
        .filter_map(move |q| {
            let active = p.quests.get(&q.id).is_some_and(|r| r.status == Status::Active);
            let focused = focus == Some(q.id.as_str());
            // An active quest is drawn as such even while previewed; only quests you haven't
            // started are drawn as a preview, with every objective.
            (active || focused).then_some((q, active, focused && !active))
        })
        .flat_map(move |(q, active, focused)| {
            q.objectives
                .iter()
                .filter(move |o| !active || p.objectives.get(&o.id).copied().unwrap_or(0) < o.count)
                .flat_map(move |o| {
                    o.positions
                        .iter()
                        .enumerate()
                        .filter(move |(_, s)| s.map == map)
                        .map(move |(n, s)| (q.id.as_str(), o.id.as_str(), n, s, focused))
                })
        })
}

/// Markers for the snapshot. Call without holding `inner`: it reads the game mode.
pub fn markers(app: &App, pack: &Pack, me: Option<Vec3>) -> Vec<QuestMarker> {
    let mode = app.effective_mode();
    let q = lock(&app.quests);
    let Some(db) = q.db.clone() else { return Vec::new() };
    let p = q.progress.get(mode);
    let m = &pack.manifest;
    let (w, h) = (f64::from(m.image.width), f64::from(m.image.height));
    // Like tarkov.dev, spots that fall outside the map image are bad data, not places to go.
    let inside = |px: f64, py: f64| (-0.02 * w..=1.02 * w).contains(&px) && (-0.02 * h..=1.02 * h).contains(&py);
    open_spots(&db, p, &m.id, q.focus.as_deref())
        .filter_map(|(quest, objective, n, s, focus)| {
            let (px, py) = pack.projection.project(s.x, s.z);
            inside(px, py).then(|| QuestMarker {
                id: format!("qo-{objective}-{n}"),
                quest: quest.to_owned(),
                objective: objective.to_owned(),
                px,
                py,
                distance_m: me.map(|l| (s.x - l.x).hypot(s.z - l.z)),
                floor: floor_at(m, s.x, s.y, s.z),
                focus,
                outline: s.outline.iter().map(|[x, z]| pack.projection.project(*x, *z)).map(|(a, b)| [a, b]).collect(),
                kind: s.kind,
            })
        })
        .take(MAX_MARKERS)
        .collect()
}

/// Parses a guide target id `qo-<objective>-<n>`.
pub fn parse_target(id: &str) -> Option<(&str, usize)> {
    let rest = id.strip_prefix("qo-")?;
    let (objective, n) = rest.split_once('-')?;
    crate::market::icons::valid_id(objective).then_some(())?;
    Some((objective, n.parse().ok().filter(|n: &usize| *n < 64)?))
}

/// Game coordinates of the quest spot chosen as guide target, if the selection is one.
/// Takes `inner` and `quests` one after the other, never both at once.
pub fn selected_spot(app: &App) -> Option<(f64, f64, f64)> {
    let (selected, map) = {
        let inner = lock(&app.inner);
        (inner.raid.selected.clone()?, inner.config.last_map.clone()?)
    };
    let (objective, n) = parse_target(&selected)?;
    let db = lock(&app.quests).db.clone()?;
    let o = db.quests.iter().flat_map(|q| &q.objectives).find(|o| o.id == objective)?;
    o.positions.get(n).filter(|s| s.map == map).map(|s| (s.x, s.y, s.z))
}

/// Changes whenever the progress or the game mode does.
pub fn progress_key(app: &App) -> String {
    let mode = app.effective_mode();
    format!("{mode:?}#{}", lock(&app.quests).progress.revision)
}

/// Quest previewed on the map (from "Show on map"); `None` stops the preview.
pub fn set_focus(app: &App, quest: Option<String>) -> crate::error::AppResult<()> {
    if quest.as_deref().is_some_and(|q| !crate::market::icons::valid_id(q)) {
        return Err(crate::error::AppError::bad("invalid quest id"));
    }
    lock(&app.quests).focus = quest.map(|q| q.to_ascii_lowercase());
    app.notify();
    Ok(())
}

pub fn progress_view(app: &App) -> ProgressView {
    let mode = app.effective_mode();
    let q = lock(&app.quests);
    let p = q.progress.get(mode);
    ProgressView {
        mode,
        level: p.level,
        faction: p.faction,
        quests: p.quests.iter().map(|(k, r)| (k.clone(), r.status)).collect(),
        objectives: p.objectives.clone(),
        hideout: p.hideout.clone(),
        revision: q.progress.revision,
    }
}

#[cfg(test)]
#[path = "view_tests.rs"]
mod tests;
