use serde::{Deserialize, Serialize};
use specta::Type;

/// One line of a conversation `.jsonl`, decoded leniently.
///
/// Mirrors the Claude Code wire format directly (v1 is intentionally coupled to
/// it). Every non-guaranteed field is optional so unknown shapes never fail to
/// decode. `parse_error` is synthesized by the parser for lines that fail to
/// decode as JSON; it is never present in the source.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ConversationEvent {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub parent_uuid: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub git_branch: Option<String>,
    #[serde(default)]
    pub message: Option<Message>,
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    #[serde(default)]
    pub attachment: Option<serde_json::Value>,
    #[serde(default)]
    pub pr_url: Option<String>,
    #[serde(default)]
    pub pr_repository: Option<String>,
    #[serde(default)]
    pub pr_number: Option<i32>,
    #[serde(default)]
    pub tool_use_result: Option<serde_json::Value>,
    /// Set by the parser when the raw line could not be decoded as JSON.
    /// Never present in source `.jsonl`; synthesized by the parser only.
    #[serde(default)]
    pub parse_error: Option<ParseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ParseError {
    pub line_number: u32,
    pub raw: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Message {
    #[serde(default)]
    pub role: Option<String>,
    pub content: Content,
}

/// User `message.content` is either a plain prompt string or an array of blocks.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

/// A content block. Unknown block kinds are preserved as raw JSON so they can be
/// rendered as a visible "unrecognized" node instead of being dropped.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(untagged)]
pub enum ContentBlock {
    Known(KnownBlock),
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KnownBlock {
    Text {
        text: String,
    },
    Thinking {
        thinking: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: ToolResultContent,
        #[serde(default)]
        is_error: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(untagged)]
pub enum ToolResultContent {
    Text(String),
    Blocks(Vec<serde_json::Value>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_user_string_prompt() {
        let line = r#"{"type":"user","uuid":"u1","message":{"role":"user","content":"hello"}}"#;
        let ev: ConversationEvent = serde_json::from_str(line).unwrap();
        assert_eq!(ev.event_type, "user");
        assert_eq!(ev.uuid.as_deref(), Some("u1"));
        match ev.message.unwrap().content {
            Content::Text(s) => assert_eq!(s, "hello"),
            _ => panic!("expected text content"),
        }
    }

    #[test]
    fn decodes_assistant_blocks_including_thinking_and_tool_use() {
        let line = r#"{"type":"assistant","uuid":"a1","message":{"role":"assistant","content":[{"type":"thinking","thinking":"hmm"},{"type":"text","text":"hi"},{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"ls"}}]}}"#;
        let ev: ConversationEvent = serde_json::from_str(line).unwrap();
        let blocks = match ev.message.unwrap().content {
            Content::Blocks(b) => b,
            _ => panic!("expected blocks"),
        };
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], ContentBlock::Known(KnownBlock::Thinking { .. })));
        assert!(matches!(&blocks[2], ContentBlock::Known(KnownBlock::ToolUse { .. })));
    }

    #[test]
    fn unknown_block_kind_is_preserved_as_raw() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"future_block","data":42}]}}"#;
        let ev: ConversationEvent = serde_json::from_str(line).unwrap();
        let blocks = match ev.message.unwrap().content {
            Content::Blocks(b) => b,
            _ => panic!("expected blocks"),
        };
        assert!(matches!(&blocks[0], ContentBlock::Unknown(_)));
    }

    #[test]
    fn unknown_event_type_still_decodes() {
        let line = r#"{"type":"queue-operation","operation":"enqueue"}"#;
        let ev: ConversationEvent = serde_json::from_str(line).unwrap();
        assert_eq!(ev.event_type, "queue-operation");
    }
}
