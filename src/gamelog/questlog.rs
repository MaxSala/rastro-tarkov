//! Quest progress from the game's own logs: the trader messages in `push-notifications_*.log`.
//!
//! A quest changing state is a trader message whose `templateId` is `<quest id> <kind>`:
//! `description` (accepted, text "quest started"), `successMessageText` (handed in; sometimes
//! followed by a trader id and an index) or `failMessageText` (failed). The message spans many
//! lines of pretty-printed JSON, so the reader is a small state machine: it remembers the header
//! line's local time and the message's `dt` (server unix seconds) until the `templateId` line.
//! Format learned from tarkov-map-companion by Casey Weed (MIT), checked there against real logs.
//!
//! One account has a PvP and a PvE character with separate quests, and a message doesn't say
//! which one it was for. [`ModeTracker`] follows the `Session mode:` and profile-load lines of the
//! application log to tell. Profile ids are only ever kept as a hash with a key that changes on
//! every start, in memory: never stored, logged or sent anywhere.

use super::GameMode;
use crate::quests::progress::Status;
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::OnceLock;

/// One thing the game said about one quest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestLogEvent {
    pub quest: String,
    pub status: Status,
    /// Unix seconds from the message (`dt`), the game's own ordering.
    pub at: u64,
    /// Local time of the log line (`AAAA-MM-DDTHH:MM:SS.mmm`), to match it with the mode.
    pub local: Option<String>,
}

struct Rules {
    header: Regex,
    dt: Regex,
    template: Regex,
    profile: Regex,
}

fn rules() -> Option<&'static Rules> {
    static R: OnceLock<Option<Rules>> = OnceLock::new();
    R.get_or_init(|| {
        Some(Rules {
            header: Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}:\d{2}\.\d{3})\|").ok()?,
            dt: Regex::new(r#""dt"\s*:\s*(\d{1,12})"#).ok()?,
            template: Regex::new(
                r#""templateId"\s*:\s*"([0-9a-fA-F]{24})\s+(description|successMessageText|failMessageText)\b"#,
            )
            .ok()?,
            profile: Regex::new(r"SelectedProfileLocally ProfileId:([0-9a-fA-F]{24})\b").ok()?,
        })
    })
    .as_ref()
}

/// Local time of a log line's header, if it has one.
pub fn line_time(line: &str) -> Option<String> {
    let c = rules()?.header.captures(line)?;
    Some(format!("{}T{}", c.get(1)?.as_str(), c.get(2)?.as_str()))
}

/// Reads the notification log line by line, continuation lines included.
#[derive(Debug, Default)]
pub struct NotificationReader {
    local: Option<String>,
    dt: Option<u64>,
}

impl NotificationReader {
    pub fn line(&mut self, line: &str) -> Option<QuestLogEvent> {
        let r = rules()?;
        if let Some(t) = line_time(line) {
            // A new notification: whatever the previous one said no longer applies.
            self.local = Some(t);
            self.dt = None;
            return None;
        }
        if let Some(c) = r.dt.captures(line) {
            self.dt = c.get(1).and_then(|m| m.as_str().parse().ok());
            return None;
        }
        let c = r.template.captures(line)?;
        let status = match c.get(2)?.as_str() {
            "description" => Status::Active,
            "successMessageText" => Status::Completed,
            _ => Status::Failed,
        };
        let at = self.dt.unwrap_or_else(crate::quests::progress::now);
        Some(QuestLogEvent { quest: c.get(1)?.as_str().to_ascii_lowercase(), status, at, local: self.local.clone() })
    }
}

/// Which game mode is loaded, from application log lines.
#[derive(Debug)]
pub struct ModeTracker {
    key: RandomState,
    current: Option<GameMode>,
    /// A mode line came after the last profile load: the next profile is in that mode.
    fresh_mode: bool,
    /// Profile hash → mode. The game writes the mode first and then loads the character.
    known: HashMap<u64, GameMode>,
}

impl Default for ModeTracker {
    fn default() -> Self {
        Self { key: RandomState::new(), current: None, fresh_mode: false, known: HashMap::new() }
    }
}

impl ModeTracker {
    pub fn current(&self) -> Option<GameMode> {
        self.current
    }

    /// Feeds one application line. Returns true when the mode changed.
    pub fn application_line(&mut self, line: &str) -> bool {
        let before = self.current;
        if let Some(mode) = line.split("|Session mode: ").nth(1) {
            match mode.trim().to_ascii_lowercase().as_str() {
                "pve" => self.current = Some(GameMode::Pve),
                "regular" | "pvp" => self.current = Some(GameMode::Regular),
                _ => {}
            }
            self.fresh_mode = self.current.is_some();
        } else if let Some(c) = rules().and_then(|r| r.profile.captures(line)) {
            let Some(hash) = c.get(1).map(|m| self.key.hash_one(m.as_str().to_ascii_lowercase())) else {
                return false;
            };
            match (self.fresh_mode, self.known.get(&hash), self.current) {
                // Just chosen: this character plays in that mode.
                (true, _, Some(m)) => {
                    self.known.insert(hash, m);
                }
                // Switching characters without a mode line: the character says which mode it is.
                (false, Some(m), _) => self.current = Some(*m),
                (false, None, Some(m)) => {
                    self.known.insert(hash, m);
                }
                _ => {}
            }
            self.fresh_mode = false;
        }
        self.current != before
    }
}

/// Modes over time within one launch, for reading old logs: `(local time, mode)` in order.
#[derive(Debug, Default)]
pub struct Timeline(Vec<(String, GameMode)>);

impl Timeline {
    pub fn push(&mut self, at: String, mode: GameMode) {
        if self.0.last().is_none_or(|(_, m)| *m != mode) {
            self.0.push((at, mode));
        }
    }

    /// The mode at a moment; before the first change, the first one known.
    pub fn at(&self, when: Option<&str>) -> Option<GameMode> {
        let when = when?;
        self.0.iter().rev().find(|(t, _)| t.as_str() <= when).or(self.0.first()).map(|(_, m)| *m)
    }

    pub fn last(&self) -> Option<GameMode> {
        self.0.last().map(|(_, m)| *m)
    }
}

/// Last word wins: the latest event per quest.
pub fn fold(events: impl IntoIterator<Item = QuestLogEvent>) -> Vec<QuestLogEvent> {
    let mut latest: HashMap<String, QuestLogEvent> = HashMap::new();
    for e in events {
        if latest.get(&e.quest).is_none_or(|l| l.at <= e.at) {
            latest.insert(e.quest.clone(), e);
        }
    }
    let mut out: Vec<QuestLogEvent> = latest.into_values().collect();
    out.sort_by(|a, b| a.at.cmp(&b.at).then_with(|| a.quest.cmp(&b.quest)));
    out
}

#[cfg(test)]
#[path = "questlog_tests.rs"]
mod tests;
