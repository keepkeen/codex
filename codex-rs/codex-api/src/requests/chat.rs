use crate::error::ApiError;
use crate::provider::Provider;
use crate::requests::headers::build_conversation_headers;
use crate::requests::headers::insert_header;
use crate::requests::headers::subagent_header;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::LocalShellAction;
use codex_protocol::models::LocalShellStatus;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::SessionSource;
use http::HeaderMap;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

/// Assembled request body plus headers for Chat Completions streaming calls.
pub struct ChatRequest {
    pub body: Value,
    pub headers: HeaderMap,
}

pub struct ChatRequestBuilder<'a> {
    model: &'a str,
    instructions: &'a str,
    input: &'a [ResponseItem],
    tools: &'a [Value],
    conversation_id: Option<String>,
    session_source: Option<SessionSource>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReasoningFormat {
    Reasoning,
    ReasoningContent,
    ReasoningDetails,
}

struct ProviderBehavior {
    model: String,
    reasoning_format: Option<ReasoningFormat>,
    extra_body: Map<String, Value>,
}

impl<'a> ChatRequestBuilder<'a> {
    pub fn new(
        model: &'a str,
        instructions: &'a str,
        input: &'a [ResponseItem],
        tools: &'a [Value],
    ) -> Self {
        Self {
            model,
            instructions,
            input,
            tools,
            conversation_id: None,
            session_source: None,
        }
    }

    pub fn conversation_id(mut self, id: Option<String>) -> Self {
        self.conversation_id = id;
        self
    }

    pub fn session_source(mut self, source: Option<SessionSource>) -> Self {
        self.session_source = source;
        self
    }

