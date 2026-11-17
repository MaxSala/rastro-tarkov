use super::*;
use zip::write::SimpleFileOptions;

fn build(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, data) in entries {
        w.start_file(*name, opts).expect("start");
        w.write_all(data).expect("write");
    }
    w.finish().expect("finish").into_inner()
}

fn tmp() -> std::path::PathBuf {
    let mut b = [0u8; 8];
    getrandom::fill(&mut b).expect("rng");
    let d = std::env::temp_dir().join(format!("rastro-zip-{}", data_encoding::HEXLOWER.encode(&b)));
    std::fs::create_dir_all(&d).expect("mkdir");
    d
}

#[test]
fn valid_pack_extracts() {
    let z = build(&[("manifest.json", b"{}"), ("map.png", b"x")]);
    let d = tmp();
    extract(&z, &d).expect("ok");
    assert!(d.join("manifest.json").exists() && d.join("map.png").exists());
}

#[test]
fn zip_slip_is_rejected() {
    for evil in ["../manifest.json", "..\\x.png", "/etc/passwd", "C:x.png", "sub/map.png"] {
        let z = build(&[("manifest.json", b"{}"), (evil, b"x")]);
        let d = tmp();
        assert!(extract(&z, &d).is_err(), "{evil}");
        assert!(!d.parent().expect("p").join("x.png").exists());
    }
}

#[test]
fn zip_bomb_is_rejected() {
    // 60 MB de ceros comprimen a ~60 KB: ratio enorme.
    let zeros = vec![0u8; 60 * 1024 * 1024];
    let z = build(&[("manifest.json", b"{}"), ("map.png", &zeros)]);
    assert!(z.len() < 1024 * 1024);
    let err = extract(&z, &tmp()).expect_err("debe fallar");
    assert!(err.to_string().contains("zip bomb"), "{err}");
}

#[test]
fn duplicates_extra_files_and_control_chars_are_rejected() {
    let cases: Vec<Vec<(&str, &[u8])>> = vec![
        vec![("manifest.json", b"{}"), ("MANIFEST.json", b"{}"), ("map.png", b"x")],
        vec![("manifest.json", b"{}"), ("map.png", b"x"), ("otra.png", b"x")],
        vec![("manifest.json", b"{}"), ("run.exe", b"x")],
        vec![("manifest.json", b"{}"), ("ma\u{7}p.png", b"x")],
        vec![("map.png", b"x")],
    ];
    for entries in cases {
        assert!(extract(&build(&entries), &tmp()).is_err());
    }
}

#[test]
fn not_a_zip_is_rejected() {
    assert!(extract(b"PK\x03\x04garbage", &tmp()).is_err());
}
