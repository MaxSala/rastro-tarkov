//! Quests: tarkov.dev's task data (downloaded only when the player asks) and the player's own
//! progress, kept per game mode on this PC.
//!
//! Progress changes arrive as [`progress::Event`]s with a [`progress::Source`]: the interface and
//! the game logs (see [`logsync`]) feed the same store.

pub mod actions;
pub mod fetch;
pub mod jsondoc;
pub mod logsync;
pub mod objectives;
pub mod progress;
pub mod query;
pub mod raw;
pub mod rewards;
pub mod view;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Everything the interface needs about every quest, already translated.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestDb {
    pub mode: String,
    pub lang: String,
    /// Unix seconds of the download.
    pub fetched_at: u64,
    pub traders: Vec<TraderRef>,
    pub quests: Vec<Quest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TraderRef {
    /// tarkov.dev normalizedName, e.g. `prapor`.
    pub id: String,
    pub name: String,
    /// The trader's 24-hex id, which also names its portrait (`/api/tarkov-image/{id}`).
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Faction {
    #[default]
    Any,
    Usec,
    Bear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub id: String,
    pub name: String,
    /// normalizedName, for the tarkov.dev link.
    pub slug: String,
    pub trader: String,
    /// Map the quest happens on, when it has one (pack id).
    pub map: Option<String>,
    pub level: u8,
    pub kappa: bool,
    pub lightkeeper: bool,
    pub faction: Faction,
    pub wiki: Option<String>,
    pub requires: Vec<Requirement>,
    /// Trader loyalty levels needed, as "trader name, level".
    pub loyalty: Vec<(String, u8)>,
    pub objectives: Vec<Objective>,
    /// What handing it in gives. Missing in caches written by earlier builds.
    #[serde(default)]
    pub rewards: rewards::Rewards,
    /// tarkov.dev has the quest's picture (`/api/tarkov-image/{id}`).
    #[serde(default)]
    pub image: bool,
}

/// Another quest that must be in one of these states first (`complete`, `active`, `failed`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    pub quest: String,
    pub status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub id: String,
    /// tarkov.dev objective type: `visit`, `mark`, `findItem`, `giveItem`, `shoot`…
    pub kind: String,
    pub text: String,
    pub optional: bool,
    /// How many times it has to be done (1 for one-off objectives).
    pub count: u32,
    /// Items must be found in raid.
    pub fir: bool,
    /// Maps it can be done on (pack ids). Empty: anywhere.
    pub maps: Vec<String>,
    pub items: Vec<ItemRef>,
    /// Keys needed: each inner list is a set of alternatives.
    pub keys: Vec<Vec<ItemRef>>,
    pub positions: Vec<Spot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemRef {
    pub id: String,
    pub name: String,
}

/// A place on a map where the objective is done, in game coordinates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spot {
    /// Pack id of the map.
    pub map: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Outline of the area on the ground as (x, z), for zones that have one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outline: Vec<[f64; 2]>,
    #[serde(default)]
    pub kind: SpotKind,
}

/// How precise a spot is, so the map never presents a guess as the place.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpotKind {
    /// A specific place (a small zone: a door, a tank, a room).
    #[default]
    Exact,
    /// A large zone: the objective counts anywhere inside its outline.
    Area,
    /// One of the places an item can spawn: it's in one of them, not in all.
    Candidate,
}

/// tarkov.dev has variants of some maps; Rastro installs one pack for each.
const MAP_ALIASES: &[(&str, &str)] =
    &[("night-factory", "factory"), ("ground-zero-21", "ground-zero"), ("ground-zero-tutorial", "ground-zero")];

pub fn pack_id_for(map: &str) -> &str {
    MAP_ALIASES.iter().find(|(from, _)| *from == map).map_or(map, |(_, to)| to)
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum QuestsState {
    #[default]
    Empty,
    Loading,
    Ready,
    Error,
}

/// Small status for every snapshot; the quest data itself goes over HTTP.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestsStatus {
    pub state: QuestsState,
    pub fetched_at: Option<u64>,
    pub count: usize,
    pub mode: Option<String>,
    pub lang: Option<String>,
    pub error: Option<crate::msg::UserMsg>,
    /// Following the game logs.
    pub logs: logsync::LogSyncView,
}

/// Shared state: download status, the loaded data and the progress store.
pub struct Quests {
    pub status: QuestsStatus,
    pub db: Option<Arc<QuestDb>>,
    pub progress: progress::ProgressStore,
    /// Quest previewed on the map, whatever its state (not saved).
    pub focus: Option<String>,
    /// What the game logs have said (memory only).
    pub logs: logsync::LogSync,
}

impl Quests {
    pub fn load(progress_file: &std::path::Path) -> Self {
        Self {
            status: QuestsStatus::default(),
            db: None,
            progress: progress::ProgressStore::load(progress_file),
            focus: None,
            logs: logsync::LogSync::default(),
        }
    }
}
