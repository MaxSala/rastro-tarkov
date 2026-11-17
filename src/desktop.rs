//! Ventana de escritorio propia (tao + wry; WebView2 en Windows, WebKitGTK en Linux).
//! La interfaz sigue siendo la web local: la ventana solo la muestra, sin barra de direcciones.

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Icon, WindowBuilder};
use wry::{NewWindowResponse, WebViewBuilder};

pub enum Page {
    Url(String),
    Html(String),
}

/// Icono de la ventana y de la barra de tareas: el logotipo en 64×64 (RGBA sin comprimir).
const WINDOW_ICON: &[u8] = include_bytes!("../assets/brand/window-64.rgba");

fn icon() -> Option<Icon> {
    Icon::from_rgba(WINDOW_ICON.to_vec(), 64, 64).ok()
}

fn is_local(url: &str, origin: &str) -> bool {
    url == origin || url.starts_with(&format!("{origin}/")) || url.starts_with("about:") || url.starts_with("data:")
}

/// Abre la ventana y bloquea el hilo principal hasta cerrarla; al cerrar, termina el proceso.
pub fn run(page: Page, origin: String) -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rastro")
        .with_inner_size(LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(LogicalSize::new(960.0, 600.0))
        .with_window_icon(icon())
        .build(&event_loop)?;
    let nav_origin = origin.clone();
    let builder = WebViewBuilder::new()
        .with_background_color((11, 11, 11, 255))
        .with_devtools(cfg!(debug_assertions))
        // Nada de navegar fuera de la app: los enlaces externos https van al navegador del sistema.
        .with_navigation_handler(move |url| {
            if is_local(&url, &nav_origin) {
                return true;
            }
            if url.starts_with("https://") {
                let _ = opener::open(&url);
            }
            false
        })
        .with_new_window_req_handler(move |url, _| {
            if url.starts_with("https://") || is_local(&url, &origin) {
                let _ = opener::open(&url);
            }
            NewWindowResponse::Deny
        })
        // Deja que la página gestione el arrastre de archivos (importar .zip).
        .with_drag_drop_handler(|_| false);
    let builder = match page {
        Page::Url(u) => builder.with_url(u),
        Page::Html(h) => builder.with_html(h),
    };
    #[cfg(not(target_os = "linux"))]
    let _webview = builder.build(&window)?;
    #[cfg(target_os = "linux")]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().ok_or_else(|| anyhow::anyhow!("sin contenedor GTK"))?;
        builder.build_gtk(vbox)?
    };
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *flow = ControlFlow::Exit;
        }
    })
}

/// Página de error mostrada en la propia ventana cuando el servidor no puede arrancar.
pub fn error_page(message: &str) -> String {
    let safe: String = message
        .chars()
        .map(|c| match c {
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '&' => "&amp;".to_owned(),
            '"' => "&quot;".to_owned(),
            c => c.to_string(),
        })
        .collect();
    format!(
        "<!doctype html><meta charset=utf-8><body style=\"margin:0;display:grid;place-items:center;height:100vh;\
background:#0b0b0b;color:#ededed;font:14px system-ui\"><div style=\"max-width:420px\"><h1 style=\"font-size:16px\">\
Rastro no ha podido arrancar</h1><p style=\"color:#b4b4b4\">{safe}</p></div></body>"
    )
}
