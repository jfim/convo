//! Transforms the raw `Vec<ConversationEvent>` (faithful to the Claude Code
//! `.jsonl` wire format) into a structured render model the frontend draws
//! directly. This is the `ConversationEvent -> RenderItem` stage of the
//! pipeline; keeping it here (headless, pure, tested) keeps the frontend
//! presentational and leaves a clean seam for future conversation sources.

use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::model::{
    Content, ContentBlock, ConversationEvent, KnownBlock, Message, ToolResultContent,
};

/// A tool call's result, joined from the `tool_result` block that appears in a
/// later user turn.
#[derive(Debug, Clone, Serialize, Type)]
pub struct ToolResult {
    pub content: Value,
    #[serde(rename = "isError")]
    pub is_error: bool,
}

/// One block within an assistant turn.
#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AssistantBlock {
    /// Assistant prose, rendered as markdown.
    Markdown { text: String },
    /// Extended thinking. Only emitted when non-empty.
    Thinking { text: String },
    /// A tool invocation with its result joined in (None if not yet available).
    ToolCall {
        id: String,
        name: String,
        input: Value,
        result: Option<ToolResult>,
    },
    /// An unrecognized content block, preserved as raw JSON.
    Unknown { raw: Value },
}

/// A top-level item in the rendered transcript.
#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RenderItem {
    /// A genuine human prompt (string content, or array content containing text).
    UserPrompt { uuid: Option<String>, text: String },
    /// A system-injected "user" turn (e.g. `<task-notification>`, slash-command
    /// invocations) — not something the human typed. Rendered distinctly.
    Notice { uuid: Option<String>, text: String },
    /// An assistant turn with its inline blocks (text, thinking, tool calls).
    AssistantTurn {
        uuid: Option<String>,
        blocks: Vec<AssistantBlock>,
    },
    System {
        uuid: Option<String>,
        subtype: Option<String>,
        content: Option<Value>,
    },
    Attachment {
        uuid: Option<String>,
        content: Option<Value>,
    },
    PrLink {
        uuid: Option<String>,
        url: Option<String>,
        number: Option<i32>,
    },
    /// An unrecognized event type, preserved as raw JSON.
    Unknown {
        uuid: Option<String>,
        label: String,
        raw: Value,
    },
    ParseError {
        uuid: Option<String>,
        #[serde(rename = "lineNumber")]
        line_number: u32,
        raw: String,
    },
}

/// Build the render model from raw events.
///
/// - Joins each assistant `tool_use` with the matching `tool_result` from a
///   later user turn (by `tool_use_id`).
/// - Drops user turns that carry only tool results (their content surfaces
///   inside the joined tool calls).
/// - Hides `queue-operation` and `last-prompt` events.
/// - Drops empty thinking blocks, empty assistant turns, and blank prompts.
/// - Classifies tag-wrapped "user" turns (e.g. `<task-notification>`) as notices.
/// - Merges contiguous assistant turns into one.
pub fn build(events: &[ConversationEvent]) -> Vec<RenderItem> {
    let results = collect_tool_results(events);

    let mut items = Vec::new();
    for ev in events {
        if let Some(pe) = &ev.parse_error {
            items.push(RenderItem::ParseError {
                uuid: ev.uuid.clone(),
                line_number: pe.line_number,
                raw: pe.raw.clone(),
            });
            continue;
        }

        match ev.event_type.as_str() {
            "queue-operation" | "last-prompt" => continue,
            "user" => {
                // None => tool-result-only turn (consumed by joined tool calls).
                let Some(text) = ev.message.as_ref().and_then(user_prompt_text) else {
                    continue;
                };
                if text.trim().is_empty() {
                    continue; // empty prompt — don't render
                }
                let uuid = ev.uuid.clone();
                if is_notice(&text) {
                    items.push(RenderItem::Notice { uuid, text });
                } else {
                    items.push(RenderItem::UserPrompt { uuid, text });
                }
            }
            "assistant" => {
                let blocks = ev
                    .message
                    .as_ref()
                    .map(|m| assistant_blocks(m, &results))
                    .unwrap_or_default();
                if blocks.is_empty() {
                    continue; // empty assistant turn — don't render
                }
                items.push(RenderItem::AssistantTurn {
                    uuid: ev.uuid.clone(),
                    blocks,
                });
            }
            "system" => items.push(RenderItem::System {
                uuid: ev.uuid.clone(),
                subtype: ev.subtype.clone(),
                content: ev.content.clone(),
            }),
            "attachment" => items.push(RenderItem::Attachment {
                uuid: ev.uuid.clone(),
                content: ev.attachment.clone(),
            }),
            "pr-link" => items.push(RenderItem::PrLink {
                uuid: ev.uuid.clone(),
                url: ev.pr_url.clone(),
                number: ev.pr_number,
            }),
            other => items.push(RenderItem::Unknown {
                uuid: ev.uuid.clone(),
                label: format!("Event: {other}"),
                raw: serde_json::to_value(ev).unwrap_or(Value::Null),
            }),
        }
    }
    merge_adjacent_assistant_turns(items)
}

