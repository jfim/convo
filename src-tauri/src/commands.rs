use serde::Serialize;
use specta::Type;

use crate::error::ConvoError;
use crate::model::ConversationEvent;
use crate::{parser, resolver};

/// The payload returned to the frontend: the ordered events plus the optional
/// turn anchor parsed from the URL fragment.
#[derive(Debug, Serialize, Type)]
pub struct LoadedConversation {
    pub events: Vec<ConversationEvent>,
    pub anchor: Option<String>,
}

/// Resolve a `convo://` URL and load the referenced conversation.
#[tauri::command]
#[specta::specta]
pub fn load_conversation(url: String) -> Result<LoadedConversation, ConvoError> {
    let home = dirs::home_dir().ok_or(ConvoError::NoHome)?;
    let projects_root = home.join(".claude").join("projects");
    let target = resolver::resolve(&url, &projects_root)?;
    let events = parser::parse(&target.path)?;
    Ok(LoadedConversation {
        events,
        anchor: target.anchor,
    })
}
