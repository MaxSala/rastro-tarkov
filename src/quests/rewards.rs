//! What a quest gives when it's handed in: experience, reputation, items, trader offers it
//! unlocks, skills, traders and crafts. Same rules as the rest of the quest data: everything is
//! optional, cleaned and capped.

use super::query::item;
use super::raw::{RawCount, RawRewards};
use super::ItemRef;
use crate::tarkovdev::api::clean;
use serde::{Deserialize, Serialize};

const MAX_ENTRIES: usize = 16;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Rewards {
    pub experience: u32,
    /// Reputation change with each trader (trader name, e.g. +0.02).
    pub standing: Vec<(String, f64)>,
    pub items: Vec<ItemCount>,
    /// Items a trader starts selling.
    pub offers: Vec<OfferUnlock>,
    /// Skill levels gained (skill name, levels).
    pub skills: Vec<(String, f64)>,
    /// Traders the quest unlocks.
    pub traders: Vec<String>,
    /// Hideout crafts it unlocks (the item crafted).
    pub crafts: Vec<ItemRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemCount {
    pub id: String,
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OfferUnlock {
    pub id: String,
    pub name: String,
    pub trader: String,
    pub level: u8,
}

fn finite(v: Option<f64>, max: f64) -> Option<f64> {
    v.filter(|v| v.is_finite() && v.abs() <= max)
}

fn counted(c: &RawCount) -> Option<ItemCount> {
    let i = item(c.item.as_ref())?;
    let count = finite(c.count, 10_000_000.0).filter(|c| *c >= 1.0).map_or(1, |c| c as u32);
    Some(ItemCount { id: i.id, name: i.name, count })
}

fn name(raw: Option<&str>) -> Option<String> {
    Some(clean(raw, 40, "")).filter(|n| !n.is_empty())
}

pub fn rewards(raw: Option<&RawRewards>, experience: Option<f64>) -> Rewards {
    let experience = finite(experience, 10_000_000.0).filter(|e| *e > 0.0).map_or(0, |e| e as u32);
    let Some(r) = raw else { return Rewards { experience, ..Rewards::default() } };
    let standing = r
        .trader_standing
        .iter()
        .flatten()
        .filter_map(|s| Some((name(s.trader.as_ref()?.name.as_deref())?, finite(s.standing, 10.0)?)))
        .filter(|(_, v)| *v != 0.0)
        .take(MAX_ENTRIES)
        .collect();
    let mut items: Vec<ItemCount> = Vec::new();
    for c in r.items.iter().flatten().filter_map(counted) {
        if let Some(x) = items.iter_mut().find(|x| x.id == c.id) {
            x.count = x.count.saturating_add(c.count);
        } else if items.len() < MAX_ENTRIES {
            items.push(c);
        }
    }
    let offers = r
        .offer_unlock
        .iter()
        .flatten()
        .filter_map(|o| {
            let i = item(o.item.as_ref())?;
            let trader = name(o.trader.as_ref()?.name.as_deref())?;
            let level = finite(o.level, 10.0).filter(|l| *l >= 1.0).map_or(1, |l| l as u8);
            Some(OfferUnlock { id: i.id, name: i.name, trader, level })
        })
        .take(MAX_ENTRIES)
        .collect();
    let skills = r
        .skill_level_reward
        .iter()
        .flatten()
        .filter_map(|s| Some((name(s.name.as_deref())?, finite(s.level, 100.0).filter(|l| *l > 0.0)?)))
        .take(MAX_ENTRIES)
        .collect();
    let traders = r.trader_unlock.iter().flatten().filter_map(|t| name(t.name.as_deref())).take(MAX_ENTRIES).collect();
    let crafts = r
        .craft_unlock
        .iter()
        .flatten()
        .flat_map(|c| c.reward_items.iter().flatten().filter_map(|i| item(i.item.as_ref())))
        .take(MAX_ENTRIES)
        .collect();
    Rewards { experience, standing, items, offers, skills, traders, crafts }
}
