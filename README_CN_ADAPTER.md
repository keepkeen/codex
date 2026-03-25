# Codex 中国 AI 模型适配器

> 为 OpenAI Codex CLI 添加中国主流 AI 模型提供商支持

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 📋 目录

- [简介](#简介)
- [支持的模型](#支持的模型)
- [快速开始](#快速开始)
- [详细配置](#详细配置)
- [功能特性](#功能特性)
- [局限性](#局限性)
- [技术实现](#技术实现)
- [贡献](#贡献)

## 简介

本项目为 [OpenAI Codex CLI](https://github.com/openai/codex) 添加了对中国主流 AI 模型提供商的原生支持。通过最小化的代码修改，实现了与 4 家中国 AI 提供商的无缝集成，让中国开发者可以使用本地 AI 服务来运行 Codex。

### 适配的提供商

- **DeepSeek** - 深度求索（高性价比，强推理能力）
- **GLM** - 智谱 AI（多模态支持）
- **Kimi** - 月之暗面（超长上下文）
- **MiniMax** - MiniMax（递归自我改进）

### 为什么需要这个适配器？

1. **降低成本** - 中国 AI 服务价格通常更低
2. **提升速度** - 国内网络访问更快，无需代理
3. **数据合规** - 数据不出境，符合数据安全要求
4. **模型多样性** - 可以使用不同特性的模型（超长上下文、多模态等）

## 支持的模型

### DeepSeek（深度求索）

| 模型 | 上下文 | 特性 | 价格（每百万 tokens） |
|------|--------|------|---------------------|
| `deepseek-chat` | 128K | V3.2，通用编程 | ¥1.0 / ¥2.0 |
| `deepseek-reasoner` | 128K | V3.2，推理模式 | ¥1.0 / ¥2.0 |

**特点**：
- ✅ 支持并行工具调用
- ✅ 支持推理模式（low/medium/high）
- ✅ 性价比极高
- ❌ 不支持图像输入

### GLM（智谱 AI）

| 模型 | 上下文 | 特性 | 价格 |
|------|--------|------|------|
| `glm-4-plus` | 128K | 多模态（文本+图像） | 中等 |
| `glm-4-flash` | 128K | 快速响应 | 较低 |

**特点**：
- ✅ 支持并行工具调用
- ✅ glm-4-plus 支持图像输入
- ✅ 响应速度快
- ❌ 上下文窗口相对较小

### Kimi（月之暗面）

| 模型 | 上下文 | 特性 | 价格 |
|------|--------|------|------|
| `kimi-k2.5` | 256K | 多模态，超长上下文 | 中等 |

**特点**：
- ✅ 最长上下文窗口（256K）
- ✅ 支持图像输入
- ✅ Agent Swarm 技术
- ❌ **不支持并行工具调用**（自动顺序执行）

### MiniMax

| 模型 | 上下文 | 特性 | 价格 |
|------|--------|------|------|
| `minimax-m2.7` | 205K | 递归自我改进 | 中等 |

**特点**：
- ✅ 支持并行工具调用
- ✅ 超长上下文（205K）
- ✅ SWE-Bench 80.2% 得分
- ❌ 不支持图像输入

## 快速开始

### 1. 安装 Codex CLI

```bash
# 使用 npm
npm install -g @openai/codex

# 或使用 Homebrew
brew install --cask codex
```

### 2. 克隆本适配器

```bash
git clone https://github.com/keepkeen/codex_adapter_CN.git
cd codex_adapter_CN
```

### 3. 构建适配版本

```bash
cd codex-rs
cargo build --release
```

构建产物位于：`codex-rs/target/release/codex`

### 4. 获取 API Key

选择一个提供商并获取 API Key：

- **DeepSeek**: https://platform.deepseek.com/api_keys
- **GLM**: https://open.bigmodel.cn/usercenter/apikeys
- **Kimi**: https://platform.moonshot.cn/console/api-keys
- **MiniMax**: https://www.minimaxi.com/user-center/basic-information/interface-key

### 5. 配置环境变量

```bash
# 选择一个提供商设置 API Key
export DEEPSEEK_API_KEY="your-api-key-here"
# 或
export GLM_API_KEY="your-api-key-here"
# 或
export KIMI_API_KEY="your-api-key-here"
# 或
export MINIMAX_API_KEY="your-api-key-here"
```

### 6. 配置 Codex

编辑 `~/.codex/config.toml`：

```toml
# 使用 DeepSeek（推荐，性价比最高）
model_provider = "deepseek"
model = "deepseek-chat"

# 或使用 GLM（多模态）
# model_provider = "glm"
# model = "glm-4-plus"

# 或使用 Kimi（超长上下文）
# model_provider = "kimi"
# model = "kimi-k2.5"

# 或使用 MiniMax
# model_provider = "minimax"
# model = "minimax-m2.7"
```

### 7. 开始使用

```bash
./codex-rs/target/release/codex
```

## 详细配置

### DeepSeek 配置示例

```toml
# ~/.codex/config.toml

model_provider = "deepseek"
model = "deepseek-chat"

# 使用推理模式
# model = "deepseek-reasoner"
# model_reasoning_effort = "medium"  # low, medium, high

# 可选：自定义提供商设置
[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com"
env_key = "DEEPSEEK_API_KEY"
wire_api = "responses"
```

### GLM 配置示例

```toml
model_provider = "glm"
model = "glm-4-plus"  # 支持图像输入

# 或使用快速版本
# model = "glm-4-flash"

[model_providers.glm]
name = "GLM (Zhipu AI)"
base_url = "https://open.bigmodel.cn/api/paas/v4"
env_key = "GLM_API_KEY"
wire_api = "responses"
```

### Kimi 配置示例

```toml
model_provider = "kimi"
model = "kimi-k2.5"

# 注意：Kimi 不支持并行工具调用
# 系统会自动顺序执行工具

[model_providers.kimi]
name = "Kimi (Moonshot AI)"
base_url = "https://api.moonshot.cn/v1"
env_key = "KIMI_API_KEY"
wire_api = "responses"
```

### MiniMax 配置示例

```toml
model_provider = "minimax"
model = "minimax-m2.7"

[model_providers.minimax]
name = "MiniMax"
base_url = "https://api.minimax.chat/v1"
env_key = "MINIMAX_API_KEY"
wire_api = "responses"
```

### 配置记忆系统

可以指定用于记忆提取和整合的模型：

```toml
model_provider = "deepseek"
model = "deepseek-chat"

[memories]
extract_model = "deepseek-chat"        # 用于提取记忆
consolidation_model = "deepseek-chat"  # 用于整合记忆
```

如果不配置，记忆系统会默认使用 OpenAI 模型。

### 使用配置文件（Profiles）

可以为不同场景创建不同的配置：

```toml
# 开发环境 - 使用 DeepSeek（便宜）
[profiles.dev]
model_provider = "deepseek"
model = "deepseek-chat"

# 生产环境 - 使用 GLM（多模态）
[profiles.prod]
model_provider = "glm"
model = "glm-4-plus"

# 长文本处理 - 使用 Kimi（256K 上下文）
[profiles.long-context]
model_provider = "kimi"
model = "kimi-k2.5"

# 推理任务 - 使用 DeepSeek Reasoner
[profiles.reasoning]
model_provider = "deepseek"
model = "deepseek-reasoner"
model_reasoning_effort = "high"
```

使用时指定 profile：

```bash
codex --profile dev
codex --profile prod
codex --profile long-context
```

## 功能特性

### ✅ 已实现的功能

1. **原生提供商支持**
   - 4 个中国 AI 提供商内置支持
   - 无需额外配置代理或转发服务
   - 与 OpenAI 提供商同等地位

2. **完整的模型能力配置**
   - 准确的上下文窗口大小
   - 正确的工具调用支持标识
   - 多模态能力标识
   - 推理模式支持

3. **自动协议适配**
   - 自动使用 HTTP SSE（所有中国提供商不支持 WebSocket）
   - 自动处理 Kimi 的顺序工具调用限制
   - 兼容 OpenAI 的 API 格式

4. **记忆系统支持**
   - 可配置任意模型用于记忆提取
   - 可配置任意模型用于记忆整合
   - 向后兼容（默认使用 OpenAI）

5. **完整的文档**
   - 中文用户指南
   - 配置示例
   - 技术实现文档
   - 更新维护指南

### 🎯 核心优势

1. **最小化修改**
   - 仅修改 4 个核心文件
   - 不破坏原有功能
   - 易于维护和更新

2. **向后兼容**
   - 不影响现有 OpenAI 配置
   - 不影响其他提供商
   - 平滑升级路径

3. **生产就绪**
   - 所有单元测试通过
   - 完整的错误处理
   - 详细的日志输出

## 局限性

### ⚠️ 当前限制

1. **WebSocket 不支持**
   - **影响**：所有中国提供商使用 HTTP SSE 而非 WebSocket
   - **原因**：中国提供商 API 不支持 WebSocket 协议
   - **解决方案**：系统自动回退到 HTTP SSE，功能完全正常
   - **用户影响**：无（透明处理）

2. **Kimi 不支持并行工具调用**
   - **影响**：Kimi 模型只能顺序执行工具
   - **原因**：Kimi API 限制
   - **解决方案**：模型配置中设置 `supports_parallel_tool_calls: false`，系统自动顺序执行
   - **用户影响**：使用 Kimi 时工具执行速度较慢

3. **部分模型不支持多模态**
   - **影响**：DeepSeek 和 MiniMax 不支持图像输入
   - **原因**：模型能力限制
   - **解决方案**：使用 GLM-4 Plus 或 Kimi K2.5 处理图像
   - **用户影响**：需要根据任务选择合适的模型

4. **记忆系统默认使用 OpenAI**
   - **影响**：如果不配置，记忆功能仍需要 OpenAI API Key
   - **原因**：保持向后兼容
   - **解决方案**：在配置中显式指定中国模型
   - **用户影响**：需要额外配置

5. **需要手动构建**
   - **影响**：不能直接使用官方发布的二进制文件
   - **原因**：这是一个第三方适配
   - **解决方案**：提供详细的构建说明
   - **用户影响**：需要本地编译（约 5-10 分钟）

### 🔄 与官方版本的差异

| 特性 | 官方 Codex | 本适配器 |
|------|-----------|---------|
| OpenAI 模型 | ✅ | ✅ |
| Anthropic 模型 | ✅ | ✅ |
| 本地模型（Ollama） | ✅ | ✅ |
| 中国 AI 模型 | ❌ | ✅ |
| WebSocket 支持 | ✅ | ✅（仅 OpenAI/Anthropic） |
| 自动更新 | ✅ | ❌（需手动更新） |

### 🚧 已知问题

1. **API 兼容性**
   - 中国提供商的 API 可能与 OpenAI 有细微差异
   - 某些高级功能可能不完全兼容
   - 建议在生产环境前充分测试

2. **错误处理**
   - 不同提供商的错误消息格式可能不同
   - 某些错误可能显示为通用错误

3. **性能差异**
   - 不同模型的响应速度差异较大
   - 建议根据任务类型选择合适的模型

### 💡 使用建议

1. **模型选择**
   - 日常开发：DeepSeek Chat（性价比最高）
   - 复杂推理：DeepSeek Reasoner
   - 图像处理：GLM-4 Plus 或 Kimi K2.5
   - 长文本：Kimi K2.5（256K 上下文）
   - 代码生成：MiniMax M2.7（SWE-Bench 高分）

2. **成本优化**
   - 记忆系统使用便宜的模型（如 DeepSeek）
   - 简单任务使用 GLM-4 Flash
   - 避免在 Kimi 上使用大量工具调用

3. **性能优化**
   - 避免在 Kimi 上使用需要大量并行工具调用的任务
   - 使用 Profile 快速切换不同场景的配置
   - 合理设置上下文窗口阈值

## 技术实现

### 修改的文件

1. **`codex-rs/core/src/model_provider_info.rs`**
   - 添加 4 个提供商常量
   - 实现提供商工厂函数
   - 注册到内置提供商列表

2. **`codex-rs/core/src/lib.rs`**
   - 导出提供商 ID 常量

3. **`codex-rs/core/chinese_models.json`**（新增）
   - 6 个模型的完整元数据
   - 基于 2026 年最新规格

4. **`codex-rs/core/src/models_manager/manager.rs`**
   - 扩展模型加载逻辑
   - 合并中国模型到主目录

### 设计原则

1. **最小侵入**：仅修改必要的文件，不改变核心架构
2. **向后兼容**：不影响现有功能和配置
3. **可维护性**：清晰的代码结构，易于更新
4. **可扩展性**：易于添加新的提供商和模型

### 测试覆盖

```bash
# 运行所有相关测试
cd codex-rs
cargo test -p codex-core --lib models_manager
cargo test -p codex-core --lib model_provider_info

# 检查编译
cargo check -p codex-core
```

所有测试均通过 ✅

## 更新维护

### 如何更新模型配置

当提供商发布新模型时，只需更新 `codex-rs/core/chinese_models.json`：

```json
{
  "slug": "new-model-name",
  "display_name": "New Model Display Name",
  "context_window": 128000,
  "supports_parallel_tool_calls": true,
  "input_modalities": ["text"],
  ...
}
```

详见：`docs/chinese-ai-providers-update-guide.md`

### 如何同步官方更新

```bash
# 添加官方仓库为上游
git remote add upstream https://github.com/openai/codex.git

# 拉取官方更新
git fetch upstream
git merge upstream/main

# 解决冲突（如果有）
# 重新测试
cargo test -p codex-core
```

## 贡献

欢迎贡献！可以通过以下方式参与：

1. **报告问题**：发现 bug 或兼容性问题
2. **添加提供商**：支持更多中国 AI 提供商
3. **改进文档**：完善使用说明和示例
4. **性能优化**：提升适配器性能

### 开发指南

```bash
# 克隆仓库
git clone https://github.com/keepkeen/codex_adapter_CN.git
cd codex_adapter_CN

# 安装依赖
cd codex-rs
cargo build

# 运行测试
cargo test -p codex-core

# 格式化代码
cargo fmt

# 检查代码
cargo clippy
```

## 许可证

本项目遵循 Apache 2.0 许可证，与 OpenAI Codex 保持一致。

## 致谢

- [OpenAI Codex](https://github.com/openai/codex) - 原始项目
- DeepSeek、智谱 AI、月之暗面、MiniMax - 提供优秀的 AI 服务

## 联系方式

- GitHub Issues: https://github.com/keepkeen/codex_adapter_CN/issues
- 项目主页: https://github.com/keepkeen/codex_adapter_CN

---

**注意**：本项目是非官方的第三方适配器，与 OpenAI 无关。使用前请确保遵守各 AI 提供商的服务条款。