    pub fn build(self, provider: &Provider) -> Result<ChatRequest, ApiError> {
        let behavior = provider_behavior(provider, self.model);
        let mut messages = Vec::<Value>::new();
        messages.push(json!({"role": "system", "content": self.instructions}));

        let mut reasoning_by_anchor_index: HashMap<usize, String> = HashMap::new();
        let mut last_emitted_role: Option<&str> = None;
        for item in self.input {
            match item {
                ResponseItem::Message { role, .. } => last_emitted_role = Some(role.as_str()),
                ResponseItem::FunctionCall { .. }
                | ResponseItem::LocalShellCall { .. }
                | ResponseItem::CustomToolCall { .. }
                | ResponseItem::ToolSearchCall { .. } => last_emitted_role = Some("assistant"),
                ResponseItem::FunctionCallOutput { .. }
                | ResponseItem::CustomToolCallOutput { .. }
                | ResponseItem::ToolSearchOutput { .. } => last_emitted_role = Some("tool"),
                ResponseItem::Reasoning { .. }
                | ResponseItem::WebSearchCall { .. }
                | ResponseItem::ImageGenerationCall { .. }
                | ResponseItem::GhostSnapshot { .. }
                | ResponseItem::Compaction { .. }
                | ResponseItem::Other => {}
            }
        }

        let mut last_user_index: Option<usize> = None;
        for (idx, item) in self.input.iter().enumerate() {
            if let ResponseItem::Message { role, .. } = item
                && role == "user"
            {
                last_user_index = Some(idx);
            }
        }

        if !matches!(last_emitted_role, Some("user")) {
            for (idx, item) in self.input.iter().enumerate() {
                if let Some(user_idx) = last_user_index
                    && idx <= user_idx
                {
                    continue;
                }

                if let ResponseItem::Reasoning {
                    content: Some(items),
                    ..
                } = item
                {
                    let mut text = String::new();
                    for entry in items {
                        match entry {
                            codex_protocol::models::ReasoningItemContent::ReasoningText {
                                text: segment,
                            }
                            | codex_protocol::models::ReasoningItemContent::Text {
                                text: segment,
                            } => text.push_str(segment),
                        }
                    }
                    if text.trim().is_empty() {
                        continue;
                    }

                    let mut attached = false;
                    if idx > 0
                        && let ResponseItem::Message { role, .. } = &self.input[idx - 1]
                        && role == "assistant"
                    {
                        reasoning_by_anchor_index
                            .entry(idx - 1)
                            .and_modify(|value| value.push_str(&text))
                            .or_insert(text.clone());
                        attached = true;
                    }

                    if !attached && idx + 1 < self.input.len() {
                        match &self.input[idx + 1] {
                            ResponseItem::FunctionCall { .. }
                            | ResponseItem::LocalShellCall { .. }
                            | ResponseItem::CustomToolCall { .. }
                            | ResponseItem::ToolSearchCall { .. } => {
                                reasoning_by_anchor_index
                                    .entry(idx + 1)
                                    .and_modify(|value| value.push_str(&text))
                                    .or_insert(text.clone());
                            }
                            ResponseItem::Message { role, .. } if role == "assistant" => {
                                reasoning_by_anchor_index
                                    .entry(idx + 1)
                                    .and_modify(|value| value.push_str(&text))
                                    .or_insert(text.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let mut last_assistant_text: Option<String> = None;

        for (idx, item) in self.input.iter().enumerate() {
            match item {
                ResponseItem::Message { role, content, .. } => {
                    let mut text = String::new();
                    let mut items: Vec<Value> = Vec::new();
                    let mut saw_image = false;

                    for content_item in content {
                        match content_item {
                            ContentItem::InputText { text: value }
                            | ContentItem::OutputText { text: value } => {
                                text.push_str(value);
                                items.push(json!({"type":"text","text": value}));
                            }
                            ContentItem::InputImage { image_url } => {
                                saw_image = true;
                                items.push(
                                    json!({"type":"image_url","image_url": {"url": image_url}}),
                                );
                            }
                        }
                    }

                    if role == "assistant" {
                        if let Some(previous) = &last_assistant_text
                            && previous == &text
                        {
                            continue;
                        }
                        last_assistant_text = Some(text.clone());
                    }

                    let content_value = if role == "assistant" {
                        json!(text)
                    } else if saw_image {
                        json!(items)
                    } else {
                        json!(text)
                    };

                    let mut message = json!({"role": role, "content": content_value});
                    if role == "assistant"
                        && let Some(reasoning) = reasoning_by_anchor_index.get(&idx)
                    {
                        attach_reasoning_field(&mut message, behavior.reasoning_format, reasoning);
                    }
                    messages.push(message);
                }
                ResponseItem::FunctionCall {
                    name,
                    namespace,
                    arguments,
                    call_id,
                    ..
                } => {
                    let reasoning = reasoning_by_anchor_index.get(&idx).map(String::as_str);
                    let tool_call = json!({
                        "id": call_id,
                        "type": "function",
                        "function": {
                            "name": qualified_tool_name(name, namespace.as_deref()),
                            "arguments": arguments,
                        }
                    });
                    push_tool_call_message(
                        &mut messages,
                        tool_call,
                        behavior.reasoning_format,
                        reasoning,
                    );
                }
                ResponseItem::LocalShellCall {
                    id,
                    call_id: _,
                    status,
                    action,
                } => {
                    let reasoning = reasoning_by_anchor_index.get(&idx).map(String::as_str);
                    let tool_call = json!({
                        "id": id.clone().unwrap_or_else(|| format!("local-shell-{idx}")),
                        "type": "function",
                        "function": {
                            "name": "local_shell_call",
                            "arguments": local_shell_call_arguments(status, action)?,
                        }
                    });
                    push_tool_call_message(
                        &mut messages,
                        tool_call,
                        behavior.reasoning_format,
                        reasoning,
                    );
                }
                ResponseItem::FunctionCallOutput { call_id, output } => {
                    let content_value = if let Some(items) = output.content_items() {
                        let mapped: Vec<Value> = items
                            .iter()
                            .map(|item| match item {
                                FunctionCallOutputContentItem::InputText { text } => {
                                    json!({"type":"text","text": text})
                                }
                                FunctionCallOutputContentItem::InputImage { image_url, .. } => {
                                    json!({"type":"image_url","image_url": {"url": image_url}})
                                }
                            })
                            .collect();
                        json!(mapped)
                    } else {
                        json!(output.text_content().unwrap_or_default())
                    };

                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": call_id,
                        "content": content_value,
                    }));
                }
                ResponseItem::CustomToolCall {
                    id,
                    call_id,
                    name,
                    input,
                    status: _,
                } => {
                    let reasoning = reasoning_by_anchor_index.get(&idx).map(String::as_str);
                    let tool_call = json!({
                        "id": id.clone().unwrap_or_else(|| call_id.clone()),
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": input,
                        }
                    });
                    push_tool_call_message(
                        &mut messages,
                        tool_call,
                        behavior.reasoning_format,
                        reasoning,
                    );
                }
                ResponseItem::CustomToolCallOutput {
                    call_id, output, ..
                } => {
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": call_id,
                        "content": output,
                    }));
                }
                ResponseItem::ToolSearchCall {
                    call_id,
                    execution,
                    arguments,
                    ..
                } => {
                    let reasoning = reasoning_by_anchor_index.get(&idx).map(String::as_str);
                    let tool_call = json!({
                        "id": call_id.clone().unwrap_or_else(|| format!("tool-search-{idx}")),
                        "type": "function",
                        "function": {
                            "name": execution,
                            "arguments": serde_json::to_string(arguments).map_err(|err| {
                                ApiError::Stream(format!(
                                    "failed to encode tool search arguments: {err}"
                                ))
                            })?,
                        }
                    });
                    push_tool_call_message(
                        &mut messages,
                        tool_call,
                        behavior.reasoning_format,
                        reasoning,
                    );
                }
                ResponseItem::ToolSearchOutput {
                    call_id,
                    status,
                    execution,
                    tools,
                } => {
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": call_id.clone().unwrap_or_else(|| execution.clone()),
                        "content": serde_json::to_string(&json!({
                            "status": status,
                            "execution": execution,
                            "tools": tools,
                        }))
                        .map_err(|err| {
                            ApiError::Stream(format!(
                                "failed to encode tool search output: {err}"
                            ))
                        })?,
                    }));
                }
                ResponseItem::Reasoning { .. }
                | ResponseItem::WebSearchCall { .. }
                | ResponseItem::ImageGenerationCall { .. }
                | ResponseItem::GhostSnapshot { .. }
                | ResponseItem::Compaction { .. }
                | ResponseItem::Other => continue,
            }
        }

        let mut payload = Map::from_iter([
            ("model".to_string(), Value::String(behavior.model)),
            ("messages".to_string(), Value::Array(messages)),
            ("stream".to_string(), Value::Bool(true)),
        ]);
        if !self.tools.is_empty() {
            payload.insert("tools".to_string(), Value::Array(self.tools.to_vec()));
            payload.insert("tool_choice".to_string(), Value::String("auto".to_string()));
        }
        payload.extend(behavior.extra_body);

        let mut headers = build_conversation_headers(self.conversation_id);
        if let Some(subagent) = subagent_header(&self.session_source) {
            insert_header(&mut headers, "x-openai-subagent", &subagent);
        }

        Ok(ChatRequest {
            body: Value::Object(payload),
            headers,
        })
    }
}

