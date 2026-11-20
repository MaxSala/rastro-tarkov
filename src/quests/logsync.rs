//! Quest progress kept in step with the game: what the logs say was accepted, handed in or failed
//! (live, and the whole history once per start), with what that implies. Accepting or finishing a
//! quest means every quest it requires was completed, and that you're at least its level.
//! Whatever happened later wins, so a change made by hand afterwards is never overwritten.

use super::actions::chain_before;
use super::progress::{Event, Status};
use super::QuestDb;
use crate::gamelog::questlog::{fold, QuestLogEvent};
use crate::gamelog::GameMode;
use crate::party::lock;
use crate::state::App;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

/// What the logs have said so far, per mode, already folded (latest per quest).
#[derive(Debug, Default)]
pub struct LogSync {
    regular: Vec<QuestLogEvent>,
    pve: Vec<QuestLogEvent>,
    pub scanning: bool,
    pub scanned: bool,
    pub sessions: usize,
}

impl LogSync {
    fn known_mut(&mut self, mode: GameMode) -> &mut Vec<QuestLogEvent> {
        match mode {
            GameMode::Regular => &mut self.regular,
            GameMode::Pve => &mut self.pve,
        }
    }

    pub fn known(&self, mode: GameMode) -> &[QuestLogEvent] {
        match mode {
            GameMode::Regular => &self.regular,
            GameMode::Pve => &self.pve,
        }
    }

    fn merge(&mut self, mode: GameMode, events: Vec<QuestLogEvent>) {
        let all = self.known_mut(mode);
        let merged = fold(all.drain(..).chain(events));
        *all = merged;
    }
}

/// For the snapshot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSyncView {
    /// Reading the game logs is on.
    pub enabled: bool,
    pub scanning: bool,
    /// Quests the logs know about in the current mode.
    pub known: usize,
    pub sessions: usize,
}

/// What the known events mean for the progress: each state with its time, plus everything they
/// imply when the quest data is there, and the lowest level that fits.
pub fn plan(db: Option<&QuestDb>, known: &[QuestLogEvent]) -> (Vec<(String, Status, u64)>, u8) {
    let mut out: HashMap<String, (Status, u64)> = HashMap::new();
    let mut level = 1;
    for e in known {
        out.insert(e.quest.clone(), (e.status, e.at));
    }
    if let Some(db) = db {
        for e in known.iter().filter(|e| e.status != Status::Failed) {
            if let Some(q) = db.quests.iter().find(|q| q.id == e.quest) {
                level = level.max(q.level);
            }
            for before in chain_before(db, &e.quest) {
                let later = out.get(&before).is_some_and(|(_, at)| *at >= e.at);
                if !later {
                    out.insert(before, (Status::Completed, e.at));
                }
            }
        }
    }
    let mut list: Vec<(String, Status, u64)> = out.into_iter().map(|(q, (s, at))| (q, s, at)).collect();
    list.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));
    (list, level)
}

impl App {
    /// New trader messages from the live watcher. `mode`: the one loaded, when known.
    pub fn quest_log_events(&self, mode: Option<GameMode>, events: Vec<QuestLogEvent>) {
        let mode = mode.unwrap_or_else(|| self.effective_mode());
        lock(&self.quests).logs.merge(mode, events);
        self.apply_quest_logs();
    }

    /// Reads every launch the game kept, once, in the background.
    pub fn scan_quest_history(self: &Arc<Self>, dir: std::path::PathBuf) {
        {
            let mut q = lock(&self.quests);
            if q.logs.scanning {
                return;
            }
            q.logs.scanning = true;
        }
        self.notify();
        let app = self.clone();
        tokio::task::spawn_blocking(move || {
            let history = crate::gamelog::history::scan(&dir);
            let fallback = app.effective_mode();
            let found = history.events.len();
            {
                let mut q = lock(&app.quests);
                let mut by_mode: [(GameMode, Vec<QuestLogEvent>); 2] =
                    [(GameMode::Regular, Vec::new()), (GameMode::Pve, Vec::new())];
                for (mode, e) in history.events {
                    let m = mode.unwrap_or(fallback);
                    if let Some((_, list)) = by_mode.iter_mut().find(|(k, _)| *k == m) {
                        list.push(e);
                    }
                }
                for (mode, list) in by_mode {
                    q.logs.merge(mode, list);
                }
                q.logs.scanning = false;
                q.logs.scanned = true;
                q.logs.sessions = history.sessions;
            }
            tracing::info!("Quests: {found} quest state(s) found in {} game log session(s)", history.sessions);
            app.apply_quest_logs();
            app.notify();
        });
    }

    /// Brings the progress in line with what the logs know (both modes). Cheap when nothing moved.
    pub fn apply_quest_logs(&self) {
        let db = self.quest_db();
        let mut q = lock(&self.quests);
        let mut changed = false;
        for mode in [GameMode::Regular, GameMode::Pve] {
            if q.logs.known(mode).is_empty() {
                continue;
            }
            // The quest data is per mode; only use it for the mode it was downloaded for.
            let db = db.as_deref().filter(|d| d.mode == mode.api_name());
            let (list, level) = plan(db, q.logs.known(mode));
            changed |= q.progress.apply(mode, Event::FromLogs(list)).unwrap_or(false);
            changed |= q.progress.apply(mode, Event::RaiseLevel(level)).unwrap_or(false);
        }
        if changed {
            if let Err(e) = q.progress.save() {
                tracing::warn!("Quests: could not save the progress: {e}");
            }
        }
        drop(q);
        if changed {
            self.notify();
        }
    }

    pub fn quest_log_view(&self) -> LogSyncView {
        let enabled = lock(&self.inner).config.read_game_logs;
        let mode = self.effective_mode();
        let q = lock(&self.quests);
        LogSyncView { enabled, scanning: q.logs.scanning, known: q.logs.known(mode).len(), sessions: q.logs.sessions }
    }
}

#[cfg(test)]
#[path = "logsync_tests.rs"]
mod tests;
