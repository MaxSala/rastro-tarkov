//! Descarga e instala todos los mapas con SVG de tarkov.dev. Un mapa que falla no para al resto.
//! [`Job::Relocalize`] rebuilds the installed maps with the data in another language: it reuses
//! each map's SVG already on disk and only downloads the (small) map data.

use super::{api, calib};
use crate::error::{AppError, AppResult};
use crate::maps::download::{fetch_with, inspect, Options};
use crate::maps::image::check_svg;
use crate::maps::manifest::{Credits, ImageRef, Level, Manifest, SCHEMA_VERSION};
use crate::party::lock;
use crate::state::App;
use serde::Serialize;
use std::sync::Arc;

pub const HOSTS: &[&str] = &["raw.githubusercontent.com", "json.tarkov.dev", "api.tarkov.dev"];
/// Datos de json.tarkov.dev por modo (`regular` o `pve`).
const JSON_BASE: &str = "https://json.tarkov.dev/";
const MAPS_JSON: &str = "https://raw.githubusercontent.com/the-hideout/tarkov-dev/main/src/data/maps.json";
const SVG_BASE: &str = "https://raw.githubusercontent.com/the-hideout/tarkov-dev-svg-maps/main/";
const API: &str = "https://api.tarkov.dev/graphql";
/// Packs generated from tarkov.dev carry this as their source.
pub const SVG_SOURCE: &str = "https://github.com/the-hideout/tarkov-dev-svg-maps";
const MAX_SVG: usize = 20 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    /// Download and install every map.
    Full,
    /// Rebuild the installed maps with the data in the current language.
    Relocalize,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SyncState {
    #[default]
    Idle,
    Running,
    Done,
    Error,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus {
    pub state: SyncState,
    /// Current stage while running: `maps`, `data`, `install` (with `current`) or `icons`.
    pub step: Option<&'static str>,
    pub current: Option<String>,
    pub done: usize,
    pub total: usize,
    /// Summary when finished; the interface writes the sentence in its own language.
    pub installed: usize,
    pub failed: Vec<String>,
    pub icons_missing: usize,
    /// The maps installed, but without extracts or loot (the data download failed).
    pub no_data: bool,
    pub error: Option<crate::msg::UserMsg>,
}

/// Result of a successful sync.
struct Summary {
    installed: usize,
    failed: Vec<String>,
    icons_missing: usize,
    no_data: bool,
}

fn set(app: &App, f: impl FnOnce(&mut SyncStatus)) {
    f(&mut lock(&app.tarkovdev));
    app.notify();
}

/// Lanza la sincronización si no hay otra en curso.
pub fn start(app: &Arc<App>) -> AppResult<()> {
    start_job(app, Job::Full)
}

pub fn start_job(app: &Arc<App>, job: Job) -> AppResult<()> {
    {
        let mut st = lock(&app.tarkovdev);
        if st.state == SyncState::Running {
            return Err(AppError::user("syncBusy", "A tarkov.dev download is already running."));
        }
        *st = SyncStatus { state: SyncState::Running, step: Some("maps"), ..SyncStatus::default() };
    }
    app.notify();
    let app = app.clone();
    tokio::spawn(async move {
        let result = run(&app, job).await;
        set(&app, |st| match result {
            Ok(sum) => {
                *st = SyncStatus {
                    state: SyncState::Done,
                    done: st.total,
                    total: st.total,
                    installed: sum.installed,
                    failed: sum.failed,
                    icons_missing: sum.icons_missing,
                    no_data: sum.no_data,
                    ..SyncStatus::default()
                }
            }
            Err(e) => {
                tracing::warn!("tarkov.dev: {job:?} failed: {e}");
                st.state = SyncState::Error;
                st.step = None;
                st.error = Some(e.user_msg());
            }
        });
    });
    Ok(())
}

async fn get(url: &str, max: usize, body: Option<&str>) -> AppResult<Vec<u8>> {
    let (u, _) = inspect(url)?;
    fetch_with(u, max, Options { json_body: body, allowed_hosts: Some(HOSTS) }).await
}

/// The tarkov.dev map installed with this id, if any.
fn installed(app: &App, id: &str) -> Option<Arc<crate::maps::pack::Pack>> {
    app.pack(id).filter(|p| p.manifest.credits.source == SVG_SOURCE)
}

async fn run(app: &Arc<App>, job: Job) -> AppResult<Summary> {
    let mut maps = calib::parse_maps(&get(MAPS_JSON, 4 << 20, None).await?).map_err(AppError::Download)?;
    if job == Job::Relocalize {
        maps.retain(|m| installed(app, &m.id).is_some());
    }
    set(app, |st| st.step = Some("data"));
    let mode = app.effective_mode();
    let language = lock(&app.inner).config.language;
    let lang = language.code();
    let (data, api_error) = match fetch_data(mode, lang).await {
        Ok(d) => (d, None),
        // Rebuilding without data would only throw away the extracts the maps already have.
        Err(e) if job == Job::Relocalize => return Err(AppError::Download(e)),
        // Sin datos los mapas siguen siendo útiles (posición, party): se instalan sin capas.
        Err(e) => {
            tracing::warn!("tarkov.dev: no map data: {e}");
            (Vec::new(), Some(e))
        }
    };
    set(app, |st| st.total = maps.len());
    let (mut installed, mut failed) = (Vec::new(), Vec::new());
    for m in &maps {
        set(app, |st| {
            st.step = Some("install");
            st.current = Some(m.id.clone());
        });
        match install_one(app, m, data.iter().find(|d| d.normalized_name == m.id), job).await {
            Ok(name) => installed.push(name),
            Err(e) => {
                tracing::warn!("tarkov.dev: {} was not installed: {e}", m.id);
                failed.push(m.id.clone());
            }
        }
        set(app, |st| st.done += 1);
    }
    if installed.is_empty() {
        return Err(AppError::Download("no map could be installed".into()));
    }
    if api_error.is_none() {
        app.set_maps_language(language);
    }
    if job == Job::Relocalize {
        return Ok(Summary { installed: installed.len(), failed, icons_missing: 0, no_data: false });
    }
    set(app, |st| {
        st.step = Some("icons");
        st.current = None;
    });
    let icons_missing = download_icons(app).await;
    Ok(Summary { installed: installed.len(), failed, icons_missing, no_data: api_error.is_some() })
}

/// Descarga los iconos de marcadores. Devuelve cuántos fallaron (no es un error fatal).
async fn download_icons(app: &Arc<App>) -> usize {
    let dir = app.paths.icons_dir.clone();
    let mut failed = 0;
    for name in super::icons::ICONS {
        let ok = match get(&format!("{}{name}.png", super::icons::BASE_URL), 64 * 1024, None).await {
            Ok(bytes) => super::icons::store(&dir, name, &bytes).is_ok(),
            Err(_) => false,
        };
        if !ok {
            failed += 1;
        }
    }
    failed
}

/// Nombres internos del juego para este mapa: el de tarkov.dev y los de la tabla de respaldo.
fn game_ids(map_id: &str, data: Option<&api::MapData>) -> Vec<String> {
    let mut ids: Vec<String> = data.and_then(|d| d.name_id.clone()).into_iter().collect();
    for loc in crate::gamelog::table_ids_for(map_id) {
        if !ids.iter().any(|i| i == loc) {
            ids.push(loc.to_owned());
        }
    }
    ids.truncate(8);
    ids
}

/// Pisos del mapa: la planta base y los grupos del SVG que existen de verdad en el archivo.
fn levels_for(m: &calib::SvgMap, svg: &str) -> (Option<String>, Vec<Level>) {
    let has = |g: &str| svg.contains(&format!("id=\"{g}\""));
    let Some(base) = m.base_layer.clone().filter(|b| has(b)) else { return (None, Vec::new()) };
    let full = |svg_layer: String, id: String, name: String, zones| Level {
        id,
        name,
        y_min: -100_000.0,
        y_max: 100_000.0,
        svg_layer: Some(svg_layer),
        zones,
    };
    let mut levels = vec![full(base.clone(), "planta-base".into(), "Ground floor".into(), Vec::new())];
    let mut used = std::collections::HashSet::from(["planta-base".to_owned()]);
    for l in m.layers.iter().filter(|l| has(&l.svg_layer) && l.svg_layer != base) {
        let mut id = api::normalize_name(&l.svg_layer);
        if id.is_empty() || !used.insert(id.clone()) {
            id = format!("piso-{}", levels.len());
            used.insert(id.clone());
        }
        levels.push(full(l.svg_layer.clone(), id, l.name.clone(), l.zones.clone()));
    }
    levels.truncate(16);
    (Some(base), levels)
}

/// json.tarkov.dev primero; si falla, GraphQL. Devuelve los dos errores si fallan ambos.
async fn fetch_data(mode: crate::gamelog::GameMode, lang: &str) -> Result<Vec<api::MapData>, String> {
    let m = mode.api_name();
    let json = async {
        let maps = get(&format!("{JSON_BASE}{m}/maps"), 64 << 20, None).await?;
        let locale = get(&format!("{JSON_BASE}{m}/maps_{lang}"), 32 << 20, None).await.unwrap_or_default();
        let english = get(&format!("{JSON_BASE}{m}/maps_en"), 32 << 20, None).await.unwrap_or_default();
        super::jsondoc::parse(&maps, &locale, &english).map_err(AppError::Download)
    };
    let json_err = match json.await {
        Ok(d) => return Ok(d),
        Err(e) => e.to_string(),
    };
    tracing::warn!("tarkov.dev: json.tarkov.dev failed ({json_err}); trying GraphQL");
    let query = serde_json::json!({ "query": api::QUERY.replace("{mode}", m).replace("{lang}", lang) }).to_string();
    match get(API, 32 << 20, Some(&query)).await.and_then(|b| api::parse(&b).map_err(AppError::Download)) {
        Ok(maps) => Ok(api::to_map_data(maps)),
        Err(e) => Err(format!("json.tarkov.dev: {json_err} · GraphQL: {e}")),
    }
}

async fn install_one(app: &Arc<App>, m: &calib::SvgMap, data: Option<&api::MapData>, job: Job) -> AppResult<String> {
    let svg = match job {
        Job::Full => get(&format!("{SVG_BASE}{}", m.file), MAX_SVG, None).await?,
        Job::Relocalize => {
            let path = installed(app, &m.id).ok_or(AppError::NotFound)?.image_path();
            tokio::task::spawn_blocking(move || crate::config::read_limited(&path, MAX_SVG as u64))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))??
                .ok_or(AppError::NotFound)?
        }
    };
    let (name, file, json) = build_pack(m, &svg, data)?;
    let app = app.clone();
    tokio::task::spawn_blocking(move || app.install_files(&[("manifest.json", &json), (&file, &svg)]))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    Ok(name)
}

