//! Bucle de una conexión autenticada: latido, timeout por inactividad y límite de tasa.

use super::frame::{read_frame, write_frame};
use super::proto::{CloseReason, Msg};
use super::secure::Channel;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio::time::{interval, sleep_until, Duration, Instant};

pub const HEARTBEAT: Duration = Duration::from_secs(5);
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(21);
const RATE_PER_SEC: f64 = 10.0;
const RATE_BURST: f64 = 30.0;

#[derive(Debug, PartialEq)]
pub enum Event {
    Msg(Msg),
    /// La conexión terminó; el texto explica por qué (en español, sin datos sensibles).
    Ended(String),
}

/// Cubo de fichas: 10 mensajes/s sostenidos, ráfagas de hasta 30.
struct Bucket {
    tokens: f64,
    last: Instant,
}

impl Bucket {
    fn take(&mut self) -> bool {
        let now = Instant::now();
        let refill = now.duration_since(self.last).as_secs_f64() * RATE_PER_SEC;
        self.tokens = (self.tokens + refill).min(RATE_BURST);
        self.last = now;
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Lanza la sesión. Cerrar el `Sender` devuelto termina la conexión tras vaciar la cola.
pub fn spawn<S>(stream: S, chan: Channel) -> (mpsc::Sender<Msg>, mpsc::Receiver<Event>)
where
    S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    let (out_tx, out_rx) = mpsc::channel(64);
    let (ev_tx, ev_rx) = mpsc::channel(64);
    tokio::spawn(async move {
        let why = run(stream, chan, out_rx, &ev_tx).await;
        let _ = ev_tx.send(Event::Ended(why)).await;
    });
    (out_tx, ev_rx)
}

async fn run<S>(stream: S, mut chan: Channel, mut out_rx: mpsc::Receiver<Msg>, ev_tx: &mpsc::Sender<Event>) -> String
where
    S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    let (mut reader, mut writer) = tokio::io::split(stream);
    // Lectura en su propia tarea: `read_exact` no es seguro ante cancelación en un select!.
    let (raw_tx, mut raw_rx) = mpsc::channel::<std::io::Result<Vec<u8>>>(16);
    let reader_task = tokio::spawn(async move {
        loop {
            let r = read_frame(&mut reader).await;
            let stop = r.is_err();
            if raw_tx.send(r).await.is_err() || stop {
                break;
            }
        }
    });
    let mut beat = interval(HEARTBEAT);
    let mut deadline = Instant::now() + IDLE_TIMEOUT;
    let mut bucket = Bucket { tokens: RATE_BURST, last: Instant::now() };
    let why = loop {
        tokio::select! {
            raw = raw_rx.recv() => {
                let frame = match raw {
                    Some(Ok(f)) => f,
                    Some(Err(_)) | None => break "Conexión cerrada.".to_string(),
                };
                deadline = Instant::now() + IDLE_TIMEOUT;
                if !bucket.take() {
                    send(&mut writer, &mut chan, &Msg::Closed { reason: CloseReason::RateLimit }).await;
                    break CloseReason::RateLimit.text().to_string();
                }
                match chan.open(&frame) {
                    Ok(Msg::Beat) => {}
                    Ok(msg) => {
                        if ev_tx.send(Event::Msg(msg)).await.is_err() {
                            break "Sesión terminada.".into();
                        }
                    }
                    Err(e) => break format!("Mensaje rechazado: {e}."),
                }
            }
            out = out_rx.recv() => match out {
                Some(msg) => {
                    if !send(&mut writer, &mut chan, &msg).await {
                        break "No se pudo enviar.".into();
                    }
                }
                None => break "Sesión cerrada.".into(),
            },
            _ = beat.tick() => {
                if !send(&mut writer, &mut chan, &Msg::Beat).await {
                    break "No se pudo enviar.".into();
                }
            }
            () = sleep_until(deadline) => break "Sin noticias del otro extremo en 21 s; conexión cortada.".into(),
        }
    };
    reader_task.abort();
    why
}

async fn send<W: AsyncWrite + Unpin>(w: &mut W, chan: &mut Channel, msg: &Msg) -> bool {
    match chan.seal(msg) {
        Ok(frame) => write_frame(w, &frame).await.is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
