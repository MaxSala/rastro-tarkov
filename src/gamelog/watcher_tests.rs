use super::*;
use std::io::Write;

fn tmp() -> PathBuf {
    let d = std::env::temp_dir().join(format!("rastro-logs-{}", crate::maps::import::random_hex(6).expect("rng")));
    std::fs::create_dir_all(&d).expect("mkdir");
    d
}

#[test]
fn picks_newest_session_and_relevant_files() {
    let d = tmp();
    for s in ["log_2026.11.13_20-00-00_1.3.0.2.48779", "log_2026.11.14_15-11-05_1.3.0.2.48779"] {
        std::fs::create_dir_all(d.join(s)).expect("mkdir");
    }
    let s = d.join("log_2026.11.14_15-11-05_1.3.0.2.48779");
    for f in ["application_000.log", "backend_000.log", "errors_000.log", "output_000.log"] {
        std::fs::write(s.join(format!("2026.11.14_15-11-05_1.3.0.2.48779 {f}")), "").expect("w");
    }
    assert_eq!(newest_session(&d), Some(s.clone()));
    assert_eq!(newest_session(&s), Some(s.clone()), "también vale apuntar a la sesión");
    assert_eq!(crate::gamelog::history::files(&s, "").len(), 2);
}

#[test]
fn tail_reads_only_new_complete_lines() {
    let d = tmp();
    let p = d.join("x application_000.log");
    std::fs::write(&p, "linea 1\nlinea 2\nmedia").expect("w");
    let mut t = Tail::open(p.clone(), false);
    let mut out = Vec::new();
    t.read_lines(&mut out);
    assert_eq!(out, vec!["linea 1", "linea 2"]);
    let mut f = std::fs::OpenOptions::new().append(true).open(&p).expect("open");
    f.write_all(b" linea\r\nlinea 4\n").expect("w");
    out.clear();
    t.read_lines(&mut out);
    assert_eq!(out, vec!["media linea", "linea 4"]);
    std::fs::write(&p, "nuevo\n").expect("truncar");
    out.clear();
    t.read_lines(&mut out);
    assert_eq!(out, vec!["nuevo"], "si el archivo se trunca se vuelve a empezar");
}

#[test]
fn catch_up_skips_partial_first_line() {
    let d = tmp();
    let p = d.join("x backend_000.log");
    let big = "a".repeat(usize::try_from(INITIAL_TAIL).expect("usize") + 10);
    std::fs::write(&p, format!("{big}\nultima\n")).expect("w");
    let mut t = Tail::open(p, true);
    let mut out = Vec::new();
    while {
        let n = out.len();
        t.read_lines(&mut out);
        out.len() != n || t.offset < std::fs::metadata(&t.path).map(|m| m.len()).unwrap_or(0)
    } {}
    assert_eq!(out, vec!["ultima"]);
}
