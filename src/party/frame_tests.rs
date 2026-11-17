use super::*;

#[tokio::test]
async fn oversized_frame_is_rejected_before_allocating() {
    let (mut a, mut b) = tokio::io::duplex(64);
    // Cabecera que anuncia 60 000 bytes: se rechaza sin leer el cuerpo.
    a.write_u16(60_000).await.expect("w");
    assert!(read_frame(&mut b).await.is_err());
    assert!(write_frame(&mut a, &vec![0u8; MAX_FRAME + 1]).await.is_err());
}

#[tokio::test]
async fn round_trip() {
    let (mut a, mut b) = tokio::io::duplex(1024);
    write_frame(&mut a, b"hola").await.expect("w");
    assert_eq!(read_frame(&mut b).await.expect("r"), b"hola");
}
