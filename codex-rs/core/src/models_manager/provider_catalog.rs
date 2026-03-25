use super::model_info::BASE_INSTRUCTIONS;
use crate::model_provider_info::DEEPSEEK_PROVIDER_ID;
use crate::model_provider_info::GLM_PROVIDER_ID;
use crate::model_provider_info::KIMI_PROVIDER_ID;
use crate::model_provider_info::MINIMAX_PROVIDER_ID;
use crate::model_provider_info::ModelProviderInfo;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::openai_models::ApplyPatchToolType;
use codex_protocol::openai_models::ConfigShellToolType;
use codex_protocol::openai_models::InputModality;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::TruncationPolicyConfig;
use codex_protocol::openai_models::WebSearchToolType;
use std::io;

pub(super) fn bundled_models_for_provider(
    provider_id: Option<&str>,
    provider: &ModelProviderInfo,
) -> io::Result<Vec<ModelInfo>> {
    match provider_id.or_else(|| provider.known_provider_id()) {
        Some(DEEPSEEK_PROVIDER_ID) => Ok(deepseek_models()),
        Some(GLM_PROVIDER_ID) => Ok(glm_models()),
        Some(KIMI_PROVIDER_ID) => Ok(kimi_models()),
        Some(MINIMAX_PROVIDER_ID) => Ok(minimax_models()),
        _ => load_openai_catalog(),
    }
}

pub(super) fn resolve_model_lookup_alias(
    provider_id: Option<&str>,
    provider: &ModelProviderInfo,
    model: &str,
) -> Option<String> {
    fn deepseek_alias(slug: &str) -> Option<&'static str> {
        match slug {
            "deepseek-chat-thinking" | "deepseek-thinking" => Some("deepseek-reasoner"),
            _ => None,
        }
    }

    match provider_id.or_else(|| provider.known_provider_id()) {
        Some(DEEPSEEK_PROVIDER_ID) => {
            if let Some(alias) = deepseek_alias(model) {
                return Some(alias.to_string());
            }

            let (namespace, suffix) = model.split_once('/')?;
            if suffix.contains('/') {
                return None;
            }
            deepseek_alias(suffix).map(|alias| format!("{namespace}/{alias}"))
        }
        _ => None,
    }
}

fn load_openai_catalog() -> io::Result<Vec<ModelInfo>> {
    let file_contents = include_str!("../../models.json");
    let response: codex_protocol::openai_models::ModelsResponse =
        serde_json::from_str(file_contents)?;
    Ok(response.models)
}

fn chat_model(
    slug: &str,
    display_name: &str,
    description: &str,
    priority: i32,
    context_window: i64,
    input_modalities: Vec<InputModality>,
) -> ModelInfo {
    ModelInfo {
        slug: slug.to_string(),
        display_name: display_name.to_string(),
        description: Some(description.to_string()),
        default_reasoning_level: None,
        supported_reasoning_levels: vec![],
        shell_type: ConfigShellToolType::ShellCommand,
        visibility: ModelVisibility::List,
        supported_in_api: true,
        priority,
        availability_nux: None,
        upgrade: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: Some(ApplyPatchToolType::Function),
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::tokens(10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: input_modalities.contains(&InputModality::Image),
        context_window: Some(context_window),
        auto_compact_token_limit: None,
        effective_context_window_percent: 90,
        experimental_supported_tools: vec![],
        input_modalities,
        used_fallback_model_metadata: false,
        supports_search_tool: false,
    }
}

fn deepseek_models() -> Vec<ModelInfo> {
    vec![
        chat_model(
            "deepseek-chat",
            "DeepSeek Chat",
            "DeepSeek-V3.2 non-thinking mode with tool calling and 128K context.",
            0,
            128_000,
            vec![InputModality::Text],
        ),
        chat_model(
            "deepseek-reasoner",
            "DeepSeek Reasoner",
            "DeepSeek-V3.2 official thinking-mode alias with tool calling and 128K context.",
            1,
            128_000,
            vec![InputModality::Text],
        ),
    ]
}

fn glm_models() -> Vec<ModelInfo> {
    vec![
        chat_model(
            "glm-5",
            "GLM-5",
            "Zhipu AI flagship text model for coding, planning, and agent workflows with 200K context.",
            0,
            200_000,
            vec![InputModality::Text],
        ),
        chat_model(
            "glm-4.6v",
            "GLM-4.6V",
            "Zhipu AI flagship visual reasoning model with tool calling and 128K context.",
            1,
            128_000,
            vec![InputModality::Text, InputModality::Image],
        ),
    ]
}

fn kimi_models() -> Vec<ModelInfo> {
    vec![chat_model(
        "kimi-latest",
        "Kimi Latest",
        "Moonshot AI stable Kimi alias with tool calling, image understanding, and OpenAI-compatible chat completions.",
        0,
        128_000,
        vec![InputModality::Text, InputModality::Image],
    )]
}

fn minimax_models() -> Vec<ModelInfo> {
    vec![
        chat_model(
            "MiniMax-M2.7",
            "MiniMax-M2.7",
            "MiniMax flagship reasoning and coding model with 204,800-token context.",
            0,
            204_800,
            vec![InputModality::Text],
        ),
        chat_model(
            "MiniMax-M2.7-highspeed",
            "MiniMax-M2.7 Highspeed",
            "MiniMax-M2.7 high-speed variant with the same 204,800-token context.",
            1,
            204_800,
            vec![InputModality::Text],
        ),
    ]
}
