//! Estado central de la app y acciones. Toda la lógica vive aquí o en los módulos de dominio.

use crate::capture::orient;
use crate::capture::parser::Vec3;
use crate::capture::watcher::{self, CaptureEvent, WatchGuard};
use crate::config::{self, Config, Paths};
use crate::error::{AppError, AppResult};
use crate::maps::import::Importer;
use crate::maps::pack::{Pack, PackStore};
use crate::party::{lock, proto::Msg, Me, Party};
use crate::protocol::SettingsInput;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use zeroize::Zeroizing;

#[derive(Debug, Clone)]
pub struct LastCapture {
    pub pos: Vec3,
    pub heading: f64,
    pub dir: (f64, f64),
    pub file_name: String,
    pub taken_at: String,
    pub raid_hours: f64,
}

#[derive(Default)]
pub struct Raid {
    pub trail: VecDeque<Vec3>,
    pub last: Option<LastCapture>,
    pub selected: Option<String>,
    /// Extracciones marcadas como disponibles, por mapa.
    pub available: HashMap<String, HashSet<String>>,
}

pub struct Inner {
    pub config: Config,
    pub store: PackStore,
    pub importer: Importer,
    pub raid: Raid,
    pub watch_folder: Option<PathBuf>,
    pub watch_error: Option<String>,
}

pub struct App {
    pub paths: Paths,
    pub token: Zeroizing<String>,
    pub port: u16,
    pub dev: bool,
    pub changed: broadcast::Sender<()>,
    pub inner: Mutex<Inner>,
    pub party: Mutex<Party>,
    pub tarkovdev: Mutex<crate::tarkovdev::sync::SyncStatus>,
    pub points: Mutex<crate::points::PointStore>,
    pub game: Mutex<crate::gamelog::GameStatus>,
    /// Estado del mercado y catálogo cargado (si lo hay).
    pub market: Mutex<(crate::market::MarketStatus, Option<Arc<crate::market::Catalog>>)>,
    capture_tx: mpsc::Sender<CaptureEvent>,
    watch_guard: Mutex<Option<WatchGuard>>,
}

impl App {
    pub fn new(paths: Paths, config: Config, token: Zeroizing<String>, port: u16, dev: bool) -> Arc<Self> {
        let (changed, _) = broadcast::channel(64);
        let (capture_tx, mut capture_rx) = mpsc::channel(64);
        let inner = Inner {
            store: PackStore::load(&paths.maps_dir),
            importer: Importer::new(&paths.staging_dir),
            config,
            raid: Raid::default(),
            watch_folder: None,
            watch_error: None,
        };
        let app = Arc::new(Self {
            party: Mutex::new(Party::new(changed.clone())),
            tarkovdev: Mutex::new(crate::tarkovdev::sync::SyncStatus::default()),
            points: Mutex::new(crate::points::PointStore::load(&paths.points_file)),
            game: Mutex::new(crate::gamelog::GameStatus::default()),
            market: Mutex::new(Default::default()),
            paths,
            token,
            port,
            dev,
            changed,
            inner: Mutex::new(inner),
            capture_tx,
            watch_guard: Mutex::new(None),
        });
        let worker = app.clone();
        tokio::spawn(async move {
            while let Some(ev) = capture_rx.recv().await {
                worker.on_capture(ev);
            }
        });
        app.restart_watcher();
        app.start_game_logs();
        app
    }

    pub fn notify(&self) {
        let _ = self.changed.send(());
    }

    pub fn restart_watcher(&self) {
        let dir = lock(&self.inner).config.effective_screenshots_dir();
        let mut guard = lock(&self.watch_guard);
        *guard = None;
        let (folder, error) = match &dir {
            None => (None, Some("No se encontró la carpeta Documentos; elige la carpeta de capturas.".into())),
            Some(d) => match watcher::spawn(d, self.capture_tx.clone()) {
                Ok(g) => {
                    *guard = Some(g);
                    (dir.clone(), None)
                }
                Err(e) => (dir.clone(), Some(e)),
            },
        };
        drop(guard);
        let mut inner = lock(&self.inner);
        inner.watch_folder = folder;
        inner.watch_error = error;
        drop(inner);
        self.notify();
    }

    pub(crate) fn active_pack(inner: &Inner) -> Option<Arc<Pack>> {
        inner.config.last_map.as_deref().and_then(|id| inner.store.get(id))
    }

