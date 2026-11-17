//! Errores de la aplicación con mensajes en español, listos para mostrar al usuario.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Petición no válida: {0}")]
    BadRequest(String),
    #[error("No encontrado.")]
    NotFound,
    #[error("Paquete de mapa no válido: {0}")]
    InvalidPack(String),
    #[error("La descarga falló: {0}")]
    Download(String),
    #[error("Ya existe un paquete con el id «{0}». Confirma para reemplazarlo.")]
    PackExists(String),
    #[error("Error de disco: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error interno: {0}")]
    Internal(String),
}

impl AppError {
    pub fn pack(msg: impl Into<String>) -> Self {
        Self::InvalidPack(msg.into())
    }
    pub fn bad(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }
    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) | Self::InvalidPack(_) | Self::Download(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::PackExists(_) => StatusCode::CONFLICT,
            Self::Io(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        if status.is_server_error() {
            tracing::error!("{self}");
        }
        (status, Json(serde_json::json!({ "error": self.to_string() }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
