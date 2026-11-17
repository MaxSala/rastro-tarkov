//! WebSocket /ws: autenticación con el primer mensaje, luego instantáneas y órdenes.

use super::{token_ok, Ctx};
use crate::error::AppError;
use crate::protocol::{ClientMsg, NoticeLevel, ServerMsg};
use crate::state::App;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::time::Duration;
use tokio::sync::broadcast::error::RecvError;

pub const MAX_SOCKETS: usize = 4;
const MAX_WS_MESSAGE: usize = 64 * 1024;
const AUTH_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn handler(State(ctx): State<Ctx>, ws: WebSocketUpgrade) -> Response {
    let Ok(slot) = ctx.ws_slots.clone().try_acquire_owned() else {
        return (StatusCode::SERVICE_UNAVAILABLE, "Demasiadas pestañas abiertas.").into_response();
    };
    ws.max_message_size(MAX_WS_MESSAGE).max_frame_size(MAX_WS_MESSAGE).on_upgrade(move |socket| async move {
        session(ctx, socket).await;
        drop(slot);
    })
}

async fn send(socket: &mut WebSocket, msg: &ServerMsg) -> bool {
    match serde_json::to_string(msg) {
        Ok(text) => socket.send(Message::Text(text.into())).await.is_ok(),
        Err(e) => {
            tracing::error!("no se pudo serializar: {e}");
            false
        }
    }
}

async fn authenticate(app: &App, socket: &mut WebSocket) -> bool {
    match tokio::time::timeout(AUTH_TIMEOUT, socket.recv()).await {
        Ok(Some(Ok(Message::Text(t)))) => {
            matches!(serde_json::from_str::<ClientMsg>(&t), Ok(ClientMsg::Auth { token }) if token_ok(app, &token))
        }
        _ => false,
    }
}