    fn on_capture(&self, ev: CaptureEvent) {
        let c = ev.capture;
        let last = LastCapture {
            pos: c.pos,
            heading: orient::heading_deg(c.rot),
            dir: orient::horizontal_dir(c.rot),
            file_name: ev.file_name,
            taken_at: c.taken_at,
            raid_hours: c.raid_hours,
        };
        let mut inner = lock(&self.inner);
        let cap = inner.config.trail_length;
        let raid = &mut inner.raid;
        raid.trail.push_back(c.pos);
        while raid.trail.len() > cap {
            raid.trail.pop_front();
        }
        let msg = inner.config.last_map.clone().map(|map| Msg::Pos {
            map,
            x: c.pos.x,
            y: c.pos.y,
            z: c.pos.z,
            heading: last.heading,
        });
        inner.raid.last = Some(last);
        drop(inner);
        if let Some(m) = msg {
            lock(&self.party).send(m);
        }
        self.notify();
    }

    pub fn set_active_map(&self, id: &str) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        if !inner.store.contains(id) {
            return Err(AppError::NotFound);
        }
        inner.config.last_map = Some(id.to_owned());
        inner.raid.selected = None;
        config::save(&self.paths.config_file, &inner.config)?;
        drop(inner);
        self.notify();
        Ok(())
    }

    pub fn new_raid(&self) {
        lock(&self.inner).raid = Raid::default();
        self.notify();
    }

    pub fn select_extract(&self, id: Option<String>) {
        lock(&self.inner).raid.selected = id;
        self.notify();
    }

    pub fn toggle_extract(&self, id: String) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        let map = inner.config.last_map.clone().ok_or_else(|| AppError::bad("no hay mapa activo"))?;
        let set = inner.raid.available.entry(map).or_default();
        if !set.remove(&id) {
            set.insert(id);
        }
        drop(inner);
        self.notify();
        Ok(())
    }

    pub fn update_settings(&self, s: SettingsInput) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        let mut next = inner.config.clone();
        let dir = s.screenshots_dir.trim();
        next.screenshots_dir = (!dir.is_empty()).then(|| PathBuf::from(dir));
        next.trail_length = s.trail_length;
        next.player_name = s.player_name.trim().to_owned();
        next.player_color = s.player_color;
        next.party_port = s.party_port;
        next.validate()?;
        config::save(&self.paths.config_file, &next)?;
        let folder_changed = next.screenshots_dir != inner.config.screenshots_dir;
        while inner.raid.trail.len() > next.trail_length {
            inner.raid.trail.pop_front();
        }
        inner.config = next;
        drop(inner);
        if folder_changed {
            self.restart_watcher();
        }
        self.notify();
        Ok(())
    }

    pub fn ack_share_warning(&self) -> AppResult<()> {
        let mut inner = lock(&self.inner);
        inner.config.share_warning_ack = true;
        config::save(&self.paths.config_file, &inner.config)
    }

    fn me(&self) -> (Me, u16) {
        let inner = lock(&self.inner);
        (Me { name: inner.config.player_name.clone(), color: inner.config.player_color }, inner.config.party_port)
    }

    pub fn party_host(&self, bind_ip: &str, advertise: &str) -> AppResult<()> {
        let ip = bind_ip.trim().parse().map_err(|_| AppError::bad("la interfaz de escucha no es una IP válida"))?;
        let (me, port) = self.me();
        lock(&self.party).host(ip, advertise, port, me)
    }

    pub fn party_join(&self, code: &str) -> AppResult<()> {
        let (me, _) = self.me();
        lock(&self.party).join(code, me)
    }

    /// Ping propio (Shift+clic): se pasa a coordenadas del juego para compartirlo.
    pub fn map_ping(&self, px: f64, py: f64) -> AppResult<()> {
        if !px.is_finite() || !py.is_finite() {
            return Err(AppError::bad("coordenadas no válidas"));
        }
        let inner = lock(&self.inner);
        let pack = Self::active_pack(&inner).ok_or_else(|| AppError::bad("no hay mapa activo"))?;
        let (x, z) = pack.projection.unproject(px, py);
        let color = inner.config.player_color;
        drop(inner);
        let map = pack.manifest.id.clone();
        let party = lock(&self.party);
        lock(&party.roster).add_ping(color, map.clone(), x, z);
        party.send(Msg::Ping { map, x, z });
        drop(party);
        self.notify();
        Ok(())
    }
}
