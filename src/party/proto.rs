//! Mensajes del party (dentro del canal cifrado) y validación campo a campo.

use crate::config::PALETTE_SIZE;
use crate::maps::manifest::{is_unsafe_char, is_valid_id};
use crate::msg::UserMsg;
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
    pub fn msg(self) -> UserMsg {
        match self {
            Self::Full => UserMsg::new("roomFull", "The room is full (8 players maximum)."),
            Self::Kicked => UserMsg::new("kicked", "The host removed you from the room."),
            Self::RateLimit => UserMsg::new("rateLimited", "Disconnected for sending too many messages."),
            Self::HostClosed => UserMsg::new("roomClosed", "The host closed the room."),
            Self::Protocol => UserMsg::new("protocolError", "Disconnected after an invalid message."),
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
        return Err("the name must be 1 to 24 characters, with no control characters".into());
    }
    Ok(())
}

fn coords(vals: &[f64]) -> Result<(), String> {
    if vals.iter().all(|v| v.is_finite() && v.abs() <= MAX_COORD) {
        Ok(())
    } else {
        Err("invalid coordinate".into())
    }
}

fn place(map: &str, vals: &[f64]) -> Result<(), String> {
    if !is_valid_id(map) {
        return Err("invalid map id".into());
    }
    coords(vals)
}

fn heading(h: f64) -> Result<(), String> {
    if h.is_finite() && (0.0..360.0).contains(&h) {
        Ok(())
    } else {
        Err("invalid heading".into())
    }
}

fn color(c: u8) -> Result<(), String> {
    if c < PALETTE_SIZE {
        Ok(())
    } else {
        Err("colour outside the palette".into())
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
