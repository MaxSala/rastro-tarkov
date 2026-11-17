use super::*;
use crate::config::{Config, Paths};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use zeroize::Zeroizing;

const PORT: u16 = 47999;
const TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn test_app() -> Arc<App> {
    let base = std::env::temp_dir().join(format!("rastro-srv-{}", crate::maps::import::random_hex(8).expect("rng")));
    let paths = Paths::at(&base);
    paths.ensure().expect("dirs");
    let cfg = Config { screenshots_dir: Some(base.join("no-existe")), ..Config::default() };
    App::new(paths, cfg, Zeroizing::new(TOKEN.into()), PORT, false)
}

fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
    let mut h = HeaderMap::new();
    for (k, v) in pairs {
        h.insert(header::HeaderName::from_bytes(k.as_bytes()).expect("k"), HeaderValue::from_str(v).expect("v"));
    }
    h
}

const HOST: (&str, &str) = ("host", "127.0.0.1:47999");
const ORIGIN: (&str, &str) = ("origin", "http://127.0.0.1:47999");
const AUTH: (&str, &str) = ("x-rastro-token", TOKEN);
const JSON: (&str, &str) = ("content-type", "application/json");

#[tokio::test]
async fn host_and_origin_must_match_exactly() {
    let app = test_app();
    let get = Method::GET;
    assert!(check_request(&app, &get, "/", &headers(&[HOST])).is_ok());
    // DNS rebinding: el navegador manda el dominio del atacante como Host.
    assert!(check_request(&app, &get, "/", &headers(&[("host", "evil.example:47999")])).is_err());
    assert!(check_request(&app, &get, "/", &headers(&[("host", "127.0.0.1:1")])).is_err());
    assert!(check_request(&app, &get, "/", &headers(&[])).is_err());
    let evil = ("origin", "https://evil.example");
    assert!(check_request(&app, &get, "/ws", &headers(&[HOST, evil])).is_err());
    assert!(check_request(&app, &get, "/ws", &headers(&[HOST])).is_err(), "ws sin Origin");
    assert!(check_request(&app, &get, "/ws", &headers(&[HOST, ORIGIN])).is_ok());
    let dev_origin = ("origin", "http://localhost:5173");
    assert!(check_request(&app, &get, "/ws", &headers(&[HOST, dev_origin])).is_err(), "sin --dev");
    assert!(check_request(&app, &get, "/", &headers(&[HOST, ("sec-fetch-site", "cross-site")])).is_err());
}

#[tokio::test]
async fn api_requires_valid_token_and_json() {
    let app = test_app();
    let (get, post) = (Method::GET, Method::POST);
    assert!(check_request(&app, &get, "/api/packs", &headers(&[HOST])).is_err(), "sin token");
    let wrong = ("x-rastro-token", "f".repeat(64));
    assert!(check_request(&app, &get, "/api/packs", &headers(&[HOST, (wrong.0, wrong.1.as_str())])).is_err());
    assert!(check_request(&app, &get, "/api/packs", &headers(&[HOST, AUTH])).is_ok());
    let p = "/api/import/commit";
    assert!(check_request(&app, &post, p, &headers(&[HOST, ORIGIN, AUTH, JSON])).is_ok());
    assert!(check_request(&app, &post, p, &headers(&[HOST, AUTH, JSON])).is_err(), "POST sin Origin");
    let form = ("content-type", "application/x-www-form-urlencoded");
    assert!(check_request(&app, &post, p, &headers(&[HOST, ORIGIN, AUTH, form])).is_err(), "CSRF por formulario");
    let zip = ("content-type", "application/zip");
    assert!(check_request(&app, &post, "/api/import/upload", &headers(&[HOST, ORIGIN, AUTH, zip])).is_ok());
    assert!(check_request(&app, &Method::DELETE, "/api/packs", &headers(&[HOST, ORIGIN, AUTH])).is_err());
}

async fn raw(port: u16, request: &str) -> String {
    let mut s = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.expect("conecta");
    s.write_all(request.as_bytes()).await.expect("w");
    let mut out = Vec::new();
    let _ = tokio::time::timeout(std::time::Duration::from_secs(3), s.read_to_end(&mut out)).await;
    String::from_utf8_lossy(&out).into_owned()
}

#[tokio::test]
async fn end_to_end_rejections_and_security_headers() {
    let app = test_app();
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", PORT)).await.expect("bind");
    tokio::spawn(async move { axum::serve(listener, router(app)).await });
    let ok = raw(
        PORT,
        &format!(
            "GET /api/packs HTTP/1.1\r\nHost: 127.0.0.1:{PORT}\r\nx-rastro-token: {TOKEN}\r\nConnection: close\r\n\r\n"
        ),
    )
    .await;
    assert!(ok.starts_with("HTTP/1.1 200"), "{ok}");
    let lower = ok.to_ascii_lowercase();
    for h in [
        "content-security-policy: default-src 'self'",
        "x-content-type-options: nosniff",
        "referrer-policy: no-referrer",
        "cross-origin-resource-policy: same-origin",
    ] {
        assert!(lower.contains(h), "falta {h}");
    }
    let no_token =
        raw(PORT, &format!("GET /api/packs HTTP/1.1\r\nHost: 127.0.0.1:{PORT}\r\nConnection: close\r\n\r\n")).await;
    assert!(no_token.starts_with("HTTP/1.1 403"));
    let rebinding = raw(
        PORT,
        &format!(
            "GET /api/packs HTTP/1.1\r\nHost: attacker.example\r\nx-rastro-token: {TOKEN}\r\nConnection: close\r\n\r\n"
        ),
    )
    .await;
    assert!(rebinding.starts_with("HTTP/1.1 403"));
    let ws = raw(PORT, &format!("GET /ws HTTP/1.1\r\nHost: 127.0.0.1:{PORT}\r\nOrigin: https://evil.example\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\r\n")).await;
    assert!(ws.starts_with("HTTP/1.1 403"), "{ws}");
    let traversal = raw(PORT, &format!("GET /api/packs/..%2F..%2Fconfig/image HTTP/1.1\r\nHost: 127.0.0.1:{PORT}\r\nx-rastro-token: {TOKEN}\r\nConnection: close\r\n\r\n")).await;
    assert!(traversal.starts_with("HTTP/1.1 404"), "{traversal}");
}
