//! Tramas con prefijo de longitud (u16, big-endian) y tope comprobado antes de reservar.

use std::io::{Error, ErrorKind};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const MAX_FRAME: usize = 16 * 1024;

pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R) -> std::io::Result<Vec<u8>> {
    let len = usize::from(r.read_u16().await?);
    if len == 0 || len > MAX_FRAME {
        return Err(Error::new(ErrorKind::InvalidData, "empty or oversized frame"));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).await?;
    Ok(buf)
}

pub async fn write_frame<W: AsyncWrite + Unpin>(w: &mut W, data: &[u8]) -> std::io::Result<()> {
    let len = u16::try_from(data.len())
        .ok()
        .filter(|l| usize::from(*l) <= MAX_FRAME && *l > 0)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "frame too large"))?;
    w.write_u16(len).await?;
    w.write_all(data).await?;
    w.flush().await
}

#[cfg(test)]
#[path = "frame_tests.rs"]
mod tests;
