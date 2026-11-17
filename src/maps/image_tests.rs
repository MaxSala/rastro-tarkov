use super::*;

fn png(w: u32, h: u32) -> Vec<u8> {
    let mut v = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
    v.extend_from_slice(&w.to_be_bytes());
    v.extend_from_slice(&h.to_be_bytes());
    v.extend_from_slice(&[8, 6, 0, 0, 0]);
    v
}

#[test]
fn png_dimensions_and_limits() {
    assert_eq!(check(Kind::Png, &png(1024, 512), (1024, 512)), Ok((1024, 512)));
    // Imagen sobredimensionada: se rechaza sin decodificar.
    assert!(check(Kind::Png, &png(50_000, 50_000), (50_000, 50_000)).is_err());
    assert!(check(Kind::Png, &png(1024, 512), (1000, 512)).is_err());
}

#[test]
fn magic_bytes_must_match_extension() {
    assert!(check(Kind::Jpeg, &png(10, 10), (10, 10)).is_err());
    assert!(check(Kind::Webp, &png(10, 10), (10, 10)).is_err());
    assert!(check(Kind::Png, b"<svg></svg>", (10, 10)).is_err());
}

#[test]
fn jpeg_and_webp_headers() {
    let jpeg = [0xFF, 0xD8, 0xFF, 0xE0, 0, 4, 0, 0, 0xFF, 0xC0, 0, 11, 8, 0x01, 0x00, 0x02, 0x00, 3];
    assert_eq!(jpeg_size(&jpeg), Some((512, 256)));
    let mut webp = b"RIFF\0\0\0\0WEBPVP8X".to_vec();
    webp.extend_from_slice(&[0; 8]);
    webp.extend_from_slice(&[0xFF, 0x03, 0x00, 0xFF, 0x01, 0x00]);
    assert_eq!(webp_size(&webp), Some((1024, 512)));
}

#[test]
fn svg_with_scripts_or_external_refs_is_rejected() {
    let bad = [
        r#"<svg><script>alert(1)</script></svg>"#,
        r#"<svg><SCRIPT src="x.js"/></svg>"#,
        r#"<svg onload="alert(1)"></svg>"#,
        r#"<svg><foreignObject><div/></foreignObject></svg>"#,
        r#"<!DOCTYPE svg [<!ENTITY x SYSTEM "file:///etc/passwd">]><svg>&x;</svg>"#,
        r#"<svg><image href="https://evil.example/a.png"/></svg>"#,
        r#"<svg><use xlink:href="file:///c:/x.svg#a"/></svg>"#,
        r#"<svg><image href="data:image/svg+xml;base64,PHN2Zz4="/></svg>"#,
        r#"<svg><a href="javascript:alert(1)">x</a></svg>"#,
        r#"<svg><style>rect{fill:url(http://evil/x)}</style></svg>"#,
    ];
    for svg in bad {
        assert!(check_svg(svg.as_bytes()).is_err(), "debería rechazar {svg}");
    }
}

#[test]
fn clean_svg_is_accepted() {
    let ok = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><defs><g id="a"/></defs><use href="#a"/><rect fill="url(#g)" width="10" height="10"/></svg>"##;
    assert_eq!(check_svg(ok.as_bytes()), Ok(()));
}
