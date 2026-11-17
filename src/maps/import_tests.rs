use super::*;
use std::io::{Cursor, Write as _};

fn tmp_root() -> PathBuf {
    let d = std::env::temp_dir().join(format!("rastro-imp-{}", random_hex(8).expect("rng")));
    std::fs::create_dir_all(d.join("maps")).expect("mkdir");
    std::fs::create_dir_all(d.join("staging")).expect("mkdir");
    d
}

fn demo_manifest() -> serde_json::Value {
    serde_json::from_slice(DEMO_MANIFEST).expect("json")
}

fn zip_with(manifest: &serde_json::Value) -> Vec<u8> {
    let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let o = zip::write::SimpleFileOptions::default();
    w.start_file("manifest.json", o).expect("s");
    w.write_all(&serde_json::to_vec(manifest).expect("j")).expect("w");
    w.start_file("demo.svg", o).expect("s");
    w.write_all(DEMO_IMAGE).expect("w");
    w.finish().expect("f").into_inner()
}

#[test]
fn demo_installs_and_overwrite_needs_confirmation() {
    let root = tmp_root();
    let mut store = PackStore::load(&root.join("maps"));
    let mut imp = Importer::new(&root.join("staging"));
    let p = imp.stage_demo(&store).expect("demo válida");
    assert_eq!(p.id, "demo");
    assert!(!p.exists);
    imp.commit(&p.staging_id, false, &mut store).expect("instala");
    assert!(store.get("demo").is_some());

    let again = imp.stage_demo(&store).expect("demo");
    assert!(again.exists);
    let err = imp.commit(&again.staging_id, false, &mut store).expect_err("pide confirmación");
    assert!(matches!(err, AppError::PackExists(_)));
    imp.commit(&again.staging_id, true, &mut store).expect("reemplaza");
    assert_eq!(PackStore::load(&root.join("maps")).all().count(), 1);
}

#[test]
fn zip_pack_with_hash_verification() {
    let root = tmp_root();
    let store = PackStore::load(&root.join("maps"));
    let mut imp = Importer::new(&root.join("staging"));
    let z = zip_with(&demo_manifest());
    let sha = data_encoding::HEXLOWER.encode(&Sha256::digest(&z));
    let p = imp.stage_zip(&z, Some(&sha.to_uppercase()), &store).expect("ok");
    assert_eq!(p.sha256_verified, Some(true));
    assert!(imp.stage_zip(&z, Some(&"0".repeat(64)), &store).is_err());
    assert!(imp.stage_zip(&z, Some("no-hex"), &store).is_err());
}

#[test]
fn manifest_with_unknown_fields_is_rejected() {
    let root = tmp_root();
    let store = PackStore::load(&root.join("maps"));
    let mut imp = Importer::new(&root.join("staging"));
    let mut m = demo_manifest();
    m["script"] = serde_json::json!("<script>");
    assert!(imp.stage_zip(&zip_with(&m), None, &store).is_err());
    let mut m = demo_manifest();
    m["extracts"][0]["onclick"] = serde_json::json!("x");
    assert!(imp.stage_zip(&zip_with(&m), None, &store).is_err());
}

#[test]
fn manifest_with_huge_or_bad_strings_is_rejected() {
    let root = tmp_root();
    let store = PackStore::load(&root.join("maps"));
    let mut imp = Importer::new(&root.join("staging"));
    let cases: Vec<(&str, serde_json::Value)> = vec![
        ("/name", serde_json::json!("x".repeat(10_000))),
        ("/extracts/0/conditions", serde_json::json!("y".repeat(5_000))),
        ("/credits/author", serde_json::json!("a\u{0}b")),
        ("/id", serde_json::json!("../../etc")),
        ("/schema_version", serde_json::json!(99)),
        ("/image/width", serde_json::json!(100_000)),
    ];
    for (ptr, val) in cases {
        let mut m = demo_manifest();
        *m.pointer_mut(ptr).expect("ptr") = val;
        assert!(imp.stage_zip(&zip_with(&m), None, &store).is_err(), "{ptr}");
    }
}
