//! Messages the player sees. The interface translates them by `code` into its 16 languages;
//! `text` is the English fallback (used for logs and by older interfaces). `arg` carries the one
//! variable part, such as a folder, a port or a map name.

use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserMsg {
    pub code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    pub text: String,
}

impl UserMsg {
    pub fn new(code: &'static str, text: impl Into<String>) -> Self {
        Self { code, arg: None, text: text.into() }
    }

    pub fn with(code: &'static str, arg: impl fmt::Display, text: impl Into<String>) -> Self {
        Self { code, arg: Some(arg.to_string()), text: text.into() }
    }
}

impl fmt::Display for UserMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}