/// Tags that mark a "user" turn as a system-injected notice rather than a
/// human-authored prompt.
const NOTICE_TAGS: &[&str] = &[
    "task-notification",
    "command-name",
    "command-message",
    "command-args",
    "local-command-stdout",
    "local-command-stderr",
    "bash-stdout",
    "bash-stderr",
    "system-reminder",
    "system",
    "user-prompt-submit-hook",
];

/// The name of the leading XML-ish tag in `text`, if it opens with one.
fn leading_tag(text: &str) -> Option<&str> {
    let rest = text.trim_start().strip_prefix('<')?;
    if rest.starts_with('/') || rest.starts_with('!') {
        return None;
    }
    let end = rest.find(['>', ' ', '\n', '\t', '/'])?;
    Some(&rest[..end])
}

/// Whether a user turn's text is a system-injected notice.
fn is_notice(text: &str) -> bool {
    leading_tag(text).is_some_and(|tag| NOTICE_TAGS.contains(&tag))
}

/// Merge directly-adjacent assistant turns into a single turn, concatenating
/// their blocks. Adjacency arises because tool-result-only user turns between
/// assistant events are dropped. The merged turn keeps the first turn's uuid.
fn merge_adjacent_assistant_turns(items: Vec<RenderItem>) -> Vec<RenderItem> {
    let mut out: Vec<RenderItem> = Vec::with_capacity(items.len());
    for item in items {
        if let RenderItem::AssistantTurn { blocks, .. } = &item {
            if let Some(RenderItem::AssistantTurn { blocks: prev, .. }) = out.last_mut() {
                prev.extend(blocks.iter().cloned());
                continue;
            }
        }
        out.push(item);
    }
    out
}

/// Scan all user turns for `tool_result` blocks, keyed by the tool-use id.
fn collect_tool_results(events: &[ConversationEvent]) -> HashMap<String, ToolResult> {
    let mut results = HashMap::new();
    for ev in events {
        let Some(Content::Blocks(blocks)) = ev.message.as_ref().map(|m| &m.content) else {
            continue;
        };
        for block in blocks {
            if let ContentBlock::Known(KnownBlock::ToolResult {
                tool_use_id,
                content,
                is_error,
            }) = block
            {
                results.insert(
                    tool_use_id.clone(),
                    ToolResult {
                        content: tool_result_content_to_value(content),
                        is_error: *is_error,
                    },
                );
            }
        }
    }
    results
}

/// The human-authored text of a user turn, or None if it carries only tool
/// results (and is therefore not a real prompt).
fn user_prompt_text(message: &Message) -> Option<String> {
    match &message.content {
        Content::Text(s) => Some(s.clone()),
        Content::Blocks(blocks) => {
            let texts: Vec<&str> = blocks
                .iter()
                .filter_map(|b| match b {
                    ContentBlock::Known(KnownBlock::Text { text }) => Some(text.as_str()),
                    _ => None,
                })
                .collect();
            if texts.is_empty() {
                None
            } else {
                Some(texts.join("\n\n"))
            }
        }
    }
}

