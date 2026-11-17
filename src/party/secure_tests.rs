use super::*;

async fn pair(psk_a: [u8; 32], psk_b: [u8; 32]) -> (Result<Channel, HsError>, Result<Channel, HsError>) {
    let (mut a, mut b) = tokio::io::duplex(4096);
    let host = tokio::spawn(async move { respond(&mut b, &psk_b).await });
    let client = initiate(&mut a, &psk_a).await;
    drop(a);
    (client, host.await.expect("join"))
}

#[tokio::test]
async fn encrypt_round_trip_both_directions() {
    let (c, h) = pair([7; 32], [7; 32]).await;
    let (mut c, mut h) = (c.expect("cliente"), h.expect("anfitrión"));
    let m = Msg::Pos { map: "demo".into(), x: 1.0, y: 2.0, z: 3.0, heading: 90.0 };
    let frame = c.seal(&m).expect("seal");
    assert!(!frame.windows(4).any(|w| w == b"demo"), "el texto no debe viajar en claro");
    assert_eq!(h.open(&frame).expect("open"), m);
    let back = h.seal(&Msg::Beat).expect("seal");
    assert_eq!(c.open(&back).expect("open"), Msg::Beat);
}

#[tokio::test]
async fn wrong_key_is_rejected_by_both_sides() {
    let (c, h) = pair([1; 32], [2; 32]).await;
    assert!(matches!(h, Err(HsError::BadKey)));
    assert!(matches!(c, Err(HsError::BadKey)));
}

#[tokio::test]
async fn replayed_or_tampered_frames_fail() {
    let (c, h) = pair([9; 32], [9; 32]).await;
    let (mut c, mut h) = (c.expect("c"), h.expect("h"));
    let f1 = c.seal(&Msg::Beat).expect("seal");
    h.open(&f1).expect("primera vez");
    assert!(h.open(&f1).is_err(), "repetición");
    let mut f2 = c.seal(&Msg::Bye).expect("seal");
    if let Some(b) = f2.first_mut() {
        *b ^= 1;
    }
    assert!(h.open(&f2).is_err(), "manipulación");
}

#[tokio::test]
async fn different_protocol_version_is_rejected_before_any_data() {
    let (mut a, mut b) = tokio::io::duplex(1024);
    let host = tokio::spawn(async move { respond(&mut b, &[0; 32]).await });
    let mut hello = MAGIC.to_vec();
    hello.extend_from_slice(&99u16.to_be_bytes());
    a.write_all(&hello).await.expect("w");
    let mut answer = [0u8; 3];
    a.read_exact(&mut answer).await.expect("r");
    assert_eq!(answer, [1, 0, PROTOCOL_VERSION as u8]);
    let err = host.await.expect("join").err().expect("rechazo");
    assert!(matches!(err, HsError::Version { theirs: 99, .. }));
    assert!(err.to_string().contains("versión"));
}