fn qualified_tool_name(name: &str, namespace: Option<&str>) -> String {
    match namespace {
        Some(namespace) => format!("{namespace}/{name}"),
        None => name.to_string(),
    }
}

fn local_shell_call_arguments(
    status: &LocalShellStatus,
    action: &LocalShellAction,
) -> Result<String, ApiError> {
    serde_json::to_string(&json!({
        "status": status,
        "action": action,
    }))
    .map_err(|err| ApiError::Stream(format!("failed to encode local shell call: {err}")))
}

fn attach_reasoning_field(
    message: &mut Value,
    reasoning_format: Option<ReasoningFormat>,
    reasoning: &str,
) {
    let Some(object) = message.as_object_mut() else {
        return;
    };
    match reasoning_format {
        Some(ReasoningFormat::Reasoning) => {
            object.insert(
                "reasoning".to_string(),
                Value::String(reasoning.to_string()),
            );
        }
        Some(ReasoningFormat::ReasoningContent) => {
            object.insert(
                "reasoning_content".to_string(),
                Value::String(reasoning.to_string()),
            );
        }
        Some(ReasoningFormat::ReasoningDetails) => {
            object.insert(
                "reasoning_details".to_string(),
                json!([{ "text": reasoning }]),
            );
        }
        None => {}
    }
}