fn assistant_blocks(
    message: &Message,
    results: &HashMap<String, ToolResult>,
) -> Vec<AssistantBlock> {
    let blocks = match &message.content {
        Content::Text(s) => return vec![AssistantBlock::Markdown { text: s.clone() }],
        Content::Blocks(b) => b,
    };
    blocks
        .iter()
        .filter_map(|block| match block {
            ContentBlock::Known(KnownBlock::Text { text }) => {
                Some(AssistantBlock::Markdown { text: text.clone() })
            }
            ContentBlock::Known(KnownBlock::Thinking { thinking }) => {
                if thinking.trim().is_empty() {
                    None
                } else {
                    Some(AssistantBlock::Thinking {
                        text: thinking.clone(),
                    })
                }
            }
            ContentBlock::Known(KnownBlock::ToolUse { id, name, input }) => {
                Some(AssistantBlock::ToolCall {
                    id: id.clone(),
                    name: name.clone(),
                    input: input.clone(),
                    result: results.get(id).cloned(),
                })
            }
            // A tool_result inside an assistant turn is unexpected; preserve it.
            other => Some(AssistantBlock::Unknown {
                raw: serde_json::to_value(other).unwrap_or(Value::Null),
            }),
        })
        .collect()
}

fn tool_result_content_to_value(content: &ToolResultContent) -> Value {
    match content {
        ToolResultContent::Text(s) => Value::String(s.clone()),
        ToolResultContent::Blocks(v) => Value::Array(v.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_str;

    fn build_from(jsonl: &str) -> Vec<RenderItem> {
        build(&parse_str(jsonl))
    }

    #[test]
    fn user_string_prompt_becomes_user_prompt() {
        let items = build_from(
            r#"{"type":"user","uuid":"u1","message":{"role":"user","content":"hello"}}"#,
        );
        assert_eq!(items.len(), 1);
        match &items[0] {
            RenderItem::UserPrompt { text, uuid } => {
                assert_eq!(text, "hello");
                assert_eq!(uuid.as_deref(), Some("u1"));
            }
            other => panic!("expected UserPrompt, got {other:?}"),
        }
    }

    #[test]
    fn tool_result_only_user_turn_is_dropped() {
        let items = build_from(
            r#"{"type":"user","message":{"role":"user","content":[{"type":"tool_result","tool_use_id":"t1","content":"ok"}]}}"#,
        );
        assert!(
            items.is_empty(),
            "tool-result-only turn should produce nothing"
        );
    }

    #[test]
    fn user_array_with_text_becomes_prompt() {
        let items = build_from(
            r#"{"type":"user","message":{"role":"user","content":[{"type":"text","text":"hi there"}]}}"#,
        );
        assert_eq!(items.len(), 1);
        assert!(matches!(&items[0], RenderItem::UserPrompt { text, .. } if text == "hi there"));
    }

    #[test]
    fn assistant_turn_joins_tool_result_and_drops_empty_thinking() {
        let jsonl = concat!(
            r#"{"type":"assistant","uuid":"a1","message":{"role":"assistant","content":[{"type":"thinking","thinking":"  "},{"type":"thinking","thinking":"real"},{"type":"text","text":"running it"},{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"ls","description":"list"}}]}}"#,
            "\n",
            r#"{"type":"user","message":{"role":"user","content":[{"type":"tool_result","tool_use_id":"t1","content":"file.txt","is_error":false}]}}"#,
        );
        let items = build_from(jsonl);
        assert_eq!(
            items.len(),
            1,
            "the tool-result user turn should be dropped"
        );
        let RenderItem::AssistantTurn { blocks, .. } = &items[0] else {
            panic!("expected AssistantTurn");
        };
        // empty thinking dropped: thinking(real), markdown, toolcall = 3 blocks
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], AssistantBlock::Thinking { text } if text == "real"));
        assert!(matches!(&blocks[1], AssistantBlock::Markdown { text } if text == "running it"));
        match &blocks[2] {
            AssistantBlock::ToolCall { name, result, .. } => {
                assert_eq!(name, "Bash");
                let r = result.as_ref().expect("result should be joined");
                assert_eq!(r.content, Value::String("file.txt".into()));
                assert!(!r.is_error);
            }
            other => panic!("expected ToolCall, got {other:?}"),
        }
    }

    #[test]
    fn tool_call_without_result_has_none() {
        let items = build_from(
            r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"t9","name":"Read","input":{"file_path":"/x"}}]}}"#,
        );
        let RenderItem::AssistantTurn { blocks, .. } = &items[0] else {
            panic!("expected AssistantTurn");
        };
        assert!(matches!(
            &blocks[0],
            AssistantBlock::ToolCall { result: None, .. }
        ));
    }

    #[test]
    fn queue_operation_and_last_prompt_are_hidden() {
        let jsonl = concat!(
            r#"{"type":"queue-operation","operation":"enqueue","content":"queued"}"#,
            "\n",
            r#"{"type":"last-prompt","lastPrompt":"x"}"#,
            "\n",
            r#"{"type":"user","message":{"role":"user","content":"real"}}"#,
        );
        let items = build_from(jsonl);
        assert_eq!(items.len(), 1);
        assert!(matches!(&items[0], RenderItem::UserPrompt { .. }));
    }

    #[test]
    fn pr_link_is_rendered() {
        let items = build_from(
            r#"{"type":"pr-link","uuid":"p1","prUrl":"https://example.com/pr/1","prNumber":1}"#,
        );
        assert!(matches!(
            &items[0],
            RenderItem::PrLink {
                number: Some(1),
                ..
            }
        ));
    }

    #[test]
    fn contiguous_assistant_turns_merge_into_one() {
        // Two assistant events separated only by a (dropped) tool-result turn.
        let jsonl = concat!(
            r#"{"type":"assistant","uuid":"a1","message":{"role":"assistant","content":[{"type":"text","text":"first"},{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"ls"}}]}}"#,
            "\n",
            r#"{"type":"user","message":{"role":"user","content":[{"type":"tool_result","tool_use_id":"t1","content":"ok"}]}}"#,
            "\n",
            r#"{"type":"assistant","uuid":"a2","message":{"role":"assistant","content":[{"type":"text","text":"second"}]}}"#,
        );
        let items = build_from(jsonl);
        assert_eq!(items.len(), 1, "adjacent assistant turns should merge");
        let RenderItem::AssistantTurn { blocks, uuid } = &items[0] else {
            panic!("expected AssistantTurn");
        };
        assert_eq!(uuid.as_deref(), Some("a1"), "keeps first turn's uuid");
        assert_eq!(blocks.len(), 3, "text + toolcall + text");
        assert!(matches!(&blocks[0], AssistantBlock::Markdown { text } if text == "first"));
        assert!(matches!(&blocks[2], AssistantBlock::Markdown { text } if text == "second"));
    }

    #[test]
    fn assistant_turns_around_a_real_prompt_do_not_merge() {
        let jsonl = concat!(
            r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"a"}]}}"#,
            "\n",
            r#"{"type":"user","message":{"role":"user","content":"hi"}}"#,
            "\n",
            r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"b"}]}}"#,
        );
        let items = build_from(jsonl);
        assert_eq!(items.len(), 3, "a real prompt between breaks contiguity");
    }

    #[test]
    fn empty_assistant_turn_is_dropped() {
        // Only an empty thinking block => no renderable blocks => dropped.
        let items = build_from(
            r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"thinking","thinking":"   "}]}}"#,
        );
        assert!(items.is_empty(), "empty assistant turn should be dropped");
    }

    #[test]
    fn blank_user_prompt_is_dropped() {
        let items = build_from(r#"{"type":"user","message":{"role":"user","content":"   "}}"#);
        assert!(items.is_empty(), "blank prompt should be dropped");
    }

    #[test]
    fn task_notification_user_turn_becomes_notice() {
        let items = build_from(
            r#"{"type":"user","message":{"role":"user","content":"<task-notification>\n<status>killed</status>\n</task-notification>"}}"#,
        );
        assert_eq!(items.len(), 1);
        assert!(
            matches!(&items[0], RenderItem::Notice { .. }),
            "tag-wrapped user turn should be a Notice, got {:?}",
            items[0]
        );
    }

    #[test]
    fn plain_prompt_starting_with_angle_bracket_is_not_a_notice() {
        let items = build_from(
            r#"{"type":"user","message":{"role":"user","content":"<div> should I use this tag?"}}"#,
        );
        assert!(
            matches!(&items[0], RenderItem::UserPrompt { .. }),
            "unknown leading tag should stay a UserPrompt"
        );
    }
}
