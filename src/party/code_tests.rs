use super::*;

#[test]
fn round_trip_ipv4_ipv6_and_names() {
    for host in ["100.64.1.2", "fd7a:115c:a1e0::1", "mi-pc.tailnet.ts.net"] {
        let inv = Invite { host: host.into(), port: 47822, secret: new_secret().expect("rng") };
        let code = encode(&inv).expect("encode");
        assert!(code.starts_with("RST-"));
        let back = decode(&code.to_lowercase()).expect("decode");
        assert_eq!(back.host, host);
        assert_eq!(back.port, 47822);
        assert_eq!(*back.secret, *inv.secret);
    }
}

#[test]
fn rejects_bad_input_without_echoing_it() {
    let err = decode("RST-AAAA-SECRETO").err().expect("error");
    assert!(!err.text.contains("SECRETO") && err.arg.is_none());
    assert_eq!(err.code, "inviteInvalid");
    assert!(encode(&Invite { host: "bad host!".into(), port: 1, secret: Zeroizing::new([0; 32]) }).is_err());
}
