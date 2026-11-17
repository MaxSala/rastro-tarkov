//! Detección automática de mapa, raid y modo leyendo los logs del juego (solo lectura).
//! Desactivada por defecto. No se guarda ni se registra nada de los logs salvo el modo y el
//! nombre interno del mapa; los identificadores de perfil y las IPs nunca salen del archivo.

pub mod control;
pub mod parse;
pub mod watcher;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    Regular,
    Pve,
}

impl GameMode {
    /// Ruta de json.tarkov.dev y valor de `gameMode` en GraphQL.
    pub fn api_name(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Pve => "pve",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameModePref {
    /// El que digan los logs; si no hay, PvP.
    #[default]
    Auto,
    Regular,
    Pve,
}

/// Nombre interno del juego → (id del mapa en tarkov.dev, nombre visible).
/// Respaldo para paquetes sin `game_ids`. Solo «laboratory» se ha visto en logs reales;
/// el resto coincide con el `nameId` de tarkov.dev. TODO: verificar con logs de cada mapa.
const LOCATIONS: &[(&str, &str, &str)] = &[
    ("bigmap", "customs", "Customs"),
    ("factory4_day", "factory", "Factory"),
    ("factory4_night", "factory", "Factory"),
    ("woods", "woods", "Woods"),
    ("shoreline", "shoreline", "Shoreline"),
    ("interchange", "interchange", "Interchange"),
    ("rezervbase", "reserve", "Reserve"),
    ("laboratory", "the-lab", "Labs"),
    ("lighthouse", "lighthouse", "Lighthouse"),
    ("tarkovstreets", "streets-of-tarkov", "Streets of Tarkov"),
    ("sandbox", "ground-zero", "Ground Zero"),
    ("sandbox_high", "ground-zero", "Ground Zero"),
    ("labyrinth", "the-labyrinth", "Labyrinth"),
    ("terminal", "terminal", "Terminal"),
];

/// Nombres internos de la tabla que corresponden a un mapa de tarkov.dev.
pub fn table_ids_for(map_id: &str) -> impl Iterator<Item = &'static str> + '_ {
    LOCATIONS.iter().filter(move |(_, id, _)| *id == map_id).map(|(k, _, _)| *k)
}

pub fn known_location(loc: &str) -> Option<(&'static str, &'static str)> {
    let loc = loc.to_ascii_lowercase();
    LOCATIONS.iter().find(|(k, _, _)| *k == loc).map(|(_, id, name)| (*id, *name))
}

/// Carpetas de logs habituales (instalación por defecto del launcher).
pub fn default_logs_dirs() -> Vec<PathBuf> {
    ["C:", "D:"]
        .iter()
        .flat_map(|d| {
            ["Escape from Tarkov", "EFT"].map(|g| PathBuf::from(format!("{d}\\Battlestate Games\\{g}\\Logs")))
        })
        .collect()
}

/// Estado compartido de la detección (lo actualiza el vigilante; lo lee la vista).
#[derive(Debug, Default)]
pub struct GameStatus {
    pub state: parse::GameState,
    pub folder: Option<PathBuf>,
    pub folder_found: bool,
    pub stop: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

/// Lo que ve la web.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameView {
    pub enabled: bool,
    pub folder: Option<String>,
    pub folder_found: bool,
    pub mode: Option<GameMode>,
    pub mode_pref: GameModePref,
    pub phase: Option<parse::Phase>,
    pub map_name: Option<String>,
    pub map_installed: bool,
    /// Hora local de inicio de la raid (`AAAA-MM-DDTHH:MM:SS.mmm`); la web calcula el reloj.
    pub started_at: Option<String>,
    pub raid_minutes: Option<u32>,
}
