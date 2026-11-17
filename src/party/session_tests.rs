use super::super::secure::{initiate, respond};
use super::*;

async fn connected() -> ((mpsc::Sender<Msg>, mpsc::Receiver<Event>), (mpsc::Sender<Msg>, mpsc::Receiver<Event>)) {
    let (mut a, mut b) = tokio::io::duplex(1 << 16);
    let h = tokio::spawn(async move {
        let ch = respond(&mut b, &[3; 32]).await.expect("h");
        (b, ch)
    });
    let ca = initiate(&mut a, &[3; 32]).await.expect("c");
    let (b, cb) = h.await.expect("join");
    (spawn(a, ca), spawn(b, cb))
}

#[tokio::test]
async fn messages_flow_and_beats_are_hidden() {
    let ((tx, _rx_a), (_tx_b, mut rx_b)) = connected().await;
    let m = Msg::Ping { map: "demo".into(), x: 1.0, z: 2.0 };
    tx.send(m.clone()).await.expect("send");
    assert_eq!(rx_b.recv().await, Some(Event::Msg(m)));
}

#[tokio::test(start_paused = true)]
async fn heartbeat_timeout_cuts_silent_peer() {
    // Un extremo autenticado que deja de hablar: el otro corta a los 21 s.
    let (mut a, mut b) = tokio::io::duplex(1 << 16);
    let h = tokio::spawn(async move {
        let ch = respond(&mut b, &[4; 32]).await.expect("h");
        (b, ch)
    });
    let _silent = initiate(&mut a, &[4; 32]).await.expect("c");
    let (b, cb) = h.await.expect("join");
    let (_tx, mut rx) = spawn(b, cb);
    let started = Instant::now();
    match rx.recv().await {
        Some(Event::Ended(why)) => assert_eq!(why.code, "connectionLost", "{why}"),
        other => panic!("se esperaba corte, llegó {other:?}"),
    }
    assert!(started.elapsed() >= IDLE_TIMEOUT);
    drop(a);
}

#[tokio::test]
async fn message_flood_gets_cut() {
    let ((tx, _rx_a), (_tx_b, mut rx_b)) = connected().await;
    for _ in 0..200 {
        if tx.send(Msg::Ping { map: "demo".into(), x: 0.0, z: 0.0 }).await.is_err() {
            break;
        }
    }
    let mut ended = None;
    while let Some(ev) = rx_b.recv().await {
        if let Event::Ended(why) = ev {
            ended = Some(why);
            break;
        }
    }
    assert_eq!(ended.expect("debe cortar").code, "rateLimited");
}
