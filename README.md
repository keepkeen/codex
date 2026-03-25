<p align="center"><code>npm i -g @openai/codex</code><br />or <code>brew install --cask codex</code></p>
<p align="center"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/codex/blob/main/.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>
</br>
If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>codex app</code> or visit <a href="https://chatgpt.com/codex?app-landing-page=true">the Codex App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Quickstart

### Installing and running Codex CLI

Install globally with your preferred package manager:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask codex
```

Then simply run `codex` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codex-x86_64-unknown-linux-musl`), so you likely want to rename it to `codex` after extracting it.

</details>

### Using Codex with your ChatGPT plan

Run `codex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Codex as part of your Plus, Pro, Team, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codex-in-chatgpt).

You can also use Codex with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## 中文适配说明

这个 fork 额外适配了 4 家中国模型 provider，并统一走官方更稳定的 OpenAI 兼容 `chat/completions` 路线，而不是把这些 provider 伪装成 OpenAI `responses`。

- 已适配 provider：`deepseek`、`glm`、`kimi`、`minimax`
- 内置模型：
  - DeepSeek：`deepseek-chat`、`deepseek-reasoner`
  - GLM：`glm-5`、`glm-4.6v`
  - Kimi：`kimi-latest`
  - MiniMax：`MiniMax-M2.7`、`MiniMax-M2.7-highspeed`
- 兼容原理：
  - 把 Codex 的工具调用和对话历史重编码成 chat-completions 兼容格式
  - 针对 DeepSeek / Kimi / MiniMax 保留各家的 reasoning 字段
  - 按 provider 载入独立模型目录，避免错误回退到 OpenAI 元数据
  - 兼容旧别名 `deepseek-chat-thinking` / `deepseek-thinking`，以及旧环境变量 `KIMI_API_KEY`
- 与原仓库不同的点：
  - 上游默认围绕 OpenAI / ChatGPT 登录和 `responses` 路径；这个 fork 为 4 家中国 provider 提供了内置 provider 和内置模型目录
  - 中国 provider 统一改走 `chat/completions` + SSE 兼容层，不依赖 OpenAI 的远端 `responses` / realtime 路径
  - 非 OpenAI provider 的上下文压缩走本地 compact 流程，不走 OpenAI 远端 compact task
  - 模型元数据改为按当前 provider 加载，不再把 DeepSeek / GLM / Kimi / MiniMax 回退到 OpenAI 模型元数据
- 最小使用方式：

```toml
model_provider = "deepseek"
model = "deepseek-chat"
```

- 需要设置对应环境变量：`DEEPSEEK_API_KEY`、`GLM_API_KEY`、`MOONSHOT_API_KEY`、`MINIMAX_API_KEY`
- Prompt / Agent 工作流：
  - `orchestrator`、`collaboration`、`compact`、`memories` 这些核心 prompt 基本通用，不需要为四家 provider 单独改写
  - 子代理、MCP、绝大多数 skills 仍可用，因为它们最终走 function tools
- 当前限制：
  - 这 4 家都走 HTTP SSE，不走 websocket 预热
  - `output_schema` 目前不走 chat-completions 兼容层
  - 当前 chat 兼容层只完整暴露 function tools，所以 `js_repl`、artifact 类工具、模型侧 `web_search` / `image_generation`、并行 tool calls 还不是和 OpenAI 原生路径完全等价
  - 图像输入只内置给 `glm-4.6v` 和 `kimi-latest`
  - `deepseek-chat`、`deepseek-reasoner`、`glm-5`、`MiniMax-M2.7`、`MiniMax-M2.7-highspeed` 按当前适配视为文本模型
  - 如果要走代理、自定义 header 或网关，不要覆盖内置 provider ID；新建自定义 provider，并把 `wire_api = "chat"`

更完整的中文说明见 [README_CN_ADAPTER.md](./README_CN_ADAPTER.md) 和 [docs/chinese-ai-providers.md](./docs/chinese-ai-providers.md)。

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
