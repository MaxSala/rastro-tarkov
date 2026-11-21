//! The player's quest progress, per game mode, in `quests.json`. Changes go through [`Event`], so
//! the interface and the game logs update it the same way.

use super::Faction;
use crate::config::read_limited;
use crate::gamelog::GameMode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_FILE: u64 = 4 << 20;
const MAX_QUESTS: usize = 5000;
const MAX_OBJECTIVES: usize = 20_000;
pub const MAX_LEVEL: u8 = 79;
const MAX_COUNT: u32 = 100_000;
const MAX_STATIONS: usize = 64;
/// Highest hideout level a station can have.
pub const MAX_STATION_LEVEL: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Completed,
    Failed,
}

/// Who changed something: the player in the interface, or the game logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[default]
    Manual,
    Logs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Record {
    pub status: Status,
    /// Unix seconds of the change.
    pub at: u64,
    #[serde(default)]
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ModeProgress {
    pub level: u8,
    pub faction: Faction,
    pub quests: BTreeMap<String, Record>,
    /// How many times each objective has been done. Absent: not started.
    pub objectives: BTreeMap<String, u32>,
    /// Level built of each hideout station, by station id. Absent: not built.
    pub hideout: BTreeMap<String, u8>,
}

impl Default for ModeProgress {
    fn default() -> Self {
        Self {
            level: 1,
            faction: Faction::Any,
            quests: BTreeMap::new(),
            objectives: BTreeMap::new(),
            hideout: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct File {
    regular: ModeProgress,
    pve: ModeProgress,
}

#[derive(Debug, Clone)]
pub enum Event {
    /// `None` clears the quest back to "not started".
    SetStatus {
        quest: String,
        status: Option<Status>,
        source: Source,
    },
    SetObjective {
        objective: String,
        count: u32,
    },
    SetLevel(u8),
    SetFaction(Faction),
    /// Level built of a hideout station; 0 clears it.
    SetStation {
        station: String,
        level: u8,
    },
    /// Several quests completed at once (everything before a quest).
    CompleteAll(Vec<String>),
    /// What the game logs say, with when it happened. Whatever was set later wins, by hand or not.
    FromLogs(Vec<(String, Status, u64)>),
    /// Raises the level (never lowers it): the logs show quests that need at least this one.
    RaiseLevel(u8),
    Reset,
}

pub struct ProgressStore {
    path: PathBuf,
    file: File,
    /// Grows with every change; the server sends the progress again when it moves.
    pub revision: u64,
}

fn valid(id: &str) -> bool {
    crate::market::icons::valid_id(id) && id.bytes().all(|b| !b.is_ascii_uppercase())
}

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn sanitize(p: &mut ModeProgress) {
    p.level = p.level.clamp(1, MAX_LEVEL);
    p.quests.retain(|k, _| valid(k));
    p.objectives.retain(|k, v| valid(k) && *v > 0 && *v <= MAX_COUNT);
    while p.quests.len() > MAX_QUESTS {
        p.quests.pop_last();
    }
    while p.objectives.len() > MAX_OBJECTIVES {
        p.objectives.pop_last();
    }
    p.hideout.retain(|k, v| valid(k) && (1..=MAX_STATION_LEVEL).contains(v));
    while p.hideout.len() > MAX_STATIONS {
        p.hideout.pop_last();
    }
}

impl ProgressStore {
    /// Lenient load: anything invalid is dropped, a broken file starts empty (with a warning).
    pub fn load(path: &Path) -> Self {
        let mut file: File = match read_limited(path, MAX_FILE) {
            Ok(Some(b)) => serde_json::from_slice(&b).unwrap_or_else(|_| {
                tracing::warn!("quests.json is not valid; starting with no quest progress");
                File::default()
            }),
            _ => File::default(),
        };
        sanitize(&mut file.regular);
        sanitize(&mut file.pve);
        Self { path: path.to_path_buf(), file, revision: 1 }
    }

    pub fn get(&self, mode: GameMode) -> &ModeProgress {
        match mode {
            GameMode::Regular => &self.file.regular,
            GameMode::Pve => &self.file.pve,
        }
    }

    fn get_mut(&mut self, mode: GameMode) -> &mut ModeProgress {
        match mode {
            GameMode::Regular => &mut self.file.regular,
            GameMode::Pve => &mut self.file.pve,
        }
    }

    #[cfg(test)]
    pub fn status(&self, mode: GameMode, quest: &str) -> Option<Status> {
        self.get(mode).quests.get(quest).map(|r| r.status)
    }

    /// Applies a change. Returns whether anything changed; ids that aren't 24 hex digits are refused.
    pub fn apply(&mut self, mode: GameMode, event: Event) -> Result<bool, &'static str> {
        let p = self.get_mut(mode);
        let before = p.clone();
        match event {
            Event::SetStatus { quest, status, source } => {
                if !valid(&quest) {
                    return Err("invalid quest id");
                }
                match status {
                    None => {
                        p.quests.remove(&quest);
                    }
                    Some(s) if p.quests.get(&quest).map(|r| r.status) != Some(s) => {
                        if p.quests.len() >= MAX_QUESTS && !p.quests.contains_key(&quest) {
                            return Err("too many quests");
                        }
                        p.quests.insert(quest, Record { status: s, at: now(), source });
                    }
                    Some(_) => {}
                }
            }
            Event::SetObjective { objective, count } => {
                if !valid(&objective) || count > MAX_COUNT {
                    return Err("invalid objective");
                }
                if count == 0 {
                    p.objectives.remove(&objective);
                } else if p.objectives.len() < MAX_OBJECTIVES || p.objectives.contains_key(&objective) {
                    p.objectives.insert(objective, count);
                } else {
                    return Err("too many objectives");
                }
            }
            Event::SetLevel(level) => p.level = level.clamp(1, MAX_LEVEL),
            Event::SetFaction(f) => p.faction = f,
            Event::SetStation { station, level } => {
                if !valid(&station) || level > MAX_STATION_LEVEL {
                    return Err("invalid station");
                }
                if level == 0 {
                    p.hideout.remove(&station);
                } else if p.hideout.len() < MAX_STATIONS || p.hideout.contains_key(&station) {
                    p.hideout.insert(station, level);
                } else {
                    return Err("too many stations");
                }
            }
            Event::CompleteAll(list) => {
                if !list.iter().all(|q| valid(q)) {
                    return Err("invalid quest id");
                }
                let at = now();
                for q in list {
                    if p.quests.len() >= MAX_QUESTS && !p.quests.contains_key(&q) {
                        break;
                    }
                    let done = p.quests.get(&q).is_some_and(|r| r.status == Status::Completed);
                    if !done {
                        p.quests.insert(q, Record { status: Status::Completed, at, source: Source::Manual });
                    }
                }
            }
            Event::FromLogs(list) => {
                for (quest, status, at) in list {
                    if !valid(&quest) {
                        continue;
                    }
                    let newer = p.quests.get(&quest).is_some_and(|r| r.at > at || r.status == status);
                    if newer || (p.quests.len() >= MAX_QUESTS && !p.quests.contains_key(&quest)) {
                        continue;
                    }
                    p.quests.insert(quest, Record { status, at, source: Source::Logs });
                }
            }
            Event::RaiseLevel(level) => p.level = p.level.max(level.clamp(1, MAX_LEVEL)),
            Event::Reset => *p = ModeProgress::default(),
        }
        let changed = *p != before;
        if changed {
            self.revision += 1;
        }
        Ok(changed)
    }

    /// Atomic write (temporary file, then rename).
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let json = serde_json::to_vec_pretty(&self.file).map_err(std::io::Error::other)?;
        let tmp = self.path.with_extension("tmp");
        std::fs::write(&tmp, json)?;
        std::fs::rename(tmp, &self.path)
    }
}

#[cfg(test)]
#[path = "progress_tests.rs"]
mod tests;
