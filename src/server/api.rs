//! Endpoints HTTP de paquetes. Cada handler valida su entrada; el token ya lo comprobó la guardia.

use super::Ctx;
use crate::config::read_limited;
use crate::error::{AppError, AppResult};
use crate::maps::import::Preview;
use crate::maps::manifest::is_valid_id;
use crate::maps::zipx::{MAX_UNPACKED_BYTES, MAX_ZIP_BYTES};
use crate::maps::{download, image::Kind};
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

/// Una importación a la vez: cada una puede ocupar hasta 50 MB en memoria.
static IMPORT_SLOT: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(1);

async fn import_slot() -> AppResult<tokio::sync::SemaphorePermit<'static>> {
    IMPORT_SLOT.try_acquire().map_err(|_| AppError::bad("ya hay una importación en curso; espera a que termine"))
}

async fn blocking<T: Send + 'static>(f: impl FnOnce() -> AppResult<T> + Send + 'static) -> AppResult<T> {
    tokio::task::spawn_blocking(f).await.map_err(|e| AppError::Internal(e.to_string()))?
}

pub async fn list(State(ctx): State<Ctx>) -> Json<Value> {
    let view = crate::view::build(&ctx.app);
    Json(json!({ "packs": view.packs }))
}

/// Oculta los pisos que no tocan inyectando un `<style>` tras la etiqueta raíz del SVG.
/// Los ids vienen del manifest y ya están validados como seguros para un selector CSS.
fn floor_variant(svg: Vec<u8>, pack: &crate::maps::pack::Pack, floor: Option<&str>) -> Vec<u8> {
    let m = &pack.manifest;
    let Some(base) = m.image.base_layer.as_deref() else { return svg };
    let shown = floor.and_then(|f| m.levels.iter().find(|l| l.id == f)).and_then(|l| l.svg_layer.as_deref());
    let hidden: Vec<String> = m
        .levels
        .iter()
        .filter_map(|l| l.svg_layer.as_deref())
        .filter(|g| *g != base && Some(*g) != shown)
        .map(|g| format!("#{g}"))
        .collect();
    let Ok(text) = String::from_utf8(svg) else { return Vec::new() };
    let insert_at = text.find("<svg").and_then(|i| text.get(i..)?.find('>').map(|j| i + j + 1));
    match (insert_at, hidden.is_empty()) {
        (Some(at), false) => {
            let mut out = String::with_capacity(text.len() + 128);
            out.push_str(text.get(..at).unwrap_or_default());
            out.push_str(&format!("<style>{}{{display:none}}</style>", hidden.join(",")));
            out.push_str(text.get(at..).unwrap_or_default());
            out.into_bytes()
        }
        _ => text.into_bytes(),
    }
}

fn query_param<'a>(uri: &'a Uri, key: &str) -> Option<&'a str> {
    uri.query()?.split('&').find_map(|kv| kv.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v))
}

/// Solo sirve la imagen declarada en el manifest de un paquete instalado (lista blanca).
/// `?floor=<id de piso>` devuelve el SVG con ese piso visible.
pub async fn image(State(ctx): State<Ctx>, Path(id): Path<String>, uri: Uri) -> AppResult<Response> {
    if !is_valid_id(&id) {
        return Err(AppError::NotFound);
    }
    let pack = ctx.app.pack(&id).ok_or(AppError::NotFound)?;
    let path = pack.image_path();
    let mut bytes = blocking(move || read_limited(&path, MAX_UNPACKED_BYTES)?.ok_or(AppError::NotFound)).await?;
    if pack.kind == Kind::Svg {
        bytes = floor_variant(bytes, &pack, query_param(&uri, "floor").filter(|f| is_valid_id(f)));
    }
    let csp = "default-src 'none'; img-src data:; style-src 'unsafe-inline'; sandbox";
    let mime = pack.kind.mime();
    let headers =
        [(header::CONTENT_TYPE, mime), (header::CONTENT_SECURITY_POLICY, csp), (header::CACHE_CONTROL, "no-store")];
    let disposition = if pack.kind == Kind::Svg { "inline; filename=\"map.svg\"" } else { "inline" };
    Ok((headers, [(header::CONTENT_DISPOSITION, disposition)], bytes).into_response())
}