/// Construye el manifest de un mapa (sin red). Devuelve nombre, archivo de imagen y manifest.
pub fn build_pack(m: &calib::SvgMap, svg: &[u8], data: Option<&api::MapData>) -> AppResult<(String, String, Vec<u8>)> {
    let text = std::str::from_utf8(svg).map_err(|_| AppError::pack("the SVG is not UTF-8"))?;
    check_svg(svg).map_err(AppError::InvalidPack)?;
    let (w, h) = calib::svg_size(text).ok_or_else(|| AppError::pack("the SVG has no usable viewBox"))?;
    let file = format!("{}.svg", m.id);
    let (base_layer, levels) = levels_for(m, text);
    let name = data.and_then(|d| d.name.clone()).unwrap_or_else(|| m.id.clone());
    let author = api::clean(Some(&format!("{} and tarkov.dev contributors", m.author)), 100, "tarkov.dev");
    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        id: m.id.clone(),
        name: name.clone(),
        image: ImageRef { file: file.clone(), width: w, height: h, base_layer },
        calibration: calib::calibration(m, w, h),
        levels,
        extracts: data.map(|d| d.extracts.clone()).unwrap_or_default(),
        markers: data.map(|d| d.markers.clone()).unwrap_or_default(),
        game_ids: game_ids(&m.id, data),
        raid_minutes: data.and_then(|d| d.raid_minutes),
        credits: Credits {
            author,
            license: "Map CC BY-NC-SA 4.0 (no use in cheats); data: tarkov.dev API".into(),
            source: SVG_SOURCE.into(),
        },
    };
    manifest.validate().map_err(AppError::InvalidPack)?;
    let json = serde_json::to_vec_pretty(&manifest).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok((name, file, json))
}
