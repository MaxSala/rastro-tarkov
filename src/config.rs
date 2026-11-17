//! Configuración persistente y rutas de datos del usuario.

use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const MAX_CONFIG_BYTES: u64 = 64 * 1024;
pub const PALETTE_SIZE: u8 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct Config {
    /// None = carpeta por defecto del juego en Documentos.
    pub screenshots_dir: Option<PathBuf>,
    pub trail_length: usize,
    pub server_port: u16,
    pub player_name: String,
    pub player_color: u8,
    pub party_port: u16,
    pub share_warning_ack: bool,
    pub last_map: Option<String>,
    /// El asistente de primer arranque ya se completó (o se descartó).
    pub onboarding_done: bool,
    /// true = ventana de escritorio propia; false = navegador del sistema.
    pub open_in_window: bool,
    /// Detección automática leyendo los logs del juego (desactivada por defecto).
    pub read_game_logs: bool,
    /// None = carpeta Logs de la instalación por defecto.
    pub logs_dir: Option<PathBuf>,
    /// Modo de los datos de tarkov.dev (auto = el que digan los logs).
    pub game_mode: crate::gamelog::GameModePref,
    /// Idioma de la interfaz y de los datos descargados.
    pub language: Language,
    /// Check GitHub for a newer release on startup. None = the player hasn't been asked yet.
    pub check_updates: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Idiomas de tarkov.dev (enum `LanguageCode` de su API). La interfaz tiene los mismos.
pub enum Language {
    Cs,
    De,
    En,
    #[default]
    Es,
    Fr,
    Hu,
    It,
    Ja,
    Ko,
    Pl,
    Pt,
    Ro,
    Ru,
    Sk,
    Tr,
    Zh,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::Cs => "cs",
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::Hu => "hu",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::Ro => "ro",
            Self::Ru => "ru",
            Self::Sk => "sk",
            Self::Tr => "tr",
            Self::Zh => "zh",
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screenshots_dir: None,
            trail_length: 200,
            server_port: 47821,
            player_name: "Player".into(),
            player_color: 0,
            party_port: 47822,
            share_warning_ack: false,
            last_map: None,
            onboarding_done: false,
            open_in_window: true,
            read_game_logs: false,
            logs_dir: None,
            game_mode: crate::gamelog::GameModePref::Auto,
            language: Language::Es,
            check_updates: None,
        }
    }
}

impl Config {
    pub fn validate(&self) -> AppResult<()> {
        if !(1..=2000).contains(&self.trail_length) {
            return Err(AppError::bad("the trail must have between 1 and 2000 points"));
        }
        if self.server_port < 1024 || self.party_port < 1024 {
            return Err(AppError::user("portTooLow", "Ports must be 1024 or higher."));
        }
        crate::party::proto::validate_name(&self.player_name).map_err(AppError::bad)?;
        if self.player_color >= PALETTE_SIZE {
            return Err(AppError::bad("colour outside the palette"));
        }
        for (dir, what) in [(&self.screenshots_dir, "screenshots"), (&self.logs_dir, "logs")] {
            if dir.as_ref().is_some_and(|d| d.as_os_str().len() > 1024 || !d.is_absolute()) {
                return Err(AppError::user_with(
                    "pathNotAbsolute",
                    what,
                    format!("The {what} folder must be a full path."),
                ));
            }
        }
        Ok(())
    }

    /// Carpeta de capturas efectiva (la configurada o la del juego por defecto).
    pub fn effective_screenshots_dir(&self) -> Option<PathBuf> {
        self.screenshots_dir.clone().or_else(default_screenshots_dir)
    }
}

/// Posibles carpetas de capturas, en orden: Documentos del sistema y Documentos en OneDrive
/// (Windows suele redirigirlos ahí). El juego crea `Escape from Tarkov\Screenshots` dentro.
pub fn screenshot_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(dirs) = directories::UserDirs::new() {
        if let Some(docs) = dirs.document_dir() {
            out.push(docs.to_path_buf());
        }
        let home = dirs.home_dir();
        for sub in ["OneDrive/Documents", "OneDrive/Documentos", "Documents", "Documentos"] {
            out.push(home.join(sub));
        }
    }
    let mut seen = std::collections::HashSet::new();
    out.into_iter()
        .map(|d| d.join("Escape from Tarkov").join("Screenshots"))
        .filter(|p| seen.insert(p.clone()))
        .collect()
}

/// La primera candidata que existe; si ninguna existe, la de Documentos (para mostrarla).
pub fn default_screenshots_dir() -> Option<PathBuf> {
    let candidates = screenshot_candidates();
    candidates.iter().find(|p| p.is_dir()).cloned().or_else(|| candidates.into_iter().next())
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub base: PathBuf,
    pub config_file: PathBuf,
    pub points_file: PathBuf,
    pub log_file: PathBuf,
    pub icons_dir: PathBuf,
    pub maps_dir: PathBuf,
    pub staging_dir: PathBuf,
}

impl Paths {
    pub fn discover() -> AppResult<Self> {
        let proj = directories::ProjectDirs::from("", "", "Rastro")
            .ok_or_else(|| AppError::Internal("the data folder was not found".into()))?;
        Ok(Self::at(proj.data_dir()))
    }

    pub fn at(base: &Path) -> Self {
        Self {
            base: base.to_path_buf(),
            config_file: base.join("config.json"),
            points_file: base.join("points.json"),
            log_file: base.join("rastro.log"),
            icons_dir: base.join("icons"),
            maps_dir: base.join("maps"),
            staging_dir: base.join("staging"),
        }
    }

    pub fn ensure(&self) -> AppResult<()> {
        std::fs::create_dir_all(&self.maps_dir)?;
        // La zona temporal se vacía en cada arranque: restos de importaciones a medias.
        if self.staging_dir.exists() {
            std::fs::remove_dir_all(&self.staging_dir)?;
        }
        std::fs::create_dir_all(&self.staging_dir)?;
        Ok(())
    }
}

/// Lee la configuración con límite de tamaño; si es inválida, avisa y usa valores por defecto.
pub fn load(path: &Path) -> Config {
    match read_limited(path, MAX_CONFIG_BYTES) {
        Ok(Some(bytes)) => match serde_json::from_slice::<Config>(&bytes) {
            Ok(cfg) if cfg.validate().is_ok() => cfg,
            _ => {
                tracing::warn!("config.json is not valid; using defaults");
                Config::default()
            }
        },
        Ok(None) => Config::default(),
        Err(e) => {
            tracing::warn!("could not read config.json: {e}");
            Config::default()
        }
    }
}

pub fn save(path: &Path, cfg: &Config) -> AppResult<()> {
    let json = serde_json::to_vec_pretty(cfg).map_err(|e| AppError::Internal(e.to_string()))?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, json)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Lee un archivo entero si no supera `max` bytes. `Ok(None)` si no existe.
pub fn read_limited(path: &Path, max: u64) -> AppResult<Option<Vec<u8>>> {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    if meta.len() > max {
        return Err(AppError::bad(format!("{} supera {max} bytes", path.display())));
    }
    Ok(Some(std::fs::read(path)?))
}
