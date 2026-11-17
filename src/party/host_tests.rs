use super::super::{Me, Party, Status};
use super::*;
use tokio::io::AsyncReadExt;
use tokio::time::sleep;

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0").expect("bind").local_addr().expect("addr").port()
}

fn me(name: &str) -> Me {
    Me { name: name.into(), color: 1 }
}

async fn wait_for<F: Fn() -> bool>(f: F) -> bool {
    for _ in 0..100 {
        if f() {
            return true;
        }
        sleep(Duration::from_millis(30)).await;
    }
    false
}

fn new_party() -> Party {
    Party::new(broadcast::channel(64).0)
}

async fn open_room() -> (Party, String, u16) {
    let mut host = new_party();
    let port = free_port();
    host.host("127.0.0.1".parse().expect("ip"), "127.0.0.1", port, me("Anfitrión")).expect("sala");
    let code = lock(&host.roster).code.as_ref().expect("código").to_string();
    (host, code, port)
}

#[tokio::test]
async fn join_exchange_positions_and_kick() {
    let (host, code, _) = open_room().await;
    let mut guest = new_party();
    guest.join(&code, me("Invitado")).expect("join");
    assert!(wait_for(|| lock(&guest.roster).status == Status::Connected && lock(&host.roster).peers.len() == 1).await);
    assert!(wait_for(|| lock(&guest.roster).peers.get(&0).is_some_and(|p| p.name == "Anfitrión")).await);

    guest.send(Msg::Pos { map: "demo".into(), x: 10.0, y: 0.0, z: -5.0, heading: 45.0 });
    assert!(wait_for(|| lock(&host.roster).peers.values().any(|p| p.pos.as_ref().is_some_and(|q| q.x == 10.0))).await);

    let id = *lock(&host.roster).peers.keys().next().expect("id");
    host.kick(id);
    assert!(wait_for(|| matches!(&lock(&guest.roster).status, Status::Error(e) if e.code == "kicked")).await);
    assert!(lock(&host.roster).peers.is_empty());
}

#[tokio::test]
async fn wrong_secret_is_rejected() {
    let (_host, code, port) = open_room().await;
    let forged = super::super::code::Invite { host: "127.0.0.1".into(), port, secret: Zeroizing::new([0u8; 32]) };
    let forged = super::super::code::encode(&forged).expect("code");
    assert_ne!(code, *forged);
    let mut guest = new_party();
    guest.join(&forged, me("Intruso")).expect("join");
    assert!(wait_for(|| matches!(&lock(&guest.roster).status, Status::Error(e) if e.code == "wrongCode")).await);
}

#[tokio::test]
async fn too_many_unauthenticated_connections_are_dropped() {
    let (_host, _code, port) = open_room().await;
    let mut idle = Vec::new();
    for _ in 0..MAX_PENDING {
        idle.push(TcpStream::connect(("127.0.0.1", port)).await.expect("conecta"));
    }
    sleep(Duration::from_millis(100)).await;
    let mut extra = TcpStream::connect(("127.0.0.1", port)).await.expect("conecta");
    let mut buf = [0u8; 1];
    let n = timeout(Duration::from_secs(2), extra.read(&mut buf)).await.expect("cierra rápido").unwrap_or(0);
    assert_eq!(n, 0, "la quinta conexión pendiente se cierra sin respuesta");
}

#[tokio::test]
async fn room_is_capped_at_eight_and_closing_frees_port() {
    let (mut host, code, port) = open_room().await;
    let mut guests = Vec::new();
    for i in 0..MAX_PEERS {
        let mut g = new_party();
        g.join(&code, me(&format!("J{i}"))).expect("join");
        guests.push(g);
        assert!(wait_for(|| lock(&host.roster).peers.len() == i + 1).await, "entra {i}");
    }
    let mut late = new_party();
    late.join(&code, me("Tarde")).expect("join");
    assert!(wait_for(|| matches!(&lock(&late.roster).status, Status::Error(e) if e.code == "roomFull")).await);

    host.leave();
    assert!(
        wait_for(|| guests
            .iter()
            .all(|g| matches!(&lock(&g.roster).status, Status::Error(e) if e.code == "roomClosed")))
        .await
    );
    assert!(wait_for(|| std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()).await, "puerto liberado");
}

#[tokio::test]
async fn reopening_quickly_keeps_the_new_room() {
    let (mut host, first_code, _) = open_room().await;
    host.leave();
    let port = free_port();
    host.host("127.0.0.1".parse().expect("ip"), "127.0.0.1", port, me("Anfitrión")).expect("segunda sala");
    sleep(Duration::from_millis(300)).await;
    let r = lock(&host.roster);
    assert_eq!(r.status, Status::Hosting, "la tarea antigua no debe resetear la sala nueva");
    let code = r.code.as_ref().expect("código nuevo").to_string();
    assert_ne!(code, first_code, "código nuevo en cada sala");
}
