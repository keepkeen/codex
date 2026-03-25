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

## 与原仓库不同的点

- 上游仓库的默认主路径是 OpenAI / ChatGPT 登录，以及 OpenAI 自己的 `responses` 能力；这个分支额外内置了 `deepseek`、`glm`、`kimi`、`minimax` 4 个 provider。
- 这 4 家 provider 不再伪装成 OpenAI Responses API，而是统一走官方更稳定的 `chat/completions` 兼容层和 HTTP SSE 流式返回。
- 非 OpenAI provider 的 compact 不走 OpenAI 远端 compact task，而是继续使用本地 compact 路径。
- 模型目录和元数据改成按当前 provider 单独加载，避免中国 provider 误回退到 OpenAI 内置模型元数据。
- 额外做了向后兼容：
  - DeepSeek 旧别名 `deepseek-chat-thinking` / `deepseek-thinking`
  - Kimi 旧环境变量 `KIMI_API_KEY`

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

## Prompt / Agent 工作流是否通用

- 结论：大多数核心 prompt 是通用的，不需要专门为这 4 家 provider 重写。
- 已检查的模板包括：
  - `orchestrator`
  - `collaboration_mode`
  - `compact`
  - `memories`
- 这些模板本质上描述的是协作方式、上下文压缩、记忆整理、工具调用纪律，并不依赖 OpenAI 专有措辞才能工作。
- 真正的兼容边界主要不在 prompt，而在工具协议层：
  - 当前 `chat/completions` 兼容层只完整暴露 function tools
  - 所以子代理、MCP、绝大多数 skills、记忆压缩/记忆 consolidation 这类 function-tool 工作流基本可用
  - 但 `js_repl`、artifact 类状态型工具、模型侧 `web_search`、`image_generation` 这类非 function / 特殊 built-in tool 还不是完全等价路径
- 这意味着：prompt 文本本身大体通用，但如果某个 workflow 强依赖 freeform 或状态型 built-in tool，它在这 4 家 provider 下仍可能受限。

## 哪些模型哪些功能不能用

- 所有这 4 家 provider：
  - 都走 HTTP SSE，不走 websocket 预热
  - 当前都不支持 `output_schema` 这条 chat-completions 兼容路径
  - 当前都不启用并行 tool calls
  - `js_repl`、artifact 类工具、模型侧 `web_search` / `image_generation` 还不是和 OpenAI 原生路径完全等价
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
