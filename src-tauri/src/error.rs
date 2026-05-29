use serde::Serialize;
use specta::Type;

/// Error returned across the Tauri boundary. Serializes to a tagged object the
/// frontend can switch on.
#[derive(Debug, thiserror::Error, Serialize, Type)]
#[serde(tag = "kind", content = "message")]
pub enum ConvoError {
    #[error("invalid convo url: {0}")]
    InvalidUrl(String),
    #[error("path escapes the projects root: {0}")]
    PathTraversal(String),
    #[error("conversation file not found: {0}")]
    NotFound(String),
    #[error("could not read conversation: {0}")]
    Io(String),
    #[error("could not locate the home directory")]
    NoHome,
}
