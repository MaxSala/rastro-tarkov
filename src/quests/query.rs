//! The tarkov.dev GraphQL query for tasks and its conversion into [`QuestDb`]. Everything in the
//! answer is treated as untrusted: unknown or malformed parts are skipped, texts are cleaned and
//! every list is capped.

use super::objectives::objective;
use super::raw::{Named, RawItem, RawRequirement, RawTask, Response};
use super::rewards::rewards;
use super::{pack_id_for, Faction, ItemRef, Quest, QuestDb, Requirement, TraderRef};
use crate::maps::manifest::is_valid_id;
use crate::tarkovdev::api::clean;

const MAX_QUESTS: usize = 3000;
const MAX_OBJECTIVES: usize = 40;
const WIKI: &str = "https://escapefromtarkov.fandom.com/";
/// Quest and trader pictures live at `<this><id>.webp`; Rastro builds that address itself.
const ASSETS: &str = "https://assets.tarkov.dev/";

/// Checked against tarkov.dev's schema. `{lang}` and `{mode}` are filled in from a closed list.
pub const QUERY: &str = include_str!("tasks.graphql");

/// 24 hex digits, lower case.
pub(super) fn id(raw: Option<&str>) -> Option<String> {
    raw.map(str::to_ascii_lowercase).filter(|s| crate::market::icons::valid_id(s))
}

pub(super) fn map_id(n: Option<&Named>) -> Option<String> {
    let raw = n?.normalized_name.as_deref()?;
    Some(pack_id_for(raw).to_owned()).filter(|m| is_valid_id(m))
}

pub(super) fn item(raw: Option<&RawItem>) -> Option<ItemRef> {
    let r = raw?;
    Some(ItemRef { id: id(r.id.as_deref())?, name: clean(r.name.as_deref(), 120, "") })
}

fn requirement(r: &RawRequirement) -> Option<Requirement> {
    let quest = id(r.task.as_ref()?.id.as_deref())?;
    let status: Vec<String> = r
        .status
        .iter()
        .flatten()
        .map(|s| s.to_ascii_lowercase())
        .filter(|s| (1..=20).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_lowercase()))
        .take(4)
        .collect();
    Some(Requirement { quest, status: if status.is_empty() { vec!["complete".into()] } else { status } })
}

fn quest(t: &RawTask) -> Option<Quest> {
    let trader = t.trader.as_ref()?;
    let trader_id = trader.normalized_name.as_deref().filter(|n| is_valid_id(n))?.to_owned();
    let slug = t.normalized_name.as_deref().filter(|s| s.len() <= 80 && is_valid_slug(s)).unwrap_or("").to_owned();
    let faction = match t.faction_name.as_deref() {
        Some("USEC") => Faction::Usec,
        Some("BEAR") => Faction::Bear,
        _ => Faction::Any,
    };
    let loyalty = t
        .trader_requirements
        .iter()
        .flatten()
        .filter(|r| r.requirement_type.as_deref() == Some("level"))
        .filter_map(|r| {
            let name = clean(r.trader.as_ref()?.name.as_deref(), 40, "");
            let level = r.value.filter(|v| (1.0..=10.0).contains(v))? as u8;
            (!name.is_empty()).then_some((name, level))
        })
        .take(4)
        .collect();
    Some(Quest {
        id: id(t.id.as_deref())?,
        name: clean(t.name.as_deref(), 120, "").chars().take(120).collect::<String>(),
        slug,
        trader: trader_id,
        map: map_id(t.map.as_ref()),
        level: t.min_player_level.filter(|l| l.is_finite()).map_or(0, |l| l.clamp(0.0, 100.0) as u8),
        kappa: t.kappa_required.unwrap_or(false),
        lightkeeper: t.lightkeeper_required.unwrap_or(false),
        faction,
        wiki: t.wiki_link.as_deref().filter(|w| w.starts_with(WIKI) && w.len() <= 300).map(|w| clean(Some(w), 300, "")),
        requires: t.task_requirements.iter().flatten().filter_map(requirement).take(16).collect(),
        loyalty,
        objectives: t.objectives.iter().flatten().filter_map(objective).take(MAX_OBJECTIVES).collect(),
        rewards: rewards(t.finish_rewards.as_ref(), t.experience),
        image: has_image(t),
    })
    .filter(|q| !q.name.is_empty())
}

/// tarkov.dev has a picture for the quest (not its "unknown task" placeholder).
fn has_image(t: &RawTask) -> bool {
    let (Some(id), Some(link)) = (id(t.id.as_deref()), t.task_image_link.as_deref()) else { return false };
    link.strip_prefix(ASSETS).and_then(|f| f.strip_suffix(".webp")).is_some_and(|f| f.eq_ignore_ascii_case(&id))
}

fn is_valid_slug(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn traders(tasks: &[RawTask], quests: &[Quest]) -> Vec<TraderRef> {
    let mut out: Vec<TraderRef> = Vec::new();
    for t in tasks.iter().filter_map(|t| t.trader.as_ref()) {
        let Some(id) = t.normalized_name.as_deref().filter(|n| is_valid_id(n)) else { continue };
        if out.iter().all(|x| x.id != id) && quests.iter().any(|q| q.trader == id) {
            let image = self::id(t.id.as_deref());
            out.push(TraderRef { id: id.to_owned(), name: clean(t.name.as_deref(), 40, id), image });
        }
    }
    out
}

/// GraphQL answer → quest data. Fails only when nothing usable came back.
pub fn parse(bytes: &[u8], mode: &str, lang: &str, fetched_at: u64) -> Result<QuestDb, String> {
    build(serde_json::from_slice(bytes).map_err(|e| format!("invalid tasks answer: {e}"))?, mode, lang, fetched_at)
}

/// The same, from json.tarkov.dev's files already put in the GraphQL shape (see `jsondoc`).
pub fn parse_value(v: serde_json::Value, mode: &str, lang: &str, at: u64) -> Result<QuestDb, String> {
    build(serde_json::from_value(v).map_err(|e| format!("invalid tasks document: {e}"))?, mode, lang, at)
}

fn build(resp: Response, mode: &str, lang: &str, fetched_at: u64) -> Result<QuestDb, String> {
    let (tasks, stations, maps) = resp.data.map_or_else(Default::default, |d| (d.tasks, d.hideout_stations, d.maps));
    let tasks: Vec<RawTask> = tasks.into_iter().flatten().collect();
    let mut quests: Vec<Quest> = Vec::new();
    for q in tasks.iter().filter_map(quest) {
        if quests.len() < MAX_QUESTS && quests.iter().all(|x| x.id != q.id) {
            quests.push(q);
        }
    }
    if quests.is_empty() {
        return Err("the answer contains no quests".into());
    }
    let traders = traders(&tasks, &quests);
    Ok(QuestDb {
        mode: mode.to_owned(),
        lang: lang.to_owned(),
        fetched_at,
        traders,
        quests,
        hideout: super::hideout::stations(&stations.unwrap_or_default()),
        bosses: super::bosses::bosses(&maps.unwrap_or_default()),
    })
}

#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
