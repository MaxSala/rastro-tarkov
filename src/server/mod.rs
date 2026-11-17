//! Servidor local en 127.0.0.1: web embebida, API de paquetes y WebSocket /ws.

mod api;
mod assets;
mod ws;

use crate::maps::zipx::MAX_ZIP_BYTES;
use crate::state::App;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use subtle::ConstantTimeEq;
use tokio::sync::Semaphore;

pub const TOKEN_HEADER: &str = "x-rastro-token";
const MAX_JSON_BODY: usize = 64 * 1024;
const MAX_INFLIGHT: usize = 32;
pub const DEV_PORT: u16 = 5173;

const CSP: &str = "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' blob: data:; \
connect-src 'self'; font-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

#[derive(Clone)]
pub struct Ctx {
    pub app: Arc<App>,
    inflight: Arc<Semaphore>,
    pub ws_slots: Arc<Semaphore>,
}

pub fn token_ok(app: &App, candidate: &str) -> bool {
    bool::from(candidate.as_bytes().ct_eq(app.token.as_bytes()))
}

fn allowed_hosts(app: &App) -> Vec<String> {
    vec![format!("127.0.0.1:{}", app.port), format!("localhost:{}", app.port)]
}

fn allowed_origins(app: &App) -> Vec<String> {
    let mut v = vec![format!("http://127.0.0.1:{}", app.port), format!("http://localhost:{}", app.port)];
    if app.dev {
        v.push(format!("http://127.0.0.1:{DEV_PORT}"));
        v.push(format!("http://localhost:{DEV_PORT}"));
    }
    v
}

fn header_str(h: &HeaderMap, name: header::HeaderName) -> Option<&str> {
    h.get(name).and_then(|v| v.to_str().ok())
}

/// Decide si una petición se acepta. Devuelve el motivo del rechazo (solo para logs).
pub fn check_request(app: &App, method: &Method, path: &str, h: &HeaderMap) -> Result<(), &'static str> {
    // Host exacto: frena el DNS rebinding (evil.com resolviendo a 127.0.0.1).
    let host = header_str(h, header::HOST).ok_or("sin Host")?;
    if !allowed_hosts(app).iter().any(|a| a == host) {
        return Err("Host no permitido");
    }
    let origin = header_str(h, header::ORIGIN);
    if let Some(o) = origin {
        if !allowed_origins(app).iter().any(|a| a == o) {
            return Err("Origin no permitido");
        }
    }
    if header_str(h, header::HeaderName::from_static("sec-fetch-site")) == Some("cross-site") {
        return Err("petición cross-site");
    }
    let is_ws = path == "/ws";
    let is_api = path.starts_with("/api/");
    if (is_ws || method != Method::GET) && origin.is_none() {
        return Err("falta Origin");
    }
    if is_api {
        let t = h.get(TOKEN_HEADER).and_then(|v| v.to_str().ok()).ok_or("sin token")?;
        if !token_ok(app, t) {
            return Err("token incorrecto");
        }
    }
    if method == Method::POST {
        let ct = header_str(h, header::CONTENT_TYPE).unwrap_or("");
        let expected = if path == "/api/import/upload" { "application/zip" } else { "application/json" };
        if ct.split(';').next().map(str::trim) != Some(expected) {
            return Err("Content-Type no permitido");
        }
    } else if method != Method::GET {
        return Err("método no permitido");
    }
    Ok(())
}

async fn guard(State(ctx): State<Ctx>, req: Request, next: Next) -> Response {
    let Ok(_permit) = ctx.inflight.clone().try_acquire_owned() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let mut resp = match check_request(&ctx.app, req.method(), req.uri().path(), req.headers()) {
        Ok(()) => next.run(req).await,
        Err(why) => {
            tracing::warn!("petición rechazada: {why}");
            (StatusCode::FORBIDDEN, "Acceso denegado.").into_response()
        }
    };
    let h = resp.headers_mut();
    if !h.contains_key(header::CONTENT_SECURITY_POLICY) {
        h.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CSP));
    }
    h.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    h.insert(header::REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    h.insert("cross-origin-resource-policy", HeaderValue::from_static("same-origin"));
    h.insert("cross-origin-opener-policy", HeaderValue::from_static("same-origin"));
    h.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    resp
}

pub fn router(app: Arc<App>) -> Router {
    let ctx = Ctx {
        app,
        inflight: Arc::new(Semaphore::new(MAX_INFLIGHT)),
        ws_slots: Arc::new(Semaphore::new(ws::MAX_SOCKETS)),
    };
    let upload =
        Router::new().route("/api/import/upload", post(api::upload)).layer(DefaultBodyLimit::max(MAX_ZIP_BYTES));
    Router::new()
        .route("/ws", get(ws::handler))
        .route("/api/packs", get(api::list))
        .route("/api/packs/{id}/image", get(api::image))
        .route("/api/icons/{name}", get(api::icon))
        .route("/api/market", get(api::market))
        .route("/api/item-icon/{id}", get(api::item_icon))
        .route("/api/key-locations/{id}", get(api::key_locations))
        .route("/api/import/demo", post(api::demo))
        .route("/api/import/inspect-url", post(api::inspect_url))
        .route("/api/import/url", post(api::from_url))
        .route("/api/import/commit", post(api::commit))
        .route("/api/import/cancel", post(api::cancel))
        .layer(DefaultBodyLimit::max(MAX_JSON_BODY))
        .merge(upload)
        .fallback(get(assets::serve))
        .layer(middleware::from_fn_with_state(ctx.clone(), guard))
        .with_state(ctx)
}

#[cfg(test)]
#[path = "guard_tests.rs"]
mod tests;
