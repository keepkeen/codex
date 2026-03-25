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
- 最小使用方式：

```toml
model_provider = "deepseek"
model = "deepseek-chat"
```

- 需要设置对应环境变量：`DEEPSEEK_API_KEY`、`GLM_API_KEY`、`MOONSHOT_API_KEY`、`MINIMAX_API_KEY`
- 当前限制：
  - 这 4 家都走 HTTP SSE，不走 websocket 预热
  - `output_schema` 目前不走 chat-completions 兼容层
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
