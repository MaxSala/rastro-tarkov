//! Descarga de paquetes por URL con defensas contra SSRF.
//!
//! Resolvemos el DNS nosotros, rechazamos cualquier IP interna y fijamos la conexión a la IP
//! validada (`ClientBuilder::resolve`), así un segundo DNS no puede cambiarla (DNS rebinding).
//! Las redirecciones se siguen a mano y cada salto se valida de nuevo.

use crate::error::{AppError, AppResult};
use reqwest::{redirect::Policy, StatusCode, Url};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

const MAX_REDIRECTS: usize = 3;
const MAX_URL_LEN: usize = 2048;

/// Comprueba la forma de la URL (sin red). Devuelve la URL y su dominio.
pub fn inspect(raw: &str) -> AppResult<(Url, String)> {
    let raw = raw.trim();
    if raw.len() > MAX_URL_LEN {
        return Err(AppError::bad("la URL es demasiado larga"));
    }
    let url = Url::parse(raw).map_err(|_| AppError::bad("la URL no es válida"))?;
    if url.scheme() != "https" {
        return Err(AppError::bad("solo se admiten URLs https"));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(AppError::bad("la URL no puede llevar usuario ni contraseña"));
    }
    let host = url.host_str().ok_or_else(|| AppError::bad("la URL no tiene dominio"))?.to_owned();
    Ok((url, host))
}

fn forbidden_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local() // incluye 169.254.169.254 (metadatos de nube)
        || ip.is_unspecified()
        || ip.is_broadcast()
        || ip.is_multicast()
        || ip.is_documentation()
        || a == 0
        || (a == 100 && (64..=127).contains(&b)) // CGNAT (incluye 100.100.100.200)
        || (a == 192 && b == 0 && c == 0)
        || (a == 198 && (18..=19).contains(&b))
        || a >= 240
}

fn forbidden_v6(ip: Ipv6Addr) -> bool {
    if let Some(v4) = ip.to_ipv4_mapped() {
        return forbidden_v4(v4);
    }
    let seg = ip.segments();
    let first = seg.first().copied().unwrap_or(0);
    ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || (first & 0xFE00) == 0xFC00 // ULA, incluye fd00:ec2::254
        || (first & 0xFFC0) == 0xFE80 // link-local
        || (first & 0xFFC0) == 0xFEC0 // site-local
        || (first == 0x2001 && seg.get(1) == Some(&0x0DB8)) // documentación
        || (first == 0x0064 && seg.get(1) == Some(&0xFF9B)) // NAT64
        || first == 0x2002 // 6to4
        || (first == 0 && ip.to_ipv4().is_some()) // IPv4-compatible obsoleta
}

pub fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => forbidden_v4(v4),
        IpAddr::V6(v6) => forbidden_v6(v6),
    }
}

/// Resuelve el dominio y exige que TODAS sus IPs sean públicas.
pub async fn resolve_public(url: &Url) -> AppResult<SocketAddr> {
    let host = url.host_str().ok_or_else(|| AppError::bad("la URL no tiene dominio"))?;
    let port = url.port_or_known_default().unwrap_or(443);
    let bare = host.trim_start_matches('[').trim_end_matches(']');
    let addrs: Vec<SocketAddr> = match bare.parse::<IpAddr>() {
        Ok(ip) => vec![SocketAddr::new(ip, port)],
        Err(_) => tokio::time::timeout(Duration::from_secs(10), tokio::net::lookup_host((bare, port)))
            .await
            .map_err(|_| AppError::Download("el DNS no respondió a tiempo".into()))?
            .map_err(|_| AppError::Download(format!("no se pudo resolver {bare}")))?
            .collect(),
    };
    if addrs.is_empty() {
        return Err(AppError::Download(format!("{bare} no tiene direcciones")));
    }
    if addrs.iter().any(|a| is_forbidden_ip(a.ip())) {
        return Err(AppError::Download(format!(
            "{bare} apunta a una red local o privada; no se permite descargar desde ahí"
        )));
    }
    addrs.first().copied().ok_or_else(|| AppError::Download("sin direcciones".into()))
}

