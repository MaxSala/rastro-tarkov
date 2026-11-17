//! Descarga e instala todos los mapas con SVG de tarkov.dev. Un mapa que falla no para al resto.

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
const SVG_SOURCE: &str = "https://github.com/the-hideout/tarkov-dev-svg-maps";

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
    pub message: String,
    pub done: usize,
    pub total: usize,
}

fn set(app: &App, f: impl FnOnce(&mut SyncStatus)) {
    f(&mut lock(&app.tarkovdev));
    app.notify();
}

/// Lanza la sincronización si no hay otra en curso.
pub fn start(app: &Arc<App>) -> AppResult<()> {
    {
        let mut st = lock(&app.tarkovdev);
        if st.state == SyncState::Running {
            return Err(AppError::bad("ya hay una descarga de tarkov.dev en curso"));
        }
        *st =
            SyncStatus { state: SyncState::Running, message: "Leyendo la lista de mapas…".into(), done: 0, total: 0 };
    }
    app.notify();
    let app = app.clone();
    tokio::spawn(async move {
        let result = run(&app).await;
        set(&app, |st| match result {
            Ok(msg) => *st = SyncStatus { state: SyncState::Done, message: msg, done: st.total, total: st.total },
            Err(e) => {
                st.state = SyncState::Error;
                st.message = e.to_string();
            }
        });
    });
    Ok(())
}

async fn get(url: &str, max: usize, body: Option<&str>) -> AppResult<Vec<u8>> {
    let (u, _) = inspect(url)?;
    fetch_with(u, max, Options { json_body: body, allowed_hosts: Some(HOSTS) }).await
}

async fn run(app: &Arc<App>) -> AppResult<String> {
    let maps = calib::parse_maps(&get(MAPS_JSON, 4 << 20, None).await?).map_err(AppError::Download)?;
    set(app, |st| st.message = "Descargando extracciones y loot…".into());
    // Sin datos los mapas siguen siendo útiles (posición, party): se instalan sin capas.
    let mode = app.effective_mode();
    let lang = lock(&app.inner).config.language.code();
    let (data, api_error) = match fetch_data(mode, lang).await {
        Ok(d) => (d, None),
        Err(e) => {
            tracing::warn!("tarkov.dev: sin datos de mapa: {e}");
            (Vec::new(), Some(e))
        }
    };
    set(app, |st| st.total = maps.len());
    let (mut installed, mut failed) = (Vec::new(), Vec::new());
    for m in &maps {
        set(app, |st| st.message = format!("Instalando {}…", m.id));
        match install_one(app, m, data.iter().find(|d| d.normalized_name == m.id)).await {
            Ok(name) => installed.push(name),
            Err(e) => {
                tracing::warn!("tarkov.dev: {} no se instaló: {e}", m.id);
                failed.push(m.id.clone());
            }
        }
        set(app, |st| st.done += 1);
    }
    if installed.is_empty() {
        return Err(AppError::Download("no se pudo instalar ningún mapa".into()));
    }
    let mut msg = format!("Instalados {} mapas.", installed.len());
    if !failed.is_empty() {
        msg.push_str(&format!(" Fallaron: {}.", failed.join(", ")));
    }
    set(app, |st| st.message = "Descargando iconos…".into());
    let missing = download_icons(app).await;
    if missing > 0 {
        msg.push_str(&format!(" {missing} iconos no se pudieron descargar (se usan formas simples)."));
    }
    if let Some(e) = api_error {
        msg.push_str(&format!(" Sin extracciones ni loot ({e}); vuelve a intentarlo más tarde."));
    }
    Ok(msg)
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
    let mut levels = vec![full(base.clone(), "planta-base".into(), "Planta baja".into(), Vec::new())];
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
    tracing::warn!("tarkov.dev: json.tarkov.dev falló ({json_err}); se prueba GraphQL");
    let query = serde_json::json!({ "query": api::QUERY.replace("{mode}", m).replace("{lang}", lang) }).to_string();
    match get(API, 32 << 20, Some(&query)).await.and_then(|b| api::parse(&b).map_err(AppError::Download)) {
        Ok(maps) => Ok(api::to_map_data(maps)),
        Err(e) => Err(format!("json.tarkov.dev: {json_err} · GraphQL: {e}")),
    }
}

async fn install_one(app: &Arc<App>, m: &calib::SvgMap, data: Option<&api::MapData>) -> AppResult<String> {
    let svg = get(&format!("{SVG_BASE}{}", m.file), 20 << 20, None).await?;
    let (name, file, json) = build_pack(m, &svg, data)?;
    let app = app.clone();
    tokio::task::spawn_blocking(move || app.install_files(&[("manifest.json", &json), (&file, &svg)]))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    Ok(name)
}

/// Construye el manifest de un mapa (sin red). Devuelve nombre, archivo de imagen y manifest.
pub fn build_pack(m: &calib::SvgMap, svg: &[u8], data: Option<&api::MapData>) -> AppResult<(String, String, Vec<u8>)> {
    let text = std::str::from_utf8(svg).map_err(|_| AppError::pack("SVG no UTF-8"))?;
    check_svg(svg).map_err(AppError::InvalidPack)?;
    let (w, h) = calib::svg_size(text).ok_or_else(|| AppError::pack("el SVG no tiene un viewBox utilizable"))?;
    let file = format!("{}.svg", m.id);
    let (base_layer, levels) = levels_for(m, text);
    let name = data.and_then(|d| d.name.clone()).unwrap_or_else(|| m.id.clone());
    let author = api::clean(Some(&format!("{} y colaboradores de tarkov.dev", m.author)), 100, "tarkov.dev");
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
            license: "Mapa CC BY-NC-SA 4.0 (sin uso en trampas); datos: API de tarkov.dev".into(),
            source: SVG_SOURCE.into(),
        },
    };
    manifest.validate().map_err(AppError::InvalidPack)?;
    let json = serde_json::to_vec_pretty(&manifest).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok((name, file, json))
}
