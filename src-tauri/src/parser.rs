use std::path::Path;

use crate::error::ConvoError;
use crate::model::{ConversationEvent, ParseError};

/// Parse a conversation `.jsonl` file into events, in file order.
///
/// Headless by design: takes a path, returns events, no Tauri dependency — so a
/// future background indexer can reuse it. A line that is not valid JSON becomes a
/// synthetic `parse-error` event carrying the raw text and line number, so one bad
/// line never aborts the whole conversation.
pub fn parse(path: &Path) -> Result<Vec<ConversationEvent>, ConvoError> {
    if !path.exists() {
        return Err(ConvoError::NotFound(path.display().to_string()));
    }
    let contents = std::fs::read_to_string(path).map_err(|e| ConvoError::Io(e.to_string()))?;
    Ok(parse_str(&contents))
}

/// Parse the textual contents of a `.jsonl` file. Split out from `parse` so it can
/// be tested without touching the filesystem.
pub fn parse_str(contents: &str) -> Vec<ConversationEvent> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(idx, line)| match serde_json::from_str::<ConversationEvent>(line) {
            Ok(ev) => ev,
            Err(e) => ConversationEvent {
                event_type: "parse-error".to_string(),
                uuid: Some(format!("parse-error-{}", idx + 1)),
                parse_error: Some(ParseError {
                    line_number: (idx + 1) as u32,
                    raw: line.to_string(),
                    message: e.to_string(),
                }),
                parent_uuid: None,
                timestamp: None,
                session_id: None,
                cwd: None,
                git_branch: None,
                message: None,
                subtype: None,
                level: None,
                content: None,
                attachment: None,
                pr_url: None,
                pr_repository: None,
                pr_number: None,
                tool_use_result: None,
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn parses_basic_fixture_in_order() {
        let events = parse(&fixture("basic.jsonl")).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].event_type, "user");
        assert_eq!(events[1].event_type, "assistant");
        assert_eq!(events[2].event_type, "pr-link");
        assert_eq!(events[2].pr_number, Some(1));
    }

    #[test]
    fn malformed_line_becomes_parse_error_without_aborting() {
        let events = parse(&fixture("malformed.jsonl")).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].event_type, "user");
        assert_eq!(events[1].event_type, "parse-error");
        assert_eq!(events[1].parse_error.as_ref().unwrap().line_number, 2);
        assert_eq!(events[2].event_type, "assistant");
    }

    #[test]
    fn missing_file_is_not_found() {
        let res = parse(&fixture("does-not-exist.jsonl"));
        assert!(matches!(res, Err(ConvoError::NotFound(_))));
    }

    #[test]
    fn blank_lines_are_skipped() {
        let events = parse_str("\n\n{\"type\":\"user\"}\n\n");
        assert_eq!(events.len(), 1);
    }
}
