//! Party propio: un anfitrión y hasta 8 compañeros por TCP cifrado con Noise.

pub mod client;
pub mod code;
pub mod frame;
pub mod host;
pub mod proto;
pub mod secure;
pub mod session;

use crate::error::{AppError, AppResult};
use proto::Msg;
use std::collections::BTreeMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;
use tokio::sync::{broadcast, mpsc};
use zeroize::Zeroizing;

pub const MAX_PEERS: usize = 8;
pub const PING_TTL_SECS: u64 = 8;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Idle,
    Hosting,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct PeerPos {
    pub map: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub heading: f64,
}

#[derive(Debug, Clone)]
pub struct PeerState {
    pub name: String,
    pub color: u8,
    pub pos: Option<PeerPos>,
}

#[derive(Debug, Clone)]
pub struct PingState {
    pub color: u8,
    pub map: String,
    pub x: f64,
    pub z: f64,
    pub at: Instant,
}

/// Estado del party compartido con la vista. Las posiciones solo viven en memoria.
#[derive(Debug)]
pub struct Roster {
    /// Cambia con cada sala o conexión: una tarea antigua no puede pisar el estado de la nueva.
    pub generation: u64,
    pub status: Status,
    pub is_host: bool,
    pub code: Option<Zeroizing<String>>,
    pub peers: BTreeMap<u8, PeerState>,
    pub pings: Vec<PingState>,
}

impl Roster {
    fn reset(&mut self, status: Status) {
        self.status = status;
        self.is_host = false;
        self.code = None; // Zeroizing borra el código de memoria al soltarlo.
        self.peers.clear();
        self.pings.clear();
    }

    pub fn add_ping(&mut self, color: u8, map: String, x: f64, z: f64) {
        self.pings.retain(|p| p.at.elapsed().as_secs() < PING_TTL_SECS);
        if self.pings.len() >= 32 {
            self.pings.remove(0);
        }
        self.pings.push(PingState { color, map, x, z, at: Instant::now() });
    }
}

pub fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Órdenes locales hacia la tarea del party.
#[derive(Debug)]
pub enum Local {
    Send(Msg),
    Kick(u8),
    Close,
}

pub struct Me {
    pub name: String,
    pub color: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Interface {
    pub name: String,
    pub ip: IpAddr,
}

pub fn interfaces() -> Vec<Interface> {
    if_addrs::get_if_addrs()
        .map(|list| {
            list.into_iter().filter(|i| !i.is_loopback()).map(|i| Interface { ip: i.ip(), name: i.name }).collect()
        })
        .unwrap_or_default()
}

pub struct Party {
    pub roster: Arc<Mutex<Roster>>,
    changed: broadcast::Sender<()>,
    cmd: Option<mpsc::Sender<Local>>,
}

impl Party {
    pub fn new(changed: broadcast::Sender<()>) -> Self {
        let roster = Roster {
            generation: 0,
            status: Status::Idle,
            is_host: false,
            code: None,
            peers: BTreeMap::new(),
            pings: Vec::new(),
        };
        Self { roster: Arc::new(Mutex::new(roster)), changed, cmd: None }
    }

    fn start(&mut self) -> (mpsc::Receiver<Local>, u64) {
        self.leave();
        let (tx, rx) = mpsc::channel(64);
        self.cmd = Some(tx);
        let mut r = lock(&self.roster);
        r.generation = r.generation.wrapping_add(1);
        (rx, r.generation)
    }

    /// Abre la sala. El puerto solo está abierto mientras la sala existe.
    pub fn host(&mut self, bind_ip: IpAddr, advertise: &str, port: u16, me: Me) -> AppResult<()> {
        let psk = code::new_secret().map_err(AppError::Internal)?;
        let invite = code::Invite { host: advertise.trim().to_owned(), port, secret: psk.clone() };
        let code = code::encode(&invite).map_err(AppError::bad)?;
        let std_listener = std::net::TcpListener::bind(SocketAddr::new(bind_ip, port)).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                AppError::bad(format!("el puerto {port} ya está en uso; elige otro en Ajustes"))
            } else {
                AppError::bad(format!("no se pudo escuchar en {bind_ip}:{port} ({e})"))
            }
        })?;
        std_listener.set_nonblocking(true)?;
        let listener = tokio::net::TcpListener::from_std(std_listener)?;
        let (rx, generation) = self.start();
        {
            let mut r = lock(&self.roster);
            r.reset(Status::Hosting);
            r.is_host = true;
            r.code = Some(code);
        }
        let ctx = host::Ctx { roster: self.roster.clone(), changed: self.changed.clone(), psk, me, generation };
        tokio::spawn(host::run(listener, ctx, rx));
        let _ = self.changed.send(());
        Ok(())
    }

    pub fn join(&mut self, code_text: &str, me: Me) -> AppResult<()> {
        let invite = code::decode(code_text).map_err(AppError::bad)?;
        let (rx, generation) = self.start();
        lock(&self.roster).reset(Status::Connecting);
        let ctx = client::Ctx { roster: self.roster.clone(), changed: self.changed.clone(), generation };
        tokio::spawn(client::run(invite, me, ctx, rx));
        let _ = self.changed.send(());
        Ok(())
    }

    pub fn leave(&mut self) {
        if let Some(tx) = self.cmd.take() {
            let _ = tx.try_send(Local::Close);
        }
        lock(&self.roster).reset(Status::Idle);
        let _ = self.changed.send(());
    }

    pub fn send(&self, msg: Msg) {
        if let Some(tx) = &self.cmd {
            let _ = tx.try_send(Local::Send(msg));
        }
    }

    pub fn kick(&self, id: u8) {
        if let Some(tx) = &self.cmd {
            let _ = tx.try_send(Local::Kick(id));
        }
    }
}
