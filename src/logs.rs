//! Registro: se guarda en memoria (visible en Ajustes → Registro) y en `rastro.log`.
//! En Windows la versión release no abre consola, así que esta es la forma de ver los logs.

use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};
use tracing_subscriber::fmt::MakeWriter;

const MAX_LINES: usize = 2000;
const MAX_FILE_BYTES: u64 = 5 * 1024 * 1024;

struct Sink {
    lines: Mutex<VecDeque<String>>,
    file: Option<Mutex<(File, u64)>>,
}

static SINK: OnceLock<Arc<Sink>> = OnceLock::new();

#[derive(Clone)]
struct Maker(Arc<Sink>);

struct LineWriter {
    sink: Arc<Sink>,
    buf: Vec<u8>,
}

impl Write for LineWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for LineWriter {
    fn drop(&mut self) {
        let text = String::from_utf8_lossy(&self.buf);
        let line = text.trim_end();
        if line.is_empty() {
            return;
        }
        eprintln!("{line}");
        if let Some(file) = &self.sink.file {
            let mut f = file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if f.1 < MAX_FILE_BYTES && writeln!(f.0, "{line}").is_ok() {
                f.1 += line.len() as u64 + 1;
            }
        }
        let mut lines = self.sink.lines.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if lines.len() >= MAX_LINES {
            lines.pop_front();
        }
        lines.push_back(line.to_owned());
    }
}

impl<'a> MakeWriter<'a> for Maker {
    type Writer = LineWriter;
    fn make_writer(&'a self) -> LineWriter {
        LineWriter { sink: self.0.clone(), buf: Vec::new() }
    }
}

/// Inicia el registro. El archivo se reescribe en cada arranque y se limita a 5 MB.
pub fn init(log_file: Option<&Path>) {
    let file = log_file.and_then(|p| File::create(p).ok()).map(|f| Mutex::new((f, 0)));
    let sink = Arc::new(Sink { lines: Mutex::new(VecDeque::new()), file });
    let _ = SINK.set(sink.clone());
    tracing_subscriber::fmt().with_target(false).with_ansi(false).with_writer(Maker(sink)).init();
}

/// Últimas líneas del registro (sin datos sensibles: nunca se registran token ni código).
pub fn lines() -> Vec<String> {
    SINK.get()
        .map(|s| s.lines.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().cloned().collect())
        .unwrap_or_default()
}
