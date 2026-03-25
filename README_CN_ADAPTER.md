# Codex 中国模型适配说明

这个分支把 Codex 对 DeepSeek、GLM、Kimi、MiniMax 的接入改成了官方更稳的 `chat/completions` 兼容路线，而不是把这些 provider 伪装成 OpenAI Responses API。

截至 2026-03-25，代码里内置了以下 provider：

- `deepseek`
- `glm`
- `kimi`
- `minimax`

## 这次改了什么

- 增加了 4 家中国 provider 的内置接入，不需要用户自己手写 provider 配置。
- 把原先不可靠的 Responses API 假设改成了 `chat/completions` 兼容层。
- 为工具调用、对话历史、reasoning 内容、流式 SSE 事件做了 provider 兼容转换。
- 修复了 `ThreadManager` 生产路径里错误使用 OpenAI 模型目录的问题，保证不同 provider 会载入各自的模型元数据。
- 删除了旧的 `chinese_models.json` 和重复/过期文档，改成 provider 独立模型目录。

## 适配原理

- 统一把 Codex 内部消息和工具调用重编码成 OpenAI 兼容 `chat/completions` 请求。
- 对不同 provider 的 reasoning 字段做兼容：
  - DeepSeek / Kimi：保留 `reasoning_content`
  - MiniMax：自动加 `reasoning_split=true` 并保留 `reasoning_details`
- SSE 解析层按 chat-completions 流式格式接收 delta、tool call 和 reasoning 内容。
- 模型目录按 provider 独立加载，避免 DeepSeek / GLM / Kimi / MiniMax 的模型错误回退到 OpenAI 默认元数据。
- 向后兼容旧配置：
  - `deepseek-chat-thinking`
  - `deepseek-thinking`
  - `KIMI_API_KEY`

## 内置模型

- DeepSeek: `deepseek-chat`, `deepseek-reasoner`
- GLM: `glm-5`, `glm-4.6v`
- Kimi: `kimi-latest`
- MiniMax: `MiniMax-M2.7`, `MiniMax-M2.7-highspeed`

## 如何使用

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

对应环境变量：

```bash
export DEEPSEEK_API_KEY="..."
export GLM_API_KEY="..."
export MOONSHOT_API_KEY="..."
export MINIMAX_API_KEY="..."
```

## 兼容性说明

- Kimi 除了 `MOONSHOT_API_KEY`，也兼容旧环境变量 `KIMI_API_KEY`。
- DeepSeek 旧模型别名 `deepseek-chat-thinking` / `deepseek-thinking` 会自动映射到新的 thinking 路径。
- 如果需要代理、自定义 header 或网关，不要覆盖保留的内置 provider ID；请新建一个自定义 provider ID，并把 `wire_api` 设成 `chat`。

## 哪些模型哪些功能不能用

- 所有这 4 家 provider：
  - 都走 HTTP SSE，不走 websocket 预热
  - 当前都不支持 `output_schema` 这条 chat-completions 兼容路径
- 仅支持文本输入：
  - `deepseek-chat`
  - `deepseek-reasoner`
  - `glm-5`
  - `MiniMax-M2.7`
  - `MiniMax-M2.7-highspeed`
- 支持图像输入：
  - `glm-4.6v`
  - `kimi-latest`

## 建议

- 纯文本编码/Agent 工作流：优先 `deepseek-chat`、`deepseek-reasoner`、`glm-5`、`MiniMax-M2.7`
- 需要图像理解：优先 `glm-4.6v` 或 `kimi-latest`
- 需要代理或企业网关：新建自定义 provider，保留内置 provider 作为官方默认配置

更完整的配置说明见 [docs/chinese-ai-providers.md](docs/chinese-ai-providers.md)。
