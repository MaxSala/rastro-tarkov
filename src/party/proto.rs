//! Mensajes del party (dentro del canal cifrado) y validación campo a campo.

use crate::config::PALETTE_SIZE;
use crate::maps::manifest::{is_unsafe_char, is_valid_id};
use serde::{Deserialize, Serialize};

const MAX_COORD: f64 = 100_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloseReason {
    Full,
    Kicked,
    RateLimit,
    HostClosed,
    Protocol,
}

impl CloseReason {
    pub fn text(self) -> &'static str {
        match self {
            Self::Full => "La sala está llena (máximo 8).",
            Self::Kicked => "El anfitrión te ha expulsado.",
            Self::RateLimit => "Conexión cortada por enviar demasiados mensajes.",
            Self::HostClosed => "El anfitrión ha cerrado la sala.",
            Self::Protocol => "Conexión cortada por un mensaje no válido.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "camelCase", deny_unknown_fields)]
pub enum Msg {
    // Cliente → anfitrión (el anfitrión también los genera para sí mismo).
    Hello { name: String, color: u8 },
    Pos { map: String, x: f64, y: f64, z: f64, heading: f64 },
    Ping { map: String, x: f64, z: f64 },
    Beat,
    Bye,
    // Anfitrión → cliente. `id` lo asigna el anfitrión según la conexión autenticada.
    Welcome { you: u8 },
    Peer { id: u8, name: String, color: u8 },
    PeerPos { id: u8, map: String, x: f64, y: f64, z: f64, heading: f64 },
    PeerPing { id: u8, map: String, x: f64, z: f64 },
    PeerLeft { id: u8 },
    Closed { reason: CloseReason },
}

pub fn validate_name(name: &str) -> Result<(), String> {
    let n = name.chars().count();
    if n == 0 || n > 24 || name.chars().any(is_unsafe_char) || name.trim() != name {
        return Err("el nombre debe tener entre 1 y 24 caracteres, sin caracteres de control".into());
    }
    Ok(())
}

fn coords(vals: &[f64]) -> Result<(), String> {
    if vals.iter().all(|v| v.is_finite() && v.abs() <= MAX_COORD) {
        Ok(())
    } else {
        Err("coordenada no válida".into())
    }
}

fn place(map: &str, vals: &[f64]) -> Result<(), String> {
    if !is_valid_id(map) {
        return Err("id de mapa no válido".into());
    }
    coords(vals)
}

fn heading(h: f64) -> Result<(), String> {
    if h.is_finite() && (0.0..360.0).contains(&h) {
        Ok(())
    } else {
        Err("rumbo no válido".into())
    }
}

fn color(c: u8) -> Result<(), String> {
    if c < PALETTE_SIZE {
        Ok(())
    } else {
        Err("color fuera de la paleta".into())
    }
}

impl Msg {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Hello { name, color: c } | Self::Peer { name, color: c, .. } => {
                validate_name(name)?;
                color(*c)
            }
            Self::Pos { map, x, y, z, heading: h } | Self::PeerPos { map, x, y, z, heading: h, .. } => {
                place(map, &[*x, *y, *z])?;
                heading(*h)
            }
            Self::Ping { map, x, z } | Self::PeerPing { map, x, z, .. } => place(map, &[*x, *z]),
            Self::Beat | Self::Bye | Self::Welcome { .. } | Self::PeerLeft { .. } | Self::Closed { .. } => Ok(()),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self).map_err(|e| e.to_string())
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let msg: Self = serde_json::from_slice(bytes).map_err(|_| "mensaje desconocido".to_string())?;
        msg.validate()?;
        Ok(msg)
    }
}

#[cfg(test)]
#[path = "proto_tests.rs"]
mod tests;
