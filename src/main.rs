//! Rastro: segundo monitor pasivo para Escape from Tarkov.
//! Solo lee nombres de capturas; nunca toca el proceso ni la memoria del juego.

#![forbid(unsafe_code)]
// En Windows la versión release no abre consola; los logs se ven en Ajustes → Registro.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing))]

mod capture;
mod config;
mod desktop;
mod error;
mod gamelog;
mod logs;
mod maps;
mod market;
mod party;
mod points;
mod protocol;
mod server;
mod setup;
mod state;
mod state_import;
mod tarkovdev;
mod view;
mod view_extra;

use anyhow::{bail, Context};
use std::net::SocketAddr;
use std::time::Duration;
use zeroize::Zeroizing;

struct Args {
    dev: bool,
    open: bool,
    port: Option<u16>,
    /// Some(true) = ventana, Some(false) = navegador, None = lo que diga la configuración.
    window: Option<bool>,
}

fn parse_args() -> anyhow::Result<Args> {
    let mut args = Args { dev: false, open: true, port: None, window: None };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--dev" => args.dev = true,
            "--no-open" => args.open = false,
            "--browser" => args.window = Some(false),
            "--window" => args.window = Some(true),
            "--port" => {
                let v = it.next().context("--port necesita un número")?;
                args.port = Some(v.parse().context("--port debe ser un número entre 1024 y 65535")?);
            }
            "--help" | "-h" => {
                println!("Uso: rastro [--port N] [--window | --browser] [--no-open] [--dev]");
                std::process::exit(0);
            }
            other => bail!("opción desconocida: {other} (usa --help)"),
        }
    }
    Ok(args)
}

fn main() -> anyhow::Result<()> {
    let args = parse_args()?;
    let paths = config::Paths::discover()?;
    paths.ensure().context("no se pudo preparar el directorio de datos")?;
    logs::init(Some(&paths.log_file));
    let cfg = config::load(&paths.config_file);
    let port = args.port.unwrap_or(cfg.server_port);
    let window = args.open && !args.dev && args.window.unwrap_or(cfg.open_in_window);
    if port < 1024 {
        bail!("el puerto debe ser 1024 o superior");
    }

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let _guard = rt.enter();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match rt.block_on(tokio::net::TcpListener::bind(addr)) {
        Ok(l) => l,
        Err(e) => {
            let msg = if e.kind() == std::io::ErrorKind::AddrInUse {
                format!("El puerto {port} ya está en uso. Probablemente Rastro ya está abierto; si no, elige otro puerto en config.json o con --port.")
            } else {
                format!("No se pudo escuchar en {addr}: {e}")
            };
            tracing::error!("{msg}");
            if window {
                desktop::run(desktop::Page::Html(desktop::error_page(&msg)), String::new())?;
            }
            bail!(msg);
        }
    };

    let token = Zeroizing::new(maps::import::random_hex(32)?);
    let app = state::App::new(paths, cfg, token.clone(), port, args.dev);
    // Los pings caducan solos: refresca la vista mientras haya alguno.
    let ticker = app.clone();
    rt.spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(1));
        loop {
            tick.tick().await;
            if !party::lock(&party::lock(&ticker.party).roster).pings.is_empty() {
                ticker.notify();
            }
        }
    });
    let router = server::router(app);
    rt.spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            tracing::error!("el servidor se detuvo: {e}");
        }
    });
    tracing::info!("Rastro escuchando en http://{addr} (solo este equipo)");

    let web_port = if args.dev { server::DEV_PORT } else { port };
    let origin = format!("http://127.0.0.1:{web_port}");
    let url = format!("{origin}/#t={}", token.as_str());
    if window {
        // Bloquea hasta cerrar la ventana; al cerrarla termina el proceso.
        if let Err(e) = desktop::run(desktop::Page::Url(url.clone()), origin) {
            tracing::error!("no se pudo abrir la ventana ({e}); se usa el navegador");
            let _ = opener::open(&url);
        }
    } else if args.open && opener::open(&url).is_ok() {
        tracing::info!("Navegador abierto.");
    } else {
        // El token solo se muestra aquí, en la consola local, y cambia en cada arranque.
        println!("\nAbre esta dirección en el navegador (no la compartas):\n{url}\n");
    }
    // Modo navegador (o si la ventana falló): seguir hasta Ctrl+C o «Salir» en la app.
    rt.block_on(async {
        let _ = tokio::signal::ctrl_c().await;
    });
    Ok(())
}
