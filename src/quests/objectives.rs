//! Objectives: their text, items, keys and the places on each map where they're done.

use super::query::{id, item, map_id};
use super::raw::{RawObjective, RawPos, RawZone};
use super::{ItemRef, Objective, Spot, SpotKind};
use crate::tarkovdev::api::clean;

const MAX_SPOTS: usize = 64;
const MAX_ITEMS: usize = 16;
const MAX_KEY_GROUPS: usize = 8;
const MAX_OUTLINE: usize = 48;
const MAX_COORD: f64 = 100_000.0;
/// A zone wider than this (diagonal, metres) is an area to be in, not a spot to go to.
const AREA_M: f64 = 30.0;

fn coord(v: Option<f64>) -> Option<f64> {
    v.filter(|v| v.is_finite() && v.abs() <= MAX_COORD)
}

fn spot(map: Option<String>, p: Option<&RawPos>) -> Option<Spot> {
    let p = p?;
    Some(Spot {
        map: map?,
        x: coord(p.x)?,
        y: coord(p.y).unwrap_or(0.0),
        z: coord(p.z)?,
        outline: Vec::new(),
        kind: SpotKind::Exact,
    })
}

/// A zone's centre, with the outline of the area on the ground (x, z) when it has a usable one.
/// Its height is taken near the zone's floor rather than its middle: a tall trigger's centre can
/// sit on the floor above, and the floor decides which level of the map it's shown on.
fn zone(z: &RawZone) -> Option<Spot> {
    let mut s = spot(map_id(z.map.as_ref()), z.position.as_ref())?;
    if let Some(bottom) = coord(z.bottom).filter(|b| *b < s.y) {
        s.y = (bottom + 0.5).min(s.y);
    }
    let outline: Vec<[f64; 2]> = z
        .outline
        .iter()
        .flatten()
        .flatten()
        .filter_map(|p| Some([coord(p.x)?, coord(p.z)?]))
        .take(MAX_OUTLINE)
        .collect();
    if outline.len() >= 3 {
        let wide = outline.iter().any(|a| outline.iter().any(|b| (a[0] - b[0]).hypot(a[1] - b[1]) > AREA_M));
        if wide {
            s.kind = SpotKind::Area;
        }
        s.outline = outline;
    }
    Some(s)
}

fn spots(o: &RawObjective) -> Vec<Spot> {
    let from_locations = o.possible_locations.iter().flatten().flatten().flat_map(|l| {
        let map = map_id(l.map.as_ref());
        l.positions.iter().flatten().filter_map(move |p| spot(map.clone(), p.as_ref())).map(|mut s| {
            s.kind = SpotKind::Candidate;
            s
        })
    });
    let from_zones = o.zones.iter().flatten().flatten().filter_map(zone);
    let mut out: Vec<Spot> = Vec::new();
    for s in from_locations.chain(from_zones) {
        let near = out.iter().any(|o| o.map == s.map && (o.x - s.x).hypot(o.z - s.z) < 1.0 && (o.y - s.y).abs() < 1.0);
        if !near && out.len() < MAX_SPOTS {
            out.push(s);
        }
    }
    // A single possible spawn isn't a guess: it's the place.
    if out.iter().filter(|s| s.kind == SpotKind::Candidate).count() == 1 {
        out.iter_mut().filter(|s| s.kind == SpotKind::Candidate).for_each(|s| s.kind = SpotKind::Exact);
    }
    out
}

pub fn objective(o: &RawObjective) -> Option<Objective> {
    let kind: String = o.kind.as_deref()?.chars().filter(char::is_ascii_alphabetic).take(24).collect();
    let text = clean(o.description.as_deref(), 300, "");
    if kind.is_empty() || text.is_empty() {
        return None;
    }
    let positions = spots(o);
    let mut maps: Vec<String> = o.maps.iter().flatten().filter_map(|m| map_id(Some(m))).collect();
    for s in &positions {
        if !maps.contains(&s.map) {
            maps.push(s.map.clone());
        }
    }
    maps.dedup();
    maps.truncate(16);
    let listed = o.items.iter().flatten().flatten().filter_map(|i| item(Some(i)));
    let single = [o.item.as_ref(), o.marker_item.as_ref(), o.quest_item.as_ref()].into_iter().filter_map(item);
    let mut items: Vec<ItemRef> = Vec::new();
    for i in listed.chain(single) {
        if items.len() < MAX_ITEMS && !items.iter().any(|x| x.id == i.id) {
            items.push(i);
        }
    }
    let keys = o
        .required_keys
        .iter()
        .flatten()
        .flatten()
        .map(|group| group.iter().filter_map(|k| item(k.as_ref())).take(8).collect::<Vec<_>>())
        .filter(|g| !g.is_empty())
        .take(MAX_KEY_GROUPS)
        .collect();
    let count = o.count.filter(|c| c.is_finite() && *c >= 1.0).map_or(1, |c| c.min(100_000.0) as u32);
    Some(Objective {
        id: id(o.id.as_deref())?,
        kind,
        text,
        optional: o.optional.unwrap_or(false),
        count,
        fir: o.found_in_raid.unwrap_or(false),
        maps,
        items,
        keys,
        positions,
    })
}