fn push_tool_call_message(
    messages: &mut Vec<Value>,
    tool_call: Value,
    reasoning_format: Option<ReasoningFormat>,
    reasoning: Option<&str>,
) {
    if let Some(Value::Object(object)) = messages.last_mut()
        && object.get("role").and_then(Value::as_str) == Some("assistant")
        && object.get("content").is_some_and(Value::is_null)
        && let Some(tool_calls) = object.get_mut("tool_calls").and_then(Value::as_array_mut)
    {
        tool_calls.push(tool_call);
        if let Some(reasoning) = reasoning {
            let mut tmp = Value::Object(object.clone());
            attach_reasoning_field(&mut tmp, reasoning_format, reasoning);
            if let Some(updated) = tmp.as_object() {
                *object = updated.clone();
            }
        }
        return;
    }

    let mut message = json!({
        "role": "assistant",
        "content": null,
        "tool_calls": [tool_call],
    });
    if let Some(reasoning) = reasoning {
        attach_reasoning_field(&mut message, reasoning_format, reasoning);
    }
    messages.push(message);
}

fn provider_behavior(provider: &Provider, model: &str) -> ProviderBehavior {
    let provider_name = provider.name.to_ascii_lowercase();
    let provider_base = provider.base_url.to_ascii_lowercase();

    if provider_base.contains("deepseek.com") || provider_name.contains("deepseek") {
        let mut extra_body = Map::new();
        let model = match model {
            "deepseek-chat-thinking" | "deepseek-thinking" => {
                extra_body.insert("thinking".to_string(), json!({ "type": "enabled" }));
                "deepseek-chat".to_string()
            }
            other => other.to_string(),
        };
        return ProviderBehavior {
            model,
            reasoning_format: Some(ReasoningFormat::ReasoningContent),
            extra_body,
        };
    }

    if provider_base.contains("moonshot.cn") || provider_name.contains("moonshot") {
        return ProviderBehavior {
            model: model.to_string(),
            reasoning_format: Some(ReasoningFormat::ReasoningContent),
            extra_body: Map::new(),
        };
    }

    if provider_base.contains("minimaxi.com") || provider_name.contains("minimax") {
        let mut extra_body = Map::new();
        extra_body.insert("reasoning_split".to_string(), Value::Bool(true));
        return ProviderBehavior {
            model: model.to_string(),
            reasoning_format: Some(ReasoningFormat::ReasoningDetails),
            extra_body,
        };
    }

    ProviderBehavior {
        model: model.to_string(),
        reasoning_format: Some(ReasoningFormat::Reasoning),
        extra_body: Map::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::RetryConfig;
    use codex_protocol::models::FunctionCallOutputPayload;
    use codex_protocol::models::LocalShellExecAction;
    use codex_protocol::protocol::SessionSource;
    use codex_protocol::protocol::SubAgentSource;
    use http::HeaderValue;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    fn provider(base_url: &str, name: &str) -> Provider {
        Provider {
            name: name.to_string(),
            base_url: base_url.to_string(),
            query_params: None,
            headers: HeaderMap::new(),
            retry: RetryConfig {
                max_attempts: 1,
                base_delay: Duration::from_millis(10),
                retry_429: false,
                retry_5xx: true,
                retry_transport: true,
            },
            stream_idle_timeout: Duration::from_secs(1),
        }
    }

    #[test]
    fn attaches_conversation_and_subagent_headers() {
        let prompt_input = vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "hi".to_string(),
            }],
            end_turn: None,
            phase: None,
        }];
        let req = ChatRequestBuilder::new("gpt-test", "inst", &prompt_input, &[])
            .conversation_id(Some("conv-1".into()))
            .session_source(Some(SessionSource::SubAgent(SubAgentSource::Review)))
            .build(&provider("https://api.openai.com/v1", "OpenAI"))
            .expect("request");

        assert_eq!(
            req.headers.get("session_id"),
            Some(&HeaderValue::from_static("conv-1"))
        );
        assert_eq!(
            req.headers.get("x-openai-subagent"),
            Some(&HeaderValue::from_static("review"))
        );
    }

    #[test]
    fn groups_consecutive_tool_calls_into_a_single_assistant_message() {
        let prompt_input = vec![
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "read these".to_string(),
                }],
                end_turn: None,
                phase: None,
            },
            ResponseItem::FunctionCall {
                id: None,
                name: "read_file".to_string(),
                namespace: None,
                arguments: r#"{"path":"a.txt"}"#.to_string(),
                call_id: "call-a".to_string(),
            },
            ResponseItem::FunctionCall {
                id: None,
                name: "read_file".to_string(),
                namespace: None,
                arguments: r#"{"path":"b.txt"}"#.to_string(),
                call_id: "call-b".to_string(),
            },
            ResponseItem::FunctionCallOutput {
                call_id: "call-a".to_string(),
                output: FunctionCallOutputPayload::from_text("A".to_string()),
            },
            ResponseItem::FunctionCallOutput {
                call_id: "call-b".to_string(),
                output: FunctionCallOutputPayload::from_text("B".to_string()),
            },
        ];

        let req = ChatRequestBuilder::new("gpt-test", "inst", &prompt_input, &[])
            .build(&provider("https://api.openai.com/v1", "OpenAI"))
            .expect("request");

        let messages = req
            .body
            .get("messages")
            .and_then(Value::as_array)
            .expect("messages array");
        assert_eq!(messages.len(), 5);
        assert_eq!(messages[2]["role"], "assistant");
        assert_eq!(
            messages[2]["tool_calls"]
                .as_array()
                .expect("tool calls")
                .len(),
            2
        );
    }

    #[test]
    fn deepseek_thinking_alias_adds_extra_body_and_reasoning_content() {
        let prompt_input = vec![
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "weather".to_string(),
                }],
                end_turn: None,
                phase: None,
            },
            ResponseItem::Reasoning {
                id: String::new(),
                summary: vec![],
                content: Some(vec![
                    codex_protocol::models::ReasoningItemContent::ReasoningText {
                        text: "need tool".to_string(),
                    },
                ]),
                encrypted_content: None,
            },
            ResponseItem::FunctionCall {
                id: None,
                name: "get_weather".to_string(),
                namespace: None,
                arguments: "{}".to_string(),
                call_id: "call-weather".to_string(),
            },
        ];

        let req = ChatRequestBuilder::new("deepseek-chat-thinking", "inst", &prompt_input, &[])
            .build(&provider("https://api.deepseek.com", "DeepSeek"))
            .expect("request");

        assert_eq!(req.body["model"], "deepseek-chat");
        assert_eq!(req.body["thinking"]["type"], "enabled");
        let assistant = &req.body["messages"].as_array().expect("messages")[2];
        assert_eq!(assistant["reasoning_content"], "need tool");
    }

    #[test]
    fn minimax_requests_reasoning_split() {
        let prompt_input = vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "hello".to_string(),
            }],
            end_turn: None,
            phase: None,
        }];

        let req = ChatRequestBuilder::new("MiniMax-M2.7", "inst", &prompt_input, &[])
            .build(&provider("https://api.minimaxi.com/v1", "MiniMax"))
            .expect("request");

        assert_eq!(req.body["reasoning_split"], true);
    }

    #[test]
    fn local_shell_calls_are_reencoded_as_function_tool_calls() {
        let prompt_input = vec![ResponseItem::LocalShellCall {
            id: Some("shell-1".to_string()),
            call_id: None,
            status: LocalShellStatus::Completed,
            action: LocalShellAction::Exec(LocalShellExecAction {
                command: vec!["pwd".to_string()],
                timeout_ms: None,
                working_directory: None,
                env: None,
                user: None,
            }),
        }];

        let req = ChatRequestBuilder::new("gpt-test", "inst", &prompt_input, &[])
            .build(&provider("https://api.openai.com/v1", "OpenAI"))
            .expect("request");

        let assistant = &req.body["messages"].as_array().expect("messages")[1];
        assert_eq!(assistant["tool_calls"][0]["type"], "function");
        assert_eq!(
            assistant["tool_calls"][0]["function"]["name"],
            "local_shell_call"
        );
    }
}
