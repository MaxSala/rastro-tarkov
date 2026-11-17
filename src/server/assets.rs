//! Web compilada embebida en el binario (rust-embed). En debug se lee de web/dist.

use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};

#[derive(rust_embed::Embed)]
#[folder = "web/dist/"]
#[allow_missing = true]
struct Web;

fn mime(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, e)| e) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("woff2") => "font/woff2",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    }
}

const MAP_PACK_DOC: &str = include_str!("../../docs/MAP_PACK.md");

pub async fn serve(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path == "docs/MAP_PACK.md" {
        // Texto plano: el navegador no interpreta nada del documento.
        return ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], MAP_PACK_DOC).into_response();
    }
    let (path, file) = match Web::get(path) {
        Some(f) if !path.is_empty() => (path, f),
        _ => match Web::get("index.html") {
            Some(f) => ("index.html", f),
            None => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "La web no está compilada. Ejecuta `npm run build` en web/ y vuelve a compilar, o usa el modo desarrollo.",
                )
                    .into_response()
            }
        },
    };
    // Los archivos de assets/ llevan hash en el nombre; index.html nunca se cachea.
    let cache = if path.starts_with("assets/") { "public, max-age=31536000, immutable" } else { "no-store" };
    ([(header::CONTENT_TYPE, mime(path)), (header::CACHE_CONTROL, cache)], file.data.into_owned()).into_response()
}