async fn session(ctx: Ctx, mut socket: WebSocket) {
    let app = ctx.app;
    if !authenticate(&app, &mut socket).await {
        let _ = socket.send(Message::Close(None)).await;
        return;
    }
    let mut changes = app.changed.subscribe();
    let mut sent_map: Option<String> = None;
    if !push_state(&app, &mut socket, &mut sent_map).await {
        return;
    }
    loop {
        tokio::select! {
            incoming = socket.recv() => {
                let text = match incoming {
                    Some(Ok(Message::Text(t))) => t,
                    Some(Ok(Message::Ping(_) | Message::Pong(_))) => continue,
                    _ => break,
                };
                let reply = match serde_json::from_str::<ClientMsg>(&text) {
                    Ok(msg) => handle(&app, msg),
                    Err(_) => Some(notice(NoticeLevel::Error, "Mensaje no reconocido.".into())),
                };
                if let Some(r) = reply {
                    if !send(&mut socket, &r).await {
                        break;
                    }
                }
            }
            change = changes.recv() => {
                match change {
                    Ok(()) | Err(RecvError::Lagged(_)) => {
                        // Agrupa ráfagas de cambios en una sola instantánea.
                        while changes.try_recv().is_ok() {}
                        if !push_state(&app, &mut socket, &mut sent_map).await {
                            break;
                        }
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        }
    }
}

/// Envía las capas del mapa si ha cambiado y después la instantánea.
async fn push_state(app: &App, socket: &mut WebSocket, sent_map: &mut Option<String>) -> bool {
    if let Some((map_id, markers, icons)) = crate::view_extra::map_markers(app) {
        // Se reenvía al cambiar de mapa o si cambian los iconos disponibles (tras descargarlos).
        let key = format!("{map_id}#{}", icons.len());
        if sent_map.as_deref() != Some(key.as_str()) {
            *sent_map = Some(key);
            if !send(socket, &ServerMsg::MapData { map_id, markers, icons }).await {
                return false;
            }
        }
    }
    send(socket, &snapshot(app)).await
}

fn snapshot(app: &App) -> ServerMsg {
    ServerMsg::Snapshot { view: Box::new(crate::view::build(app)) }
}

fn notice(level: NoticeLevel, text: String) -> ServerMsg {
    ServerMsg::Notice { level, text }
}

fn handle(app: &std::sync::Arc<App>, msg: ClientMsg) -> Option<ServerMsg> {
    let result: Result<Option<ServerMsg>, AppError> = match msg {
        ClientMsg::Auth { .. } => Ok(None),
        ClientMsg::SetActiveMap { map_id } => app.set_active_map(&map_id).map(|()| None),
        ClientMsg::NewRaid => {
            app.new_raid();
            Ok(None)
        }
        ClientMsg::SelectExtract { extract_id } => {
            app.select_extract(extract_id);
            Ok(None)
        }
        ClientMsg::ToggleExtract { extract_id } => app.toggle_extract(extract_id).map(|()| None),
        ClientMsg::UpdateSettings { settings } => {
            app.update_settings(settings).map(|()| Some(notice(NoticeLevel::Info, "Ajustes guardados.".into())))
        }
        ClientMsg::AckShareWarning => app.ack_share_warning().map(|()| None),
        ClientMsg::PartyHost { bind_ip, advertise } => app.party_host(&bind_ip, &advertise).map(|()| None),
        ClientMsg::PartyJoin { code } => app.party_join(&code).map(|()| None),
        ClientMsg::PartyLeave => {
            crate::party::lock(&app.party).leave();
            Ok(None)
        }
        ClientMsg::PartyKick { peer_id } => {
            crate::party::lock(&app.party).kick(peer_id);
            Ok(None)
        }
        ClientMsg::MapPing { px, py } => app.map_ping(px, py).map(|()| None),
        ClientMsg::ListInterfaces => Ok(Some(ServerMsg::Interfaces { list: crate::party::interfaces() })),
        ClientMsg::RemovePack { pack_id } => app.remove_pack(&pack_id).map(|()| None),
        ClientMsg::TarkovDevSync => crate::tarkovdev::sync::start(app).map(|()| None),
        ClientMsg::FinishOnboarding => app.finish_onboarding().map(|()| None),
        ClientMsg::CheckFolder { path } => {
            crate::setup::check_folder(&path).map(|result| Some(ServerMsg::FolderCheck { result }))
        }
        ClientMsg::SetScreenshotsDir { path } => app.set_screenshots_dir(&path).map(|()| None),
        ClientMsg::AddPoint { px, py, name, kind, note } => app.add_point(px, py, &name, kind, &note).map(|()| None),
        ClientMsg::UpdatePoint { id, name, kind, note } => app.update_point(&id, &name, kind, &note).map(|()| None),
        ClientMsg::DeletePoint { id } => app.delete_point(&id).map(|()| None),
        ClientMsg::GetLogs => Ok(Some(ServerMsg::Logs { lines: crate::logs::lines() })),
        ClientMsg::OpenDataDir => opener::open(&app.paths.base)
            .map(|()| None)
            .map_err(|e| AppError::Internal(format!("no se pudo abrir la carpeta: {e}"))),
        ClientMsg::SetOpenInWindow { value } => app.set_open_in_window(value).map(|()| None),
        ClientMsg::SetGameLogs { enabled, path } => app.set_game_logs(enabled, &path).map(|()| None),
        ClientMsg::SetGameMode { mode } => app.set_game_mode(mode).map(|()| None),
        ClientMsg::MarketRefresh { flea_name } => {
            let flea: String = flea_name.chars().filter(|c| !c.is_control()).take(40).collect();
            app.market_refresh(flea).map(|()| None)
        }
        ClientMsg::SetLanguage { lang } => app.set_language(lang).map(|()| None),
        ClientMsg::Quit => {
            tracing::info!("Cierre pedido desde la app.");
            std::process::exit(0);
        }
    };
    result.unwrap_or_else(|e| Some(notice(NoticeLevel::Error, e.to_string())))
}
