use serde::Serialize;
use specta::Type;

use crate::error::ConvoError;
use crate::view::{ConversationStats, RenderItem};
use crate::{parser, resolver, view};

/// The payload returned to the frontend: the structured render model, the
/// optional turn anchor parsed from the URL fragment, and aggregate stats for
/// the conversation-details summary.
#[derive(Debug, Serialize, Type)]
pub struct LoadedConversation {
    pub items: Vec<RenderItem>,
    pub anchor: Option<String>,
    pub stats: ConversationStats,
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
        items: view::build(&events),
        stats: view::compute_stats(&events),
        anchor: target.anchor,
    })
}

/// Find the first `convo://` deep-link URL among the given arguments.
///
/// On Linux/Windows a cold-start deep link arrives as a CLI argument. The
/// deep-link plugin's own `get_current()` only matches when the URL is the
/// *sole* argument, which breaks under `cargo`/`tauri dev` (they append flags
/// like `--no-default-features`). Scanning all arguments for the scheme is
/// robust in both dev and production launches.
fn find_deep_link_arg<I: IntoIterator<Item = String>>(args: I) -> Option<String> {
    args.into_iter().find(|a| a.starts_with("convo://"))
}

/// The deep-link URL the app was cold-started with, if any. Read by the frontend
/// on load instead of the plugin's stricter `getCurrent()`.
#[tauri::command]
#[specta::specta]
pub fn initial_url() -> Option<String> {
    find_deep_link_arg(std::env::args())
}

#[cfg(test)]
mod tests {
    use super::find_deep_link_arg;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn finds_sole_url() {
        assert_eq!(
            find_deep_link_arg(args(&["/bin/convo", "convo://claude-code/p/u"])),
            Some("convo://claude-code/p/u".to_string())
        );
    }

    #[test]
    fn finds_url_among_extra_flags() {
        assert_eq!(
            find_deep_link_arg(args(&[
                "/bin/convo",
                "convo://claude-code/p/u",
                "--no-default-features",
                "--color",
                "always",
            ])),
            Some("convo://claude-code/p/u".to_string())
        );
    }

    #[test]
    fn finds_url_even_when_not_first_arg() {
        assert_eq!(
            find_deep_link_arg(args(&["/bin/convo", "--color", "convo://claude-code/p/u"])),
            Some("convo://claude-code/p/u".to_string())
        );
    }

    #[test]
    fn none_when_no_deep_link() {
        assert_eq!(
            find_deep_link_arg(args(&["/bin/convo", "--color", "always"])),
            None
        );
    }
}
