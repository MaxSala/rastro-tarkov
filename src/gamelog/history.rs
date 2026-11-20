//! Reads every launch the game has kept in its Logs folder to rebuild quest progress: what was
//! accepted, handed in or failed, and in which mode. Local files only, read once in the
//! background; nothing from them is stored except the resulting quest states.

use super::questlog::{fold, line_time, ModeTracker, NotificationReader, QuestLogEvent, Timeline};
use super::GameMode;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Launches read at most, newest kept.
const MAX_SESSIONS: usize = 3000;
/// Bytes read per file at most. Notification logs are small; application logs can be large, and
/// only two kinds of line matter in them.
const MAX_FILE: u64 = 256 << 20;
const MAX_LINE: usize = 64 * 1024;

#[derive(Debug, Default)]
pub struct History {
    /// Latest state per quest and mode (`None`: no mode line in the logs at all).
    pub events: Vec<(Option<GameMode>, QuestLogEvent)>,
    pub sessions: usize,
}

fn modified(p: &Path) -> SystemTime {
    std::fs::metadata(p).and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH)
}

/// Launch folders, oldest first. By modification time: the folder names carry an unpadded hour,
/// so sorted as text a 9 am launch would come after a 7 pm one.
pub fn sessions(dir: &Path) -> Vec<PathBuf> {
    if !files(dir, "").is_empty() {
        return vec![dir.to_path_buf()];
    }
    let mut out: Vec<(SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_name().to_str().is_some_and(|n| n.starts_with("log_")) && e.path().is_dir())
        .map(|e| (modified(&e.path()), e.path()))
        .collect();
    out.sort();
    let skip = out.len().saturating_sub(MAX_SESSIONS);
    out.into_iter().skip(skip).map(|(_, p)| p).collect()
}

/// `… <kind>_NNN.log` files of one launch, in order (`""`: any of the three Rastro reads).
pub fn files(session: &Path, kind: &str) -> Vec<PathBuf> {
    let wanted = |name: &str| {
        name.ends_with(".log")
            && if kind.is_empty() {
                [" application_", " backend_", " push-notifications_"].iter().any(|k| name.contains(k))
            } else {
                name.contains(&format!(" {kind}_"))
            }
    };
    let mut out: Vec<PathBuf> = std::fs::read_dir(session)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.file_name().and_then(|n| n.to_str()).is_some_and(wanted))
        .collect();
    out.sort();
    out
}

/// Calls `f` for each line of a file, within the limits. Unreadable files are skipped.
fn each_line(path: &Path, mut f: impl FnMut(&str)) {
    let Ok(file) = File::open(path) else { return };
    let mut reader = BufReader::new(file.take(MAX_FILE));
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match reader.by_ref().take(MAX_LINE as u64).read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => break,
            Ok(_) => f(String::from_utf8_lossy(&buf).trim_end()),
        }
        // A line longer than the limit: drop the rest of it.
        if buf.len() >= MAX_LINE && buf.last() != Some(&b'\n') {
            let mut rest = Vec::new();
            if reader.read_until(b'\n', &mut rest).is_err() {
                break;
            }
        }
    }
}

/// The whole history, folded to the latest state per quest and mode.
pub fn scan(dir: &Path) -> History {
    let mut tracker = ModeTracker::default();
    let mut all: Vec<(Option<GameMode>, QuestLogEvent)> = Vec::new();
    let list = sessions(dir);
    for session in &list {
        let mut events = Vec::new();
        for f in files(session, "push-notifications") {
            let mut reader = NotificationReader::default();
            each_line(&f, |l| events.extend(reader.line(l)));
        }
        // The (large) application log is only read when there's something to attribute, or to
        // keep following a character switch into the next launch.
        let mut timeline = Timeline::default();
        let carried = tracker.current();
        if !events.is_empty() || carried.is_none() {
            for f in files(session, "application") {
                each_line(&f, |l| {
                    if (l.contains("Session mode:") || l.contains("SelectedProfileLocally"))
                        && tracker.application_line(l)
                    {
                        if let (Some(t), Some(m)) = (line_time(l), tracker.current()) {
                            timeline.push(t, m);
                        }
                    }
                });
            }
        }
        for e in events {
            let mode = timeline.at(e.local.as_deref()).or(carried).or(timeline.last());
            all.push((mode, e));
        }
    }
    let mut out = Vec::new();
    for mode in [None, Some(GameMode::Regular), Some(GameMode::Pve)] {
        let mine = all.iter().filter(|(m, _)| *m == mode).map(|(_, e)| e.clone());
        out.extend(fold(mine).into_iter().map(|e| (mode, e)));
    }
    History { events: out, sessions: list.len() }
}

#[cfg(test)]
#[path = "history_tests.rs"]
mod tests;