/// Catálogo del mercado (grande: va por HTTP, no por el WebSocket).
pub async fn market(State(ctx): State<Ctx>) -> AppResult<Response> {
    let app = ctx.app.clone();
    let cat = blocking(move || app.market_catalog().ok_or(AppError::NotFound)).await?;
    let json = serde_json::to_vec(&*cat).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(([(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")], json).into_response())
}

/// Mapas instalados donde se usa una llave.
pub async fn key_locations(State(ctx): State<Ctx>, Path(id): Path<String>) -> AppResult<Response> {
    if !crate::market::icons::valid_id(&id) {
        return Err(AppError::NotFound);
    }
    let list = crate::view_extra::key_locations(&ctx.app, &id.to_ascii_lowercase());
    Ok(([(header::CACHE_CONTROL, "no-store")], axum::Json(list)).into_response())
}

/// Icono de un objeto (id de 24 hex): de la caché o de assets.tarkov.dev.
pub async fn item_icon(State(ctx): State<Ctx>, Path(id): Path<String>) -> AppResult<Response> {
    let (bytes, kind) = ctx.app.item_icon(&id).await?;
    Ok(([(header::CONTENT_TYPE, kind.mime()), (header::CACHE_CONTROL, "private, max-age=86400")], bytes)
        .into_response())
}

/// Iconos de marcadores: solo nombres de la lista cerrada y ya descargados.
pub async fn icon(State(ctx): State<Ctx>, Path(name): Path<String>) -> AppResult<Response> {
    let name = name.strip_suffix(".png").unwrap_or(&name).to_owned();
    let path = crate::tarkovdev::icons::path(&ctx.app.paths.icons_dir, &name).ok_or(AppError::NotFound)?;
    let bytes = blocking(move || read_limited(&path, 64 * 1024)?.ok_or(AppError::NotFound)).await?;
    Ok(([(header::CONTENT_TYPE, "image/png"), (header::CACHE_CONTROL, "private, max-age=3600")], bytes).into_response())
}

pub async fn upload(State(ctx): State<Ctx>, headers: HeaderMap, body: Bytes) -> AppResult<Json<Preview>> {
    if body.len() > MAX_ZIP_BYTES {
        return Err(AppError::pack("el zip supera 50 MB"));
    }
    let sha = headers.get("x-expected-sha256").and_then(|v| v.to_str().ok()).map(str::to_owned);
    let app = ctx.app.clone();
    blocking(move || app.stage_zip(&body, sha.as_deref())).await.map(Json)
}

pub async fn demo(State(ctx): State<Ctx>) -> AppResult<Json<Preview>> {
    let app = ctx.app.clone();
    blocking(move || app.stage_demo()).await.map(Json)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlInput {
    url: String,
    #[serde(default)]
    sha256: Option<String>,
}

pub async fn inspect_url(Json(input): Json<UrlInput>) -> AppResult<Json<Value>> {
    let (_, host) = download::inspect(&input.url)?;
    Ok(Json(json!({ "host": host })))
}

pub async fn from_url(State(ctx): State<Ctx>, Json(input): Json<UrlInput>) -> AppResult<Json<Preview>> {
    let _slot = import_slot().await?;
    let (url, _) = download::inspect(&input.url)?;
    let bytes = download::fetch(url, MAX_ZIP_BYTES).await?;
    let app: Arc<_> = ctx.app.clone();
    blocking(move || app.stage_zip(&bytes, input.sha256.as_deref())).await.map(Json)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CommitInput {
    staging_id: String,
    overwrite: bool,
    accept_license: bool,
}

pub async fn commit(State(ctx): State<Ctx>, Json(input): Json<CommitInput>) -> AppResult<Json<Value>> {
    if !input.accept_license {
        return Err(AppError::bad("hay que aceptar los créditos y la licencia del paquete"));
    }
    let app = ctx.app.clone();
    let id = blocking(move || app.commit_import(&input.staging_id, input.overwrite)).await?;
    Ok(Json(json!({ "id": id })))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelInput {
    staging_id: String,
}

pub async fn cancel(State(ctx): State<Ctx>, Json(input): Json<CancelInput>) -> Json<Value> {
    ctx.app.cancel_import(&input.staging_id);
    Json(json!({}))
}
