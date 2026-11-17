//! Anfitrión de la sala: acepta, autentica, reparte posiciones y expulsa.

use super::proto::{CloseReason, Msg};
use super::session::{self, Event};
use super::{lock, Local, Me, PeerPos, PeerState, Roster, Status, MAX_PEERS};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, OwnedSemaphorePermit, Semaphore};
use tokio::time::{timeout, Duration};
use zeroize::Zeroizing;

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_PENDING: usize = 4;
const HOST_ID: u8 = 0;

pub struct Ctx {
    pub roster: Arc<Mutex<Roster>>,
    pub changed: broadcast::Sender<()>,
    pub psk: Zeroizing<[u8; 32]>,
    pub me: Me,
    pub generation: u64,
}

struct Joined {
    tx: mpsc::Sender<Msg>,
    rx: mpsc::Receiver<Event>,
    name: String,
    color: u8,
}

/// Handshake + primer mensaje (hola) en 5 s; si algo falla, la conexión se descarta.
async fn authenticate(
    mut stream: TcpStream,
    psk: Zeroizing<[u8; 32]>,
    _permit: OwnedSemaphorePermit,
) -> Option<Joined> {
    let chan = timeout(HANDSHAKE_TIMEOUT, super::secure::respond(&mut stream, &psk)).await.ok()?.ok()?;
    drop(psk);
    let (tx, mut rx) = session::spawn(stream, chan);
    match timeout(HANDSHAKE_TIMEOUT, rx.recv()).await {
        Ok(Some(Event::Msg(Msg::Hello { name, color }))) => Some(Joined { tx, rx, name, color }),
        _ => None,
    }
}

struct Room {
    ctx: Ctx,
    peers: HashMap<u8, mpsc::Sender<Msg>>,
    next_id: u8,
    events: mpsc::Sender<(u8, Event)>,
}

impl Room {
    fn broadcast(&self, except: Option<u8>, msg: &Msg) {
        for (id, tx) in &self.peers {
            if Some(*id) != except {
                let _ = tx.try_send(msg.clone()); // un compañero lento pierde actualizaciones
            }
        }
    }

    fn notify(&self) {
        let _ = self.ctx.changed.send(());
    }

    fn next_free_id(&mut self) -> Option<u8> {
        for _ in 0..255 {
            self.next_id = self.next_id.checked_add(1).unwrap_or(1);
            if !self.peers.contains_key(&self.next_id) {
                return Some(self.next_id);
            }
        }
        None
    }

    fn admit(&mut self, j: Joined) {
        let id = match self.next_free_id() {
            Some(id) if self.peers.len() < MAX_PEERS => id,
            _ => {
                let _ = j.tx.try_send(Msg::Closed { reason: CloseReason::Full });
                return;
            }
        };
        let mut intro = vec![
            Msg::Welcome { you: id },
            Msg::Peer { id: HOST_ID, name: self.ctx.me.name.clone(), color: self.ctx.me.color },
        ];
        {
            let roster = lock(&self.ctx.roster);
            for (pid, p) in &roster.peers {
                intro.push(Msg::Peer { id: *pid, name: p.name.clone(), color: p.color });
                if let Some(pos) = &p.pos {
                    intro.push(Msg::PeerPos {
                        id: *pid,
                        map: pos.map.clone(),
                        x: pos.x,
                        y: pos.y,
                        z: pos.z,
                        heading: pos.heading,
                    });
                }
            }
        }
        for m in intro {
            let _ = j.tx.try_send(m);
        }
        self.broadcast(None, &Msg::Peer { id, name: j.name.clone(), color: j.color });
        self.peers.insert(id, j.tx);
        lock(&self.ctx.roster).peers.insert(id, PeerState { name: j.name, color: j.color, pos: None });
        let events = self.events.clone();
        let mut rx = j.rx;
        tokio::spawn(async move {
            while let Some(ev) = rx.recv().await {
                if events.send((id, ev)).await.is_err() {
                    break;
                }
            }
        });
        self.notify();
    }

