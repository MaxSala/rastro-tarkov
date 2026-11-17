//! Código de invitación: host, puerto y secreto de 256 bits en base32 con guiones.

use data_encoding::BASE32_NOPAD;
use zeroize::Zeroizing;

const CODE_VERSION: u8 = 1;
const PREFIX: &str = "RST";

pub struct Invite {
    pub host: String,
    pub port: u16,
    pub secret: Zeroizing<[u8; 32]>,
}

fn valid_host(host: &str) -> bool {
    if host.parse::<std::net::IpAddr>().is_ok() {
        return true;
    }
    !host.is_empty()
        && host.len() <= 253
        && host.split('.').all(|label| {
            (1..=63).contains(&label.len())
                && label.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                && !label.starts_with('-')
                && !label.ends_with('-')
        })
}

pub fn new_secret() -> Result<Zeroizing<[u8; 32]>, String> {
    let mut s = Zeroizing::new([0u8; 32]);
    getrandom::fill(s.as_mut_slice()).map_err(|e| format!("aleatoriedad del sistema: {e}"))?;
    Ok(s)
}

pub fn encode(invite: &Invite) -> Result<Zeroizing<String>, String> {
    if !valid_host(&invite.host) {
        return Err("la dirección del anfitrión no es válida".into());
    }
    let host = invite.host.as_bytes();
    let mut raw = Zeroizing::new(Vec::with_capacity(4 + host.len() + 32));
    raw.push(CODE_VERSION);
    raw.push(u8::try_from(host.len()).map_err(|_| "dirección demasiado larga")?);
    raw.extend_from_slice(host);
    raw.extend_from_slice(&invite.port.to_be_bytes());
    raw.extend_from_slice(invite.secret.as_slice());
    let b32 = Zeroizing::new(BASE32_NOPAD.encode(&raw));
    let mut out = Zeroizing::new(String::from(PREFIX));
    for (i, ch) in b32.chars().enumerate() {
        if i % 4 == 0 {
            out.push('-');
        }
        out.push(ch);
    }
    Ok(out)
}

/// Mensajes de error genéricos: nunca incluyen el código ni partes del secreto.
pub fn decode(code: &str) -> Result<Invite, String> {
    let bad = || "El código de invitación no es válido. Cópialo de nuevo completo.".to_string();
    if code.len() > 600 {
        return Err(bad());
    }
    let cleaned: Zeroizing<String> = Zeroizing::new(
        code.trim()
            .to_ascii_uppercase()
            .trim_start_matches(PREFIX)
            .chars()
            .filter(|c| !matches!(c, '-' | ' ' | '\n' | '\r' | '\t'))
            .collect(),
    );
    let raw = Zeroizing::new(BASE32_NOPAD.decode(cleaned.as_bytes()).map_err(|_| bad())?);
    let (&ver, rest) = raw.split_first().ok_or_else(bad)?;
    if ver != CODE_VERSION {
        return Err("Este código es de otra versión de Rastro.".into());
    }
    let (&hlen, rest) = rest.split_first().ok_or_else(bad)?;
    let hlen = usize::from(hlen);
    if rest.len() != hlen + 2 + 32 {
        return Err(bad());
    }
    let host = std::str::from_utf8(rest.get(..hlen).ok_or_else(bad)?).map_err(|_| bad())?;
    if !valid_host(host) {
        return Err(bad());
    }
    let port_b: [u8; 2] = rest.get(hlen..hlen + 2).ok_or_else(bad)?.try_into().map_err(|_| bad())?;
    let mut secret = Zeroizing::new([0u8; 32]);
    secret.copy_from_slice(rest.get(hlen + 2..).ok_or_else(bad)?);
    Ok(Invite { host: host.to_owned(), port: u16::from_be_bytes(port_b), secret })
}

#[cfg(test)]
#[path = "code_tests.rs"]
mod tests;
