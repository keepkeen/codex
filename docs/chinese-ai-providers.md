# Chinese AI Model Providers

Codex now includes built-in support for popular Chinese AI model providers. This document explains how to configure and use them.

## Supported Providers

### DeepSeek
- **Provider ID**: `deepseek`
- **Base URL**: `https://api.deepseek.com`
- **Environment Variable**: `DEEPSEEK_API_KEY`
- **Get API Key**: https://platform.deepseek.com/api_keys
- **Models**:
  - `deepseek-chat`: DeepSeek-V3.2, 128K context, general purpose
  - `deepseek-reasoner`: DeepSeek-V3.2, 128K context, with reasoning (low/medium/high)
- **WebSocket Support**: ❌ No (automatically uses HTTP SSE)
- **Parallel Tool Calls**: ✅ Yes
- **Image Input**: ❌ No

### GLM (Zhipu AI / 智谱AI)
- **Provider ID**: `glm`
- **Base URL**: `https://open.bigmodel.cn/api/paas/v4`
- **Environment Variable**: `GLM_API_KEY`
- **Get API Key**: https://open.bigmodel.cn/usercenter/apikeys
- **Models**:
  - `glm-4-plus`: 128K context, multimodal (text + image)
  - `glm-4-flash`: 128K context, fast responses
- **WebSocket Support**: ❌ No (automatically uses HTTP SSE)
- **Parallel Tool Calls**: ✅ Yes
- **Image Input**: ✅ Yes (glm-4-plus only)

### Kimi (Moonshot AI / 月之暗面)
- **Provider ID**: `kimi`
- **Base URL**: `https://api.moonshot.cn/v1`
- **Environment Variable**: `KIMI_API_KEY`
- **Get API Key**: https://platform.moonshot.cn/console/api-keys
- **Models**:
  - `kimi-k2.5`: 256K context, multimodal, latest flagship (2026)
- **WebSocket Support**: ❌ No (automatically uses HTTP SSE)
- **Parallel Tool Calls**: ❌ No (executes tools sequentially)
- **Image Input**: ✅ Yes

### MiniMax
- **Provider ID**: `minimax`
- **Base URL**: `https://api.minimax.chat/v1`
- **Environment Variable**: `MINIMAX_API_KEY`
- **Get API Key**: https://www.minimaxi.com/user-center/basic-information/interface-key
- **Models**:
  - `minimax-m2.7`: 205K context, latest flagship with recursive self-improvement (2026)
- **WebSocket Support**: ❌ No (automatically uses HTTP SSE)
- **Parallel Tool Calls**: ✅ Yes
- **Image Input**: ❌ No

## Quick Start

### 1. Set Your API Key

Choose one of the providers and set the corresponding environment variable:

```bash
# For DeepSeek
export DEEPSEEK_API_KEY="your-api-key-here"

# For GLM
export GLM_API_KEY="your-api-key-here"

# For Kimi
export KIMI_API_KEY="your-api-key-here"

# For MiniMax
export MINIMAX_API_KEY="your-api-key-here"
```

### 2. Configure Codex

Add the provider configuration to your `~/.codex/config.toml`:

```toml
# Use DeepSeek as your model provider
model_provider = "deepseek"
model = "deepseek-chat"

# Or use DeepSeek Reasoner with reasoning
# model = "deepseek-reasoner"
# model_reasoning_effort = "medium"

# Or use GLM
# model_provider = "glm"
# model = "glm-4-plus"

# Or use Kimi (note: does not support parallel tool calls)
# model_provider = "kimi"
# model = "kimi-k2.5"

# Or use MiniMax
# model_provider = "minimax"
# model = "minimax-m2.7"
```

### 3. Start Using Codex

```bash
codex
```

## Advanced Configuration

You can customize provider settings in your config file:

```toml
[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com"
env_key = "DEEPSEEK_API_KEY"
wire_api = "responses"
request_max_retries = 3
stream_max_retries = 5

[model_providers.glm]
name = "GLM (Zhipu AI)"
base_url = "https://open.bigmodel.cn/api/paas/v4"
env_key = "GLM_API_KEY"
wire_api = "responses"

[model_providers.kimi]
name = "Kimi (Moonshot AI)"
base_url = "https://api.moonshot.cn/v1"
env_key = "KIMI_API_KEY"
wire_api = "responses"

[model_providers.minimax]
name = "MiniMax"
base_url = "https://api.minimax.chat/v1"
env_key = "MINIMAX_API_KEY"
wire_api = "responses"
```

### Memory System Configuration

You can configure the memory system to use Chinese models for extraction and consolidation:

```toml
[memories]
# Model for extracting memories from conversation history
extract_model = "deepseek-chat"

# Model for consolidating memories
consolidation_model = "deepseek-chat"
```

**Note**: If not specified, the memory system defaults to OpenAI models (`gpt-5.1-codex-mini` for extraction, `gpt-5.3-codex` for consolidation).

## Using with Profiles

You can create different profiles for different providers:

```toml
[profiles.deepseek-dev]
model_provider = "deepseek"
model = "deepseek-chat"

[profiles.glm-prod]
model_provider = "glm"
model = "glm-4-plus"

[profiles.kimi-long]
model_provider = "kimi"
model = "kimi-k2.5"
```

Then switch between profiles:

```bash
codex --profile deepseek-dev
codex --profile glm-prod
codex --profile kimi-long
```

## Troubleshooting

### API Key Not Found

If you see an error about missing API key:

1. Make sure you've set the environment variable
2. Restart your terminal or reload your shell configuration
3. Verify the variable is set: `echo $DEEPSEEK_API_KEY`

### Connection Issues

If you experience connection problems:

1. Check your network connection
2. Verify the API endpoint is accessible from your location
3. Check if you need to configure a proxy

### Model Not Available

If a specific model is not available:

1. Check the provider's documentation for the latest model names
2. Verify your API key has access to the model
3. Some models may require additional permissions or subscriptions

### WebSocket Errors

All Chinese providers use HTTP SSE (Server-Sent Events) instead of WebSocket. This is handled automatically by Codex - no action needed.

### Parallel Tool Call Issues (Kimi Only)

Kimi does not support parallel tool calls. The model instructions automatically tell the agent to execute tools sequentially. If you see errors related to parallel execution:

1. This is expected behavior for Kimi
2. The agent will automatically execute tools one at a time
3. Consider using DeepSeek, GLM, or MiniMax if parallel execution is important

### Image Input Not Working

Only GLM-4 Plus supports image input. If you need multimodal capabilities:

1. Use `model_provider = "glm"` and `model = "glm-4-plus"`
2. Other Chinese models only support text input

## Notes

- All providers use the OpenAI-compatible Responses API format
- API keys are read from environment variables for security
- These providers do not require OpenAI authentication
- **WebSocket support**: All Chinese providers use HTTP SSE (Server-Sent Events) transport instead of WebSocket. This is handled automatically by Codex with no performance impact.
- **Parallel tool calls**: Kimi does not support parallel tool execution. Other providers (DeepSeek, GLM, MiniMax) support it.
- **Image input**: Only GLM-4 Plus supports image input. Other models are text-only.
- **Reasoning**: Only DeepSeek Reasoner supports extended reasoning modes (low/medium/high).
- **Context windows**: MiniMax offers the longest context (245K), followed by GLM and Kimi (128K), and DeepSeek (64K).
