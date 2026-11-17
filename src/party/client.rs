//! Cliente de una sala: conecta, se autentica y aplica lo que reparte el anfitrión.

use super::code::Invite;
use super::proto::Msg;
use super::session::{self, Event};
use super::{lock, Local, Me, PeerPos, PeerState, Roster, Status};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{timeout, Duration};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Ctx {
    pub roster: Arc<Mutex<Roster>>,
    pub changed: broadcast::Sender<()>,
    pub generation: u64,
}

async fn connect(invite: &Invite) -> Result<(TcpStream, super::secure::Channel), String> {
    let target = (invite.host.as_str(), invite.port);
    let mut stream = timeout(CONNECT_TIMEOUT, TcpStream::connect(target))
        .await
        .map_err(|_| "El anfitrión no responde (5 s).".to_string())?
        .map_err(|e| format!("No se pudo conectar con el anfitrión: {e}"))?;
    let chan = timeout(CONNECT_TIMEOUT, super::secure::initiate(&mut stream, &invite.secret))
        .await
        .map_err(|_| "El anfitrión no completó el saludo a tiempo.".to_string())?
        .map_err(|e| e.to_string())?;
    Ok((stream, chan))
}

/// Aplica un mensaje del anfitrión. `Err` = hay que cortar con ese motivo.
fn apply(roster: &Mutex<Roster>, generation: u64, msg: Msg) -> Result<(), String> {
    let mut r = lock(roster);
    if r.generation != generation {
        return Err("Sesión sustituida.".into());
    }
    match msg {
        Msg::Welcome { .. } => {}
        Msg::Peer { id, name, color } => {
            r.peers.insert(id, PeerState { name, color, pos: None });
        }
        Msg::PeerPos { id, map, x, y, z, heading } => {
            if let Some(p) = r.peers.get_mut(&id) {
                p.pos = Some(PeerPos { map, x, y, z, heading });
            }
        }
        Msg::PeerPing { id, map, x, z } => {
            if let Some(color) = r.peers.get(&id).map(|p| p.color) {
                r.add_ping(color, map, x, z);
            }
        }
        Msg::PeerLeft { id } => {
            r.peers.remove(&id);
        }
        Msg::Closed { reason } => return Err(reason.text().into()),
        _ => return Err("El anfitrión envió un mensaje no válido.".into()),
    }
    Ok(())
}

pub async fn run(invite: Invite, me: Me, ctx: Ctx, mut local: mpsc::Receiver<Local>) {
    let Ctx { roster, changed, generation } = ctx;
    let notify = || {
        let _ = changed.send(());
    };
    let set_status = |s: Status| {
        let mut r = lock(&roster);
        // Si el usuario ya salió o abrió otra sala, no pisar su estado.
        if r.generation == generation {
            r.status = s;
            if !matches!(r.status, Status::Connected) {
                r.peers.clear();
            }
        }
    };
    let (stream, chan) = match connect(&invite).await {
        Ok(v) => v,
        Err(e) => {
            set_status(Status::Error(e));
            return notify();
        }
    };
    drop(invite);
    let (tx, mut rx) = session::spawn(stream, chan);
    let _ = tx.send(Msg::Hello { name: me.name, color: me.color }).await;
    set_status(Status::Connected);
    notify();
    let outcome: Option<String> = loop {
        tokio::select! {
            ev = rx.recv() => match ev {
                Some(Event::Msg(m)) => match apply(&roster, generation, m) {
                    Ok(()) => notify(),
                    Err(why) => break Some(why),
                },
                Some(Event::Ended(why)) => break Some(why),
                None => break Some("Conexión cerrada.".into()),
            },
            cmd = local.recv() => match cmd {
                Some(Local::Send(m)) => { let _ = tx.try_send(m); }
                Some(Local::Kick(_)) => {}
                None | Some(Local::Close) => {
                    let _ = tx.send(Msg::Bye).await;
                    break None;
                }
            },
        }
    };
    if let Some(why) = outcome {
        set_status(Status::Error(why));
    }
    notify();
}
