//! Application errors. Their text is English; `user_msg` adds the code the interface translates.

use crate::msg::UserMsg;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Not found.")]
    NotFound,
    #[error("Invalid map pack: {0}")]
    InvalidPack(String),
    #[error("Download failed: {0}")]
    Download(String),
    #[error("A pack with the id \"{0}\" already exists. Confirm to replace it.")]
    PackExists(String),
    #[error("Disk error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
    /// A message the player is expected to read and act on.
    #[error("{0}")]
    User(UserMsg),
}

impl AppError {
    pub fn pack(msg: impl Into<String>) -> Self {
        Self::InvalidPack(msg.into())
    }
    pub fn bad(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }
    pub fn user(code: &'static str, text: impl Into<String>) -> Self {
        Self::User(UserMsg::new(code, text))
    }
    pub fn user_with(code: &'static str, arg: impl std::fmt::Display, text: impl Into<String>) -> Self {
        Self::User(UserMsg::with(code, arg, text))
    }

    /// Translatable version of the error. Technical details (`arg`) stay in English.
    pub fn user_msg(&self) -> UserMsg {
        let text = self.to_string();
        let detail = |code, d: &String| UserMsg { code, arg: Some(d.clone()), text: text.clone() };
        match self {
            Self::User(m) => m.clone(),
            Self::PackExists(id) => detail("packExists", id),
            Self::InvalidPack(d) => detail("invalidPack", d),
            Self::Download(d) => detail("downloadFailed", d),
            Self::BadRequest(d) => detail("badRequest", d),
            Self::Internal(d) => detail("internalError", d),
            Self::Io(e) => UserMsg { code: "diskError", arg: Some(e.to_string()), text },
            Self::NotFound => UserMsg::new("notFound", text),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) | Self::InvalidPack(_) | Self::Download(_) | Self::User(_) => StatusCode::BAD_REQUEST,
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
        let m = self.user_msg();
        (status, Json(serde_json::json!({ "error": m.text, "code": m.code, "arg": m.arg }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
