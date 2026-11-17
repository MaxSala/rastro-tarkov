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
mod msg;
mod party;
mod points;
mod protocol;
mod server;
mod setup;
mod state;
mod state_import;
mod tarkovdev;
mod update;
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
                let v = it.next().context("--port needs a number")?;
                args.port = Some(v.parse().context("--port must be a number between 1024 and 65535")?);
            }
            "--help" | "-h" => {
                println!("Usage: rastro [--port N] [--window | --browser] [--no-open] [--dev]");
                std::process::exit(0);
            }
            other => bail!("unknown option: {other} (see --help)"),
        }
    }
    Ok(args)
}

fn main() -> anyhow::Result<()> {
    let args = parse_args()?;
    let paths = config::Paths::discover()?;
    paths.ensure().context("could not prepare the data folder")?;
    logs::init(Some(&paths.log_file));
    let cfg = config::load(&paths.config_file);
    let port = args.port.unwrap_or(cfg.server_port);
    let window = args.open && !args.dev && args.window.unwrap_or(cfg.open_in_window);
    if port < 1024 {
        bail!("the port must be 1024 or higher");
    }

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let _guard = rt.enter();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match rt.block_on(tokio::net::TcpListener::bind(addr)) {
        Ok(l) => l,
        Err(e) => {
            let msg = if e.kind() == std::io::ErrorKind::AddrInUse {
                format!("Port {port} is already in use. Rastro is probably already open; if not, choose another port in config.json or with --port.")
            } else {
                format!("Could not listen on {addr}: {e}")
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
    if app.check_updates_on_start() {
        let app = app.clone();
        // A few seconds after starting, so the window opens first.
        rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(4)).await;
            app.check_updates();
        });
    }
    let router = server::router(app);
    rt.spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            tracing::error!("the server stopped: {e}");
        }
    });
    tracing::info!("Rastro listening on http://{addr} (this PC only)");

    let web_port = if args.dev { server::DEV_PORT } else { port };
    let origin = format!("http://127.0.0.1:{web_port}");
    let url = format!("{origin}/#t={}", token.as_str());
    if window {
        // Bloquea hasta cerrar la ventana; al cerrarla termina el proceso.
        if let Err(e) = desktop::run(desktop::Page::Url(url.clone()), origin) {
            tracing::error!("could not open the window ({e}); using the browser");
            let _ = opener::open(&url);
        }
    } else if args.open && opener::open(&url).is_ok() {
        tracing::info!("Browser opened.");
    } else {
        // El token solo se muestra aquí, en la consola local, y cambia en cada arranque.
        println!("\nOpen this address in your browser (don't share it):\n{url}\n");
    }
    // Modo navegador (o si la ventana falló): seguir hasta Ctrl+C o «Salir» en la app.
    rt.block_on(async {
        let _ = tokio::signal::ctrl_c().await;
    });
    Ok(())
}
