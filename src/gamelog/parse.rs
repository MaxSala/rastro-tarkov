//! Lectura de líneas de los logs del juego. Función pura: texto → evento. Nada se guarda en
//! bruto; solo se extraen el modo, el nombre interno del mapa y las marcas de tiempo.
//!
//! Formato real (EFT 1.3, `application_000.log` y `backend_000.log`):
//! `2026-11-14 15:49:14.188|1.3.0.2.48779|Info|application|GameStarted:44.08(0) real:60.29(0) diff:16.2`
//! Los mensajes pueden ocupar varias líneas; las de continuación no empiezan por fecha y se ignoran.

use super::GameMode;
use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogEvent {
    Mode(GameMode),
    /// Empieza a cargar un mapa (nombre interno del juego, p. ej. `laboratory`).
    MapLoading(String),
    /// La raid empieza; marca de tiempo local `AAAA-MM-DDTHH:MM:SS.mmm`.
    RaidStarted(String),
    /// Fin de raid explícito (PvE: petición `/client/match/local/end`).
    RaidEnded,
    /// El perfil vuelve a cargarse en el menú (pasa al terminar cada raid, en cualquier modo).
    ProfileReady,
}

struct Rules {
    head: Regex,
    preset: Regex,
    transit: Regex,
}

fn rules() -> Option<&'static Rules> {
    static R: OnceLock<Option<Rules>> = OnceLock::new();
    R.get_or_init(|| {
        Some(Rules {
            // fecha y hora | versión | nivel | categoría | mensaje
            head: Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}:\d{2}\.\d{3})\|[^|]*\|[^|]*\|([a-z-]+)\|(.*)$")
                .ok()?,
            // TODO: suposición — en otros mapas el bundle sigue el patrón maps/<nombre>_preset.bundle.
            preset: Regex::new(r"scene preset path:maps/([A-Za-z0-9_]{1,40})_preset\.bundle").ok()?,
            // «Locations:a -> b -> »: lista de mapas de la cadena de transiciones; el último es el actual.
            transit: Regex::new(r"^\[Transit\] .*Locations:(.*)$").ok()?,
        })
    })
    .as_ref()
}

/// Nombre interno de mapa aceptable (evita arrastrar texto arbitrario del log).
pub fn valid_location(s: &str) -> bool {
    (1..=40).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Marca de tiempo de la cabecera, si la línea es una entrada (no una continuación).
pub fn timestamp(line: &str) -> Option<String> {
    let c = rules()?.head.captures(line)?;
    Some(format!("{}T{}", c.get(1)?.as_str(), c.get(2)?.as_str()))
}

pub fn parse_line(line: &str) -> Option<LogEvent> {
    let r = rules()?;
    let c = r.head.captures(line.trim_end())?;
    let (category, msg) = (c.get(3)?.as_str(), c.get(4)?.as_str());
    match category {
        "application" => parse_application(r, msg, || Some(format!("{}T{}", c.get(1)?.as_str(), c.get(2)?.as_str()))),
        // Solo la línea de petición: la respuesta repite la URL y duplicaría el evento.
        "backend" if msg.starts_with("---> Request") && msg.contains("/client/match/local/end") => {
            Some(LogEvent::RaidEnded)
        }
        _ => None,
    }
}

fn parse_application(r: &Rules, msg: &str, ts: impl Fn() -> Option<String>) -> Option<LogEvent> {
    if let Some(mode) = msg.strip_prefix("Session mode: ") {
        return match mode.trim().to_ascii_lowercase().as_str() {
            "pve" => Some(LogEvent::Mode(GameMode::Pve)),
            // TODO: suposición — en PvP el valor es «Regular» (solo se ha visto «Pve» en logs reales).
            "regular" | "pvp" => Some(LogEvent::Mode(GameMode::Regular)),
            _ => None,
        };
    }
    if msg.starts_with("GameStarted:") {
        return ts().map(LogEvent::RaidStarted);
    }
    if msg.starts_with("PrepareSelectedProfileLocally ") {
        return Some(LogEvent::ProfileReady);
    }
    if let Some(c) = r.preset.captures(msg) {
        return c.get(1).map(|m| LogEvent::MapLoading(m.as_str().to_owned()));
    }
    if let Some(c) = r.transit.captures(msg) {
        let list = c.get(1)?.as_str();
        let current = list.split("->").map(str::trim).filter(|s| !s.is_empty()).last()?;
        return valid_location(current).then(|| LogEvent::MapLoading(current.to_owned()));
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    #[default]
    Menu,
    Loading,
    Raid,
}

/// Estado del juego deducido de los logs.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameState {
    pub mode: Option<GameMode>,
    pub phase: Phase,
    pub location: Option<String>,
    pub started_at: Option<String>,
}

/// Lo que la app debe hacer ante un evento en directo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Cambiar al mapa (nombre interno) y empezar una raid nueva.
    EnterMap(String),
    Changed,
}

impl GameState {
    pub fn apply(&mut self, ev: LogEvent) -> Option<Action> {
        match ev {
            LogEvent::Mode(m) => {
                self.mode = Some(m);
                Some(Action::Changed)
            }
            LogEvent::MapLoading(loc) => {
                let loc = loc.to_ascii_lowercase();
                // El preset y la línea [Transit] llegan seguidos: actuar solo una vez por carga.
                let repeated = self.phase == Phase::Loading && self.location.as_deref() == Some(loc.as_str());
                self.phase = Phase::Loading;
                self.location = Some(loc.clone());
                self.started_at = None;
                (!repeated).then_some(Action::EnterMap(loc))
            }
            LogEvent::RaidStarted(ts) => {
                // Sin mapa conocido no hay raid que cronometrar.
                self.location.as_ref()?;
                self.phase = Phase::Raid;
                self.started_at = Some(ts);
                Some(Action::Changed)
            }
            LogEvent::RaidEnded | LogEvent::ProfileReady => {
                if self.phase == Phase::Menu {
                    return None;
                }
                self.phase = Phase::Menu;
                self.started_at = None;
                Some(Action::Changed)
            }
        }
    }
}

#[cfg(test)]
#[path = "parse_tests.rs"]
mod tests;
