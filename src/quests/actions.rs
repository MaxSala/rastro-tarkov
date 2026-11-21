//! Quest actions from the interface. Each one becomes a progress event with source `manual`.
//! The game mode is read before taking the quests lock (it needs `inner`), never inside it.

use super::progress::{Event, Source, Status};
use super::{Faction, QuestDb};
use crate::error::{AppError, AppResult};
use crate::party::lock;
use crate::state::App;
use std::collections::HashSet;

/// Every quest that has to be completed before `id`, following requirements back to the start.
/// Only requirements asking for `complete` count; a quest that must be failed or merely active
/// is left alone.
pub fn chain_before(db: &QuestDb, id: &str) -> Vec<String> {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec![id];
    let mut out = Vec::new();
    while let Some(cur) = stack.pop() {
        let Some(q) = db.quests.iter().find(|q| q.id == cur) else { continue };
        for r in q.requires.iter().filter(|r| r.status.iter().any(|s| s == "complete")) {
            if r.quest != id && seen.insert(r.quest.as_str()) && out.len() < 3000 {
                out.push(r.quest.clone());
                stack.push(r.quest.as_str());
            }
        }
    }
    out
}

impl App {
    fn quest_event(&self, event: Event) -> AppResult<()> {
        let mode = self.effective_mode();
        let mut q = lock(&self.quests);
        let changed = q.progress.apply(mode, event).map_err(AppError::bad)?;
        if changed {
            q.progress.save()?;
        }
        drop(q);
        if changed {
            self.notify();
        }
        Ok(())
    }

    pub fn quest_set(&self, quest: String, status: Option<Status>) -> AppResult<()> {
        self.quest_event(Event::SetStatus { quest, status, source: Source::Manual })
    }

    pub fn quest_objective(&self, objective: String, count: u32) -> AppResult<()> {
        self.quest_event(Event::SetObjective { objective, count })
    }

    pub fn quests_level(&self, level: u8) -> AppResult<()> {
        self.quest_event(Event::SetLevel(level))
    }

    pub fn quests_faction(&self, faction: Faction) -> AppResult<()> {
        self.quest_event(Event::SetFaction(faction))
    }

    /// Level built of a hideout station (0: not built).
    pub fn hideout_level(&self, station: String, level: u8) -> AppResult<()> {
        self.quest_event(Event::SetStation { station: station.to_ascii_lowercase(), level })
    }

    /// Marks everything that leads to `quest` as completed (for players who start using Rastro
    /// halfway through a wipe).
    pub fn quests_complete_before(&self, quest: &str) -> AppResult<()> {
        let db = self.quest_db().ok_or(AppError::NotFound)?;
        if !db.quests.iter().any(|q| q.id == quest) {
            return Err(AppError::NotFound);
        }
        let list = chain_before(&db, quest);
        if list.is_empty() {
            return Ok(());
        }
        self.quest_event(Event::CompleteAll(list))
    }

    /// Catch-up from the interface: each quest picked, and everything before it, is completed.
    pub fn quests_complete_through(&self, quests: &[String]) -> AppResult<()> {
        if quests.len() > 32 {
            return Err(AppError::bad("too many quests"));
        }
        let db = self.quest_db().ok_or(AppError::NotFound)?;
        let mut list: Vec<String> = Vec::new();
        for id in quests {
            if !db.quests.iter().any(|q| &q.id == id) {
                return Err(AppError::NotFound);
            }
            for q in chain_before(&db, id).into_iter().chain([id.clone()]) {
                if !list.contains(&q) {
                    list.push(q);
                }
            }
        }
        if list.is_empty() {
            return Ok(());
        }
        self.quest_event(Event::CompleteAll(list))
    }

    pub fn quests_reset(&self) -> AppResult<()> {
        self.quest_event(Event::Reset)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Quest, Requirement};
    use super::*;

    fn quest(id: &str, requires: &[(&str, &str)]) -> Quest {
        Quest {
            id: id.into(),
            name: id.into(),
            slug: String::new(),
            trader: "prapor".into(),
            map: None,
            level: 1,
            kappa: false,
            lightkeeper: false,
            faction: Faction::Any,
            wiki: None,
            requires: requires
                .iter()
                .map(|(q, s)| Requirement { quest: (*q).into(), status: vec![(*s).into()] })
                .collect(),
            loyalty: vec![],
            objectives: vec![],
            rewards: Default::default(),
            image: false,
        }
    }

    #[test]
    fn follows_the_whole_chain_once() {
        let db = QuestDb {
            mode: "regular".into(),
            lang: "en".into(),
            fetched_at: 0,
            traders: vec![],
            hideout: vec![],
            bosses: vec![],
            quests: vec![
                quest("a", &[]),
                quest("b", &[("a", "complete")]),
                quest("c", &[("a", "complete"), ("b", "complete")]),
                quest("d", &[("c", "complete"), ("x", "failed"), ("d", "complete")]),
            ],
        };
        let mut chain = chain_before(&db, "d");
        chain.sort();
        assert_eq!(chain, vec!["a", "b", "c"], "failed requirements and loops are ignored");
        assert!(chain_before(&db, "a").is_empty());
        assert!(chain_before(&db, "missing").is_empty());
    }
}
