# Codex 中国模型适配说明

这个分支把 Codex 对 DeepSeek、GLM、Kimi、MiniMax 的接入改成了官方更稳的 `chat/completions` 兼容路线，而不是把这些 provider 伪装成 OpenAI Responses API。

截至 2026-03-25，代码里内置了以下 provider：

- `deepseek`
- `glm`
- `kimi`
- `minimax`

默认内置模型目录：

- DeepSeek: `deepseek-chat`, `deepseek-reasoner`
- GLM: `glm-5`, `glm-4.6v`
- Kimi: `kimi-latest`
- MiniMax: `MiniMax-M2.7`, `MiniMax-M2.7-highspeed`

最小配置：

```toml
model_provider = "deepseek"
model = "deepseek-chat"
```

其他常用组合：

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

环境变量：

```bash
export DEEPSEEK_API_KEY="..."
export GLM_API_KEY="..."
export MOONSHOT_API_KEY="..."
export MINIMAX_API_KEY="..."
```

兼容性说明：

- Kimi 也兼容旧环境变量 `KIMI_API_KEY`。
- 这四家 provider 都走 HTTP SSE，不走 websocket 预热。
- `output_schema` 目前不走 chat-completions 适配层。
- 如果需要代理或自定义 header，不要覆盖保留的内置 provider ID；请新建一个自定义 provider ID，并把 `wire_api` 设成 `chat`。

更完整的配置说明见 [docs/chinese-ai-providers.md](docs/chinese-ai-providers.md)。
