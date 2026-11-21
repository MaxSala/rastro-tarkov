//! Mensajes del WebSocket /ws entre backend y web. Espejo en web/src/lib/protocol.ts.

use crate::party::Interface;
use serde::{Deserialize, Serialize};

/// Web → backend. El primer mensaje debe ser `auth`.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ClientMsg {
    Auth {
        token: String,
    },
    SetActiveMap {
        map_id: String,
    },
    NewRaid,
    SelectExtract {
        extract_id: Option<String>,
    },
    ToggleExtract {
        extract_id: String,
    },
    UpdateSettings {
        settings: SettingsInput,
    },
    AckShareWarning,
    PartyHost {
        bind_ip: String,
        advertise: String,
    },
    PartyJoin {
        code: String,
    },
    PartyLeave,
    PartyKick {
        peer_id: u8,
    },
    MapPing {
        px: f64,
        py: f64,
    },
    ListInterfaces,
    RemovePack {
        pack_id: String,
    },
    TarkovDevSync,
    FinishOnboarding,
    CheckFolder {
        path: String,
    },
    SetScreenshotsDir {
        path: String,
    },
    AddPoint {
        px: f64,
        py: f64,
        name: String,
        kind: crate::points::PointKind,
        note: String,
    },
    UpdatePoint {
        id: String,
        name: String,
        kind: crate::points::PointKind,
        note: String,
    },
    DeletePoint {
        id: String,
    },
    GetLogs,
    OpenDataDir,
    SetOpenInWindow {
        value: bool,
    },
    Quit,
    SetGameLogs {
        enabled: bool,
        /// Logs folder; empty means the default one. Left out, the current folder is kept.
        #[serde(default)]
        path: Option<String>,
    },
    SetGameMode {
        mode: crate::gamelog::GameModePref,
    },
    /// `flea_name`: nombre del mercadillo en el idioma de la interfaz.
    MarketRefresh {
        flea_name: String,
    },
    SetLanguage {
        lang: crate::config::Language,
    },
    /// Bring maps, prices and quests downloaded in another language over to the current one.
    UpdateLanguageData {
        flea_name: String,
    },
    /// Do that on its own after every language change.
    SetAutoLanguageData {
        enabled: bool,
    },
    /// The player's answer to "check for updates on startup?".
    SetCheckUpdates {
        enabled: bool,
    },
    /// "Check now" in Settings → About.
    CheckUpdates,
    /// Download the quests from tarkov.dev (only on request).
    QuestsRefresh,
    /// A quest's state; `null` takes it back to "not started".
    QuestSet {
        quest_id: String,
        status: Option<crate::quests::progress::Status>,
    },
    /// How many times an objective has been done (0 clears it).
    QuestObjective {
        objective_id: String,
        count: u32,
    },
    QuestsLevel {
        level: u8,
    },
    QuestsFaction {
        faction: crate::quests::Faction,
    },
    /// Level built of a hideout station (0: not built).
    HideoutLevel {
        station_id: String,
        level: u8,
    },
    /// Marks every quest leading to this one as completed.
    QuestsCompleteBefore {
        quest_id: String,
    },
    /// Clears the progress of the current game mode.
    QuestsReset,
    /// Read the quest history in the game logs again.
    QuestsReadLogs,
    /// Catching up: these quests, and everything before each of them, are done (max 32).
    QuestsCompleteThrough {
        quest_ids: Vec<String>,
    },
    /// Previews a quest's objectives on the map; `null` stops.
    QuestFocus {
        quest_id: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SettingsInput {
    /// Vacío = carpeta por defecto del juego.
    pub screenshots_dir: String,
    pub trail_length: usize,
    pub player_name: String,
    pub player_color: u8,
    pub party_port: u16,
}

/// Backend → web.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ServerMsg {
    Snapshot {
        view: Box<crate::view::View>,
    },
    Interfaces {
        list: Vec<Interface>,
    },
    /// `msg` is flattened: `{ "type": "notice", "level", "code", "arg"?, "text" }`.
    Notice {
        level: NoticeLevel,
        #[serde(flatten)]
        msg: crate::msg::UserMsg,
    },
    FolderCheck {
        result: crate::setup::FolderCheck,
    },
    Logs {
        lines: Vec<String>,
    },
    MapData {
        map_id: String,
        markers: Vec<crate::view_extra::MarkerView>,
        icons: Vec<String>,
    },
    /// Quest progress of the current game mode; sent again whenever it changes.
    QuestProgress {
        progress: crate::quests::view::ProgressView,
    },
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NoticeLevel {
    Info,
    Error,
}
