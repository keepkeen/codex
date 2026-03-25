# Chinese AI Providers

This fork includes built-in adapters for four Chinese model families and routes them through the OpenAI-compatible `chat/completions` path instead of the OpenAI Responses API.

The details below reflect the implementation and the official provider docs reviewed for March 25, 2026.

## Built-in Providers

### DeepSeek
- Provider ID: `deepseek`
- Base URL: `https://api.deepseek.com`
- API key env: `DEEPSEEK_API_KEY`
- Built-in models:
  - `deepseek-chat`
  - `deepseek-reasoner`
- Notes:
  - Uses HTTP streaming, not websockets.
  - `deepseek-reasoner` is the official thinking-mode alias.
  - Legacy compatibility aliases `deepseek-chat-thinking` and `deepseek-thinking` are still accepted internally and mapped onto DeepSeek thinking mode.

### GLM
- Provider ID: `glm`
- Base URL: `https://open.bigmodel.cn/api/paas/v4`
- API key env: `GLM_API_KEY`
- Built-in models:
  - `glm-5`
  - `glm-4.6v`
- Notes:
  - `glm-5` is used as the default text flagship.
  - `glm-4.6v` is exposed as the built-in multimodal option.

### Kimi / Moonshot
- Provider ID: `kimi`
- Base URL: `https://api.moonshot.cn/v1`
- API key env: `MOONSHOT_API_KEY`
- Legacy env fallback: `KIMI_API_KEY`
- Built-in models:
  - `kimi-latest`
- Notes:
  - The built-in catalog intentionally uses the stable alias instead of hard-wiring a preview-only K2 variant.
  - The adapter treats Kimi as chat-completions compatible and enables image input metadata for the built-in model.

### MiniMax
- Provider ID: `minimax`
- Base URL: `https://api.minimaxi.com/v1`
- API key env: `MINIMAX_API_KEY`
- Built-in models:
  - `MiniMax-M2.7`
  - `MiniMax-M2.7-highspeed`
- Notes:
  - The adapter enables MiniMax `reasoning_split=true` so reasoning traces are preserved across tool loops.

## Quick Start

```toml
model_provider = "deepseek"
model = "deepseek-chat"
```

Other built-in combinations:

```toml
model_provider = "deepseek"
model = "deepseek-reasoner"
```

```toml
model_provider = "glm"
model = "glm-5"
```

```toml
model_provider = "kimi"
model = "kimi-latest"
```

```toml
model_provider = "minimax"
model = "MiniMax-M2.7"
```

## Custom Endpoints

Built-in provider IDs are reserved. If you need a proxy, gateway, or custom headers, create a new provider ID and point `model_provider` at that custom entry:

```toml
model_provider = "deepseek-proxy"
model = "deepseek-chat"

[model_providers.deepseek-proxy]
name = "DeepSeek via proxy"
base_url = "https://your-proxy.example.com/v1"
env_key = "DEEPSEEK_API_KEY"
wire_api = "chat"
```

For custom providers that are not one of the built-in IDs, Codex can still talk to the endpoint, but the best built-in model metadata is only guaranteed for the reserved built-in provider IDs.

## Behavioral Notes

- These providers use HTTP SSE transport. Websocket prewarm is disabled.
- `output_schema` is not supported on the chat-completions path.
- Function tools, shell, and `apply_patch` are re-exposed through chat-completions-compatible function calling.
- DeepSeek, Kimi, and MiniMax reasoning traces are preserved when the provider exposes them through `reasoning_content` or `reasoning_details`.