    fn remove(&mut self, id: u8, reason: Option<CloseReason>) {
        if let Some(tx) = self.peers.remove(&id) {
            if let Some(reason) = reason {
                let _ = tx.try_send(Msg::Closed { reason });
            }
        }
        lock(&self.ctx.roster).peers.remove(&id);
        self.broadcast(None, &Msg::PeerLeft { id });
        self.notify();
    }

    fn on_peer(&mut self, id: u8, ev: Event) {
        if !self.peers.contains_key(&id) {
            return; // eventos rezagados de un compañero ya expulsado
        }
        let msg = match ev {
            Event::Msg(m) => m,
            Event::Ended(_) => return self.remove(id, None),
        };
        match msg {
            Msg::Pos { map, x, y, z, heading } => {
                if let Some(p) = lock(&self.ctx.roster).peers.get_mut(&id) {
                    p.pos = Some(PeerPos { map: map.clone(), x, y, z, heading });
                }
                self.broadcast(Some(id), &Msg::PeerPos { id, map, x, y, z, heading });
                self.notify();
            }
            Msg::Ping { map, x, z } => {
                let color = lock(&self.ctx.roster).peers.get(&id).map(|p| p.color);
                if let Some(color) = color {
                    lock(&self.ctx.roster).add_ping(color, map.clone(), x, z);
                }
                self.broadcast(Some(id), &Msg::PeerPing { id, map, x, z });
                self.notify();
            }
            Msg::Bye => self.remove(id, None),
            Msg::Beat => {}
            // Un segundo hola o mensajes reservados al anfitrión: fuera.
            _ => self.remove(id, Some(CloseReason::Protocol)),
        }
    }

    fn on_local(&mut self, cmd: Local) {
        match cmd {
            Local::Send(Msg::Pos { map, x, y, z, heading }) => {
                self.broadcast(None, &Msg::PeerPos { id: HOST_ID, map, x, y, z, heading });
            }
            Local::Send(Msg::Ping { map, x, z }) => {
                self.broadcast(None, &Msg::PeerPing { id: HOST_ID, map, x, z });
            }
            Local::Kick(id) => self.remove(id, Some(CloseReason::Kicked)),
            Local::Send(_) | Local::Close => {}
        }
    }
}

pub async fn run(listener: TcpListener, ctx: Ctx, mut local: mpsc::Receiver<Local>) {
    let pending = Arc::new(Semaphore::new(MAX_PENDING));
    let (join_tx, mut join_rx) = mpsc::channel::<Joined>(8);
    let (ev_tx, mut ev_rx) = mpsc::channel::<(u8, Event)>(256);
    let mut room = Room { ctx, peers: HashMap::new(), next_id: 0, events: ev_tx };
    loop {
        tokio::select! {
            acc = listener.accept() => {
                let Ok((stream, _addr)) = acc else { continue };
                // Más de 4 conexiones sin autenticar a la vez: se descarta sin leer nada.
                let Ok(permit) = pending.clone().try_acquire_owned() else { continue };
                let (psk, tx) = (room.ctx.psk.clone(), join_tx.clone());
                tokio::spawn(async move {
                    if let Some(j) = authenticate(stream, psk, permit).await {
                        let _ = tx.send(j).await;
                    }
                });
            }
            Some(j) = join_rx.recv() => room.admit(j),
            Some((id, ev)) = ev_rx.recv() => room.on_peer(id, ev),
            cmd = local.recv() => match cmd {
                None | Some(Local::Close) => break,
                Some(c) => room.on_local(c),
            },
        }
    }
    room.broadcast(None, &Msg::Closed { reason: CloseReason::HostClosed });
    room.peers.clear();
    let mut r = lock(&room.ctx.roster);
    if r.generation == room.ctx.generation {
        r.reset(Status::Idle);
    }
    drop(r);
    room.notify();
    // Al salir se sueltan el listener (puerto cerrado) y el secreto (Zeroizing).
}

#[cfg(test)]
#[path = "host_tests.rs"]
mod tests;