fn client_for(host: &str, addr: SocketAddr) -> AppResult<reqwest::Client> {
    reqwest::Client::builder()
        .redirect(Policy::none())
        .no_proxy()
        .https_only(true)
        .http1_only()
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(180))
        .user_agent("Rastro")
        .resolve(host, addr)
        .build()
        .map_err(|e| AppError::Internal(format!("cliente HTTP: {e}")))
}

/// Siguiente salto de una redirección, validado como una URL nueva.
pub fn next_hop(current: &Url, location: &str) -> AppResult<Url> {
    let joined = current.join(location).map_err(|_| AppError::Download("redirección no válida".into()))?;
    Ok(inspect(joined.as_str())?.0)
}

/// Petición opcional: cuerpo JSON (POST) y dominios permitidos (también en cada redirección).
#[derive(Default)]
pub struct Options<'a> {
    pub json_body: Option<&'a str>,
    pub allowed_hosts: Option<&'a [&'a str]>,
}

/// Descarga en streaming con tope de bytes. Sin cookies ni cabeceras de autenticación.
pub async fn fetch(start: Url, max_bytes: usize) -> AppResult<Vec<u8>> {
    fetch_with(start, max_bytes, Options::default()).await
}

pub async fn fetch_with(start: Url, max_bytes: usize, opts: Options<'_>) -> AppResult<Vec<u8>> {
    let mut url = start;
    for _ in 0..=MAX_REDIRECTS {
        let (_, host) = inspect(url.as_str())?;
        if let Some(allowed) = opts.allowed_hosts {
            if !allowed.contains(&host.as_str()) {
                return Err(AppError::Download(format!("{host} no está en la lista de dominios permitidos")));
            }
        }
        let addr = resolve_public(&url).await?;
        let client = client_for(&host, addr)?;
        let req = match opts.json_body {
            Some(body) => {
                client.post(url.clone()).header(reqwest::header::CONTENT_TYPE, "application/json").body(body.to_owned())
            }
            None => client.get(url.clone()),
        };
        let mut resp =
            req.send().await.map_err(|e| AppError::Download(format!("no se pudo conectar con {host}: {e}")))?;
        let status = resp.status();
        if status.is_redirection() && status != StatusCode::NOT_MODIFIED {
            let loc = resp
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .ok_or_else(|| AppError::Download("redirección sin destino".into()))?;
            url = next_hop(&url, loc)?;
            continue;
        }
        if !status.is_success() {
            // Un trozo del cuerpo ayuda a diagnosticar (p. ej. errores de GraphQL); se sanea.
            let mut detail = String::new();
            if let Ok(Some(chunk)) = resp.chunk().await {
                let text = String::from_utf8_lossy(chunk.get(..chunk.len().min(300)).unwrap_or_default());
                detail = text.chars().filter(|c| !c.is_control()).collect();
            }
            let sep = if detail.is_empty() { "" } else { ": " };
            return Err(AppError::Download(format!("{host} respondió {status}{sep}{detail}")));
        }
        if resp.content_length().is_some_and(|n| n > max_bytes as u64) {
            return Err(AppError::Download("el archivo supera el tamaño máximo".into()));
        }
        let mut out = Vec::new();
        while let Some(chunk) =
            resp.chunk().await.map_err(|e| AppError::Download(format!("descarga interrumpida: {e}")))?
        {
            if out.len() + chunk.len() > max_bytes {
                return Err(AppError::Download("el archivo supera el tamaño máximo".into()));
            }
            out.extend_from_slice(&chunk);
        }
        return Ok(out);
    }
    Err(AppError::Download(format!("demasiadas redirecciones (máximo {MAX_REDIRECTS})")))
}

#[cfg(test)]
#[path = "download_tests.rs"]
mod tests;
