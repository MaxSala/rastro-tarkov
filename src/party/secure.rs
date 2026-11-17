//! Negociación de versión y canal cifrado con Noise (crate `snow`, revisado y mantenido).
//!
//! Patrón Noise_NNpsk0_25519_ChaChaPoly_BLAKE2s: claves efímeras X25519 por sesión,
//! autenticadas con el secreto de 256 bits del código de invitación. Cada dirección usa su
//! propia clave y un contador de nonce implícito: una trama repetida, reordenada o
//! manipulada no descifra y la conexión se corta.

use super::frame::{read_frame, write_frame, MAX_FRAME};
use super::proto::Msg;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const PROTOCOL_VERSION: u16 = 1;
const MAGIC: &[u8; 4] = b"RSTP";
const PATTERN: &str = "Noise_NNpsk0_25519_ChaChaPoly_BLAKE2s";
const TAG_LEN: usize = 16;

#[derive(Debug, thiserror::Error)]
pub enum HsError {
    #[error("El anfitrión usa la versión {theirs} del protocolo y tú la {ours}. Actualizad los dos a la misma versión de Rastro.")]
    Version { ours: u16, theirs: u16 },
    #[error("Código incorrecto o la sala ya no existe.")]
    BadKey,
    #[error("El otro extremo no habla el protocolo de Rastro.")]
    NotRastro,
    #[error("Error de conexión: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error criptográfico interno.")]
    Crypto,
}

fn prologue() -> Vec<u8> {
    let mut p = MAGIC.to_vec();
    p.extend_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    p
}

fn builder<'a>(psk: &'a [u8; 32], prologue: &'a [u8]) -> Result<snow::Builder<'a>, HsError> {
    let params = PATTERN.parse().map_err(|_| HsError::Crypto)?;
    snow::Builder::new(params).psk(0, psk).and_then(|b| b.prologue(prologue)).map_err(|_| HsError::Crypto)
}

pub struct Channel {
    transport: snow::TransportState,
}

impl Channel {
    pub fn seal(&mut self, msg: &Msg) -> Result<Vec<u8>, String> {
        let plain = msg.encode()?;
        if plain.len() + TAG_LEN > MAX_FRAME {
            return Err("mensaje demasiado grande".into());
        }
        let mut out = vec![0u8; plain.len() + TAG_LEN];
        let n = self.transport.write_message(&plain, &mut out).map_err(|_| "cifrado".to_string())?;
        out.truncate(n);
        Ok(out)
    }

    pub fn open(&mut self, frame: &[u8]) -> Result<Msg, String> {
        let mut plain = vec![0u8; frame.len()];
        let n =
            self.transport.read_message(frame, &mut plain).map_err(|_| "trama no auténtica o repetida".to_string())?;
        Msg::decode(plain.get(..n).ok_or("longitud")?)
    }
}

/// Lado cliente: anuncia versión, luego handshake Noise como iniciador.
pub async fn initiate<S>(s: &mut S, psk: &[u8; 32]) -> Result<Channel, HsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    s.write_all(&prologue()).await?;
    s.flush().await?;
    let mut answer = [0u8; 3];
    s.read_exact(&mut answer).await.map_err(|_| HsError::NotRastro)?;
    let [status, hi, lo] = answer;
    let theirs = u16::from_be_bytes([hi, lo]);
    match status {
        0 => {}
        1 => return Err(HsError::Version { ours: PROTOCOL_VERSION, theirs }),
        _ => return Err(HsError::NotRastro),
    }
    let pro = prologue();
    let mut hs = builder(psk, &pro)?.build_initiator().map_err(|_| HsError::Crypto)?;
    let mut buf = vec![0u8; 256];
    let n = hs.write_message(&[], &mut buf).map_err(|_| HsError::Crypto)?;
    write_frame(s, buf.get(..n).ok_or(HsError::Crypto)?).await?;
    let reply = read_frame(s).await.map_err(|_| HsError::BadKey)?;
    hs.read_message(&reply, &mut buf).map_err(|_| HsError::BadKey)?;
    let transport = hs.into_transport_mode().map_err(|_| HsError::Crypto)?;
    Ok(Channel { transport })
}

/// Lado anfitrión: comprueba versión antes de nada y responde al handshake.
pub async fn respond<S>(s: &mut S, psk: &[u8; 32]) -> Result<Channel, HsError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut hello = [0u8; 6];
    s.read_exact(&mut hello).await?;
    let (magic, ver) = hello.split_at(4);
    if magic != MAGIC {
        return Err(HsError::NotRastro);
    }
    let theirs = u16::from_be_bytes([*ver.first().unwrap_or(&0), *ver.get(1).unwrap_or(&0)]);
    let [hi, lo] = PROTOCOL_VERSION.to_be_bytes();
    if theirs != PROTOCOL_VERSION {
        s.write_all(&[1, hi, lo]).await?;
        s.flush().await?;
        return Err(HsError::Version { ours: PROTOCOL_VERSION, theirs });
    }
    s.write_all(&[0, hi, lo]).await?;
    s.flush().await?;
    let pro = prologue();
    let mut hs = builder(psk, &pro)?.build_responder().map_err(|_| HsError::Crypto)?;
    let first = read_frame(s).await?;
    let mut buf = vec![0u8; 256];
    hs.read_message(&first, &mut buf).map_err(|_| HsError::BadKey)?;
    let n = hs.write_message(&[], &mut buf).map_err(|_| HsError::Crypto)?;
    write_frame(s, buf.get(..n).ok_or(HsError::Crypto)?).await?;
    let transport = hs.into_transport_mode().map_err(|_| HsError::Crypto)?;
    Ok(Channel { transport })
}

#[cfg(test)]
#[path = "secure_tests.rs"]
mod tests;
