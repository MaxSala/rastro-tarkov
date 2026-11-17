use super::*;

#[test]
fn only_https_without_credentials() {
    assert!(inspect("https://example.com/pack.zip").is_ok());
    for bad in ["http://example.com/a.zip", "file:///etc/passwd", "ftp://x/y", "https://u:p@example.com/", "nota url"] {
        assert!(inspect(bad).is_err(), "{bad}");
    }
}

#[test]
fn internal_ips_are_forbidden() {
    for ip in [
        "127.0.0.1",
        "10.1.2.3",
        "172.16.0.1",
        "192.168.1.1",
        "169.254.169.254",
        "100.100.100.200",
        "0.0.0.0",
        "255.255.255.255",
        "::1",
        "fe80::1",
        "fd00:ec2::254",
        "::ffff:127.0.0.1",
        "::ffff:169.254.169.254",
        "64:ff9b::a9fe:a9fe",
    ] {
        assert!(is_forbidden_ip(ip.parse().expect("ip")), "{ip}");
    }
    for ip in ["1.1.1.1", "93.184.216.34", "2606:4700:4700::1111"] {
        assert!(!is_forbidden_ip(ip.parse().expect("ip")), "{ip}");
    }
}

#[tokio::test]
async fn ssrf_to_loopback_and_metadata_is_blocked_before_connecting() {
    for raw in [
        "https://127.0.0.1/a.zip",
        "https://169.254.169.254/latest/meta-data",
        "https://[::1]/x",
        "https://localhost/x",
    ] {
        let (url, _) = inspect(raw).expect("forma válida");
        assert!(resolve_public(&url).await.is_err(), "{raw}");
        // `fetch` valida antes de abrir ninguna conexión.
        assert!(fetch(url, 1024).await.is_err(), "{raw}");
    }
}

#[tokio::test]
async fn redirect_hops_are_revalidated() {
    let start = Url::parse("https://example.com/pack.zip").expect("url");
    // Redirección a http o con credenciales: rechazada en la forma.
    assert!(next_hop(&start, "http://example.com/x").is_err());
    assert!(next_hop(&start, "https://a:b@example.com/x").is_err());
    // Redirección a metadatos de nube: la forma es válida pero la IP no.
    let hop = next_hop(&start, "https://169.254.169.254/latest").expect("forma válida");
    assert!(resolve_public(&hop).await.is_err());
    let rel = next_hop(&start, "/otra.zip").expect("relativa");
    assert_eq!(rel.as_str(), "https://example.com/otra.zip");
}

#[tokio::test]
async fn host_allowlist_is_enforced_before_connecting() {
    let (url, _) = inspect("https://example.com/x.zip").expect("forma válida");
    let opts = Options { json_body: None, allowed_hosts: Some(&["api.tarkov.dev"]) };
    let err = fetch_with(url, 1024, opts).await.expect_err("dominio fuera de la lista");
    assert!(err.to_string().contains("lista de dominios permitidos"), "{err}");
}
