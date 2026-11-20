//! Vigilancia por sondeo (1 s) de la sesión de logs más reciente. Lee solo lo nuevo, con topes.

use super::parse::{parse_line, timestamp, Action, GameState};
use super::questlog::{ModeTracker, NotificationReader, QuestLogEvent};
use crate::party::lock;
use crate::state::App;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const INITIAL_TAIL: u64 = 4 * 1024 * 1024;
const MAX_READ_PER_TICK: u64 = 1024 * 1024;
const TICK: Duration = Duration::from_secs(1);

struct Tail {
    path: PathBuf,
    offset: u64,
    partial: String,
    /// Trader messages (`push-notifications`): read with their continuation lines.
    notes: Option<NotificationReader>,
}

impl Tail {
    /// `catch_up`: archivo que ya existía al arrancar; se leen solo los últimos 4 MB.
    fn open(path: PathBuf, catch_up: bool) -> Self {
        let len = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let offset = if catch_up { len.saturating_sub(INITIAL_TAIL) } else { 0 };
        let is_notes = path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains(" push-notifications_"));
        let notes = is_notes.then(NotificationReader::default);
        let mut t = Self { path, offset, partial: String::new(), notes };
        if offset > 0 {
            // Se empieza a mitad de línea: descartar hasta el primer salto.
            t.partial = "\u{0}".into();
        }
        t
    }

    fn read_lines(&mut self, out: &mut Vec<String>) {
        let Ok(mut f) = File::open(&self.path) else { return };
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        if len < self.offset {
            self.offset = 0; // archivo truncado o reemplazado
            self.partial.clear();
        }
        let want = (len - self.offset).min(MAX_READ_PER_TICK);
        if want == 0 || f.seek(SeekFrom::Start(self.offset)).is_err() {
            return;
        }
        let mut buf = Vec::with_capacity(usize::try_from(want).unwrap_or(0));
        if f.take(want).read_to_end(&mut buf).is_err() {
            return;
        }
        self.offset += buf.len() as u64;
        let text = std::mem::take(&mut self.partial) + &String::from_utf8_lossy(&buf);
        let mut lines: Vec<&str> = text.split('\n').collect();
        let rest = lines.pop().unwrap_or_default();
        let skip_first = text.starts_with('\u{0}');
        for (i, l) in lines.into_iter().enumerate() {
            if !(skip_first && i == 0) {
                out.push(l.trim_end_matches('\r').to_owned());
            }
        }
        // Una línea sin terminar se guarda para la siguiente vuelta (con tope).
        self.partial = rest.chars().take(64 * 1024).collect();
    }
}

/// Sesión más reciente (por fecha de modificación), o la propia carpeta si ya es una sesión.
pub fn newest_session(dir: &Path) -> Option<PathBuf> {
    super::history::sessions(dir).pop()
}

/// Lanza el vigilante en un hilo de bloqueo. Para pararlo, poner `stop` a true.
pub fn spawn(app: Arc<App>, dir: PathBuf, stop: Arc<AtomicBool>) {
    tokio::task::spawn_blocking(move || {
        let mut session: Option<PathBuf> = None;
        let mut tails: Vec<Tail> = Vec::new();
        let mut state = GameState::default();
        let mut modes = ModeTracker::default();
        let mut first = true;
        while !stop.load(Ordering::Relaxed) {
            let found = dir.is_dir();
            let newest = if found { newest_session(&dir) } else { None };
            if newest != session {
                // Sesión nueva (el juego se ha reiniciado): estado limpio.
                session = newest.clone();
                tails.clear();
                state = GameState::default();
            }
            if let Some(s) = &session {
                for p in super::history::files(s, "") {
                    if !tails.iter().any(|t| t.path == p) {
                        tails.push(Tail::open(p, first));
                    }
                }
            }
            let mut lines = Vec::new();
            let mut quests: Vec<QuestLogEvent> = Vec::new();
            for t in &mut tails {
                if t.notes.is_some() {
                    let mut raw = Vec::new();
                    t.read_lines(&mut raw);
                    if let Some(reader) = t.notes.as_mut() {
                        quests.extend(raw.iter().filter_map(|l| reader.line(l)));
                    }
                } else {
                    t.read_lines(&mut lines);
                }
            }
            // Ordenar por marca de tiempo mezcla bien application y backend dentro de la vuelta.
            let mut stamped: Vec<(String, String)> =
                lines.into_iter().filter_map(|l| Some((timestamp(&l)?, l))).collect();
            stamped.sort_by(|a, b| a.0.cmp(&b.0));
            let mut enter = None;
            let mut changed = false;
            for (_, line) in stamped {
                modes.application_line(&line);
                match parse_line(&line).and_then(|ev| state.apply(ev)) {
                    Some(Action::EnterMap(loc)) => enter = Some((loc, true)),
                    Some(Action::Changed) => changed = true,
                    None => {}
                }
            }
            // Al arrancar a mitad de raid solo se sincroniza el mapa (sin reiniciar nada).
            if first && state.phase != super::parse::Phase::Menu {
                enter = state.location.clone().map(|l| (l, false));
            }
            first = false;
            app.game_tick(state.clone(), found, enter, changed);
            if !quests.is_empty() {
                app.quest_log_events(modes.current().or(state.mode), quests);
            }
            std::thread::sleep(TICK);
        }
        lock(&app.game).stop = None;
    });
}

#[cfg(test)]
#[path = "watcher_tests.rs"]
mod tests;
