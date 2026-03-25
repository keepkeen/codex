# 切换到中国模型提供商的兼容性分析

本文档分析了将 Codex 从 OpenAI 模型切换到中国模型提供商（DeepSeek、GLM、Kimi、MiniMax）时需要注意的关键领域。

## 执行摘要

中国模型提供商已经通过 `model_provider_info.rs` 集成到 Codex 中，但要完全兼容，需要为每个模型配置特定的能力元数据。主要挑战在于：

1. **模型能力定义缺失** - 需要为每个中国模型创建 `ModelInfo` 配置
2. **硬编码的 GPT 模型引用** - 某些功能（如记忆系统）硬编码了特定的 GPT 模型
3. **功能支持差异** - 不同模型对工具调用、推理、图像等的支持程度不同

## 1. 模型能力配置 (ModelInfo)

### 位置
- `codex-rs/protocol/src/openai_models.rs` - `ModelInfo` 结构定义
- `codex-rs/core/models.json` - OpenAI 模型的能力配置

### 关键字段需要配置

```rust
pub struct ModelInfo {
    pub slug: String,                              // 模型标识符
    pub display_name: String,                      // 显示名称
    pub context_window: Option<i64>,               // 上下文窗口大小（tokens）
    pub supports_parallel_tool_calls: bool,        // 是否支持并行工具调用
    pub supports_search_tool: bool,                // 是否支持搜索工具
    pub supports_image_detail_original: bool,      // 是否支持原始图像细节
    pub input_modalities: Vec<InputModality>,      // 支持的输入模态（文本/图像）
    pub shell_type: ConfigShellToolType,           // Shell 工具类型
    pub apply_patch_tool_type: Option<ApplyPatchToolType>, // 补丁应用工具类型
    pub web_search_tool_type: WebSearchToolType,   // Web 搜索工具类型
    pub truncation_policy: TruncationPolicyConfig, // 截断策略
    pub default_reasoning_level: Option<ReasoningEffort>, // 默认推理级别
    pub supported_reasoning_levels: Vec<ReasoningEffortPreset>, // 支持的推理级别
    pub base_instructions: String,                 // 基础指令
    pub experimental_supported_tools: Vec<String>, // 实验性工具支持
    // ... 其他字段
}
```

### 需要为每个中国模型配置的参数

#### DeepSeek
```json
{
  "slug": "deepseek-chat",
  "display_name": "DeepSeek Chat",
  "context_window": 64000,
  "supports_parallel_tool_calls": true,
  "supports_search_tool": false,
  "supports_image_detail_original": false,
  "input_modalities": ["text"],
  "shell_type": "shell_command",
  "apply_patch_tool_type": "freeform",
  "truncation_policy": {"mode": "tokens", "limit": 10000},
  "default_reasoning_level": "medium",
  "supported_reasoning_levels": [
    {"effort": "low", "description": "快速响应"},
    {"effort": "medium", "description": "平衡速度和深度"}
  ]
}
```

#### GLM-4
```json
{
  "slug": "glm-4-plus",
  "display_name": "GLM-4 Plus",
  "context_window": 128000,
  "supports_parallel_tool_calls": true,
  "supports_search_tool": false,
  "supports_image_detail_original": true,
  "input_modalities": ["text", "image"],
  "shell_type": "shell_command",
  "apply_patch_tool_type": "freeform"
}
```

#### Kimi
```json
{
  "slug": "kimi-k2.5",
  "display_name": "Kimi K2.5",
  "context_window": 256000,
  "supports_parallel_tool_calls": false,
  "supports_search_tool": false,
  "supports_image_detail_original": true,
  "input_modalities": ["text", "image"],
  "shell_type": "shell_command"
}
```

#### MiniMax
```json
{
  "slug": "minimax-m2.7",
  "display_name": "MiniMax M2.7",
  "context_window": 205000,
  "supports_parallel_tool_calls": true,
  "supports_search_tool": false,
  "supports_image_detail_original": false,
  "input_modalities": ["text"],
  "shell_type": "shell_command"
}
```

## 2. 硬编码的模型引用

### 记忆系统 (Memory System)

**位置**: `codex-rs/core/src/memories/mod.rs`

```rust
mod phase_one {
    pub(super) const MODEL: &str = "gpt-5.1-codex-mini";  // ⚠️ 硬编码
    pub(super) const REASONING_EFFORT: super::ReasoningEffort = super::ReasoningEffort::Low;
}

mod phase_two {
    pub(super) const MODEL: &str = "gpt-5.3-codex";  // ⚠️ 硬编码
    pub(super) const REASONING_EFFORT: super::ReasoningEffort = super::ReasoningEffort::Medium;
}
```

**影响**:
- Phase 1: 启动时从历史会话中提取原始记忆
- Phase 2: 合并记忆摘要

**解决方案**:
1. 将这些常量改为配置项
2. 在 `config.toml` 中添加：
```toml
[memories]
phase_one_model = "deepseek-chat"
phase_one_reasoning_effort = "low"
phase_two_model = "deepseek-chat"
phase_two_reasoning_effort = "medium"
```

### 测试中的模型引用

**位置**: 多个测试文件
- `codex-rs/core/tests/suite/compact_resume_fork.rs:541`
- `codex-rs/core/tests/suite/client_websockets.rs:54`

这些是测试代码，不影响生产环境。

## 3. 工具支持 (Tool Support)

### 关键工具能力

**位置**: `codex-rs/core/src/tools/spec.rs`

```rust
// 并行工具调用支持
model_info.supports_parallel_tool_calls

// 搜索工具支持
model_info.supports_search_tool && features.enabled(Feature::ToolSearch)

// 实验性工具
model_info.experimental_supported_tools
```

### 各模型的工具支持建议

| 工具类型 | DeepSeek | GLM-4 | Kimi | MiniMax |
|---------|----------|-------|------|---------|
| 并行工具调用 | ✅ 是 | ✅ 是 | ❌ 否 | ✅ 是 |
| Shell 命令 | ✅ 是 | ✅ 是 | ✅ 是 | ✅ 是 |
| 文件操作 | ✅ 是 | ✅ 是 | ✅ 是 | ✅ 是 |
| 图像输入 | ❌ 否 | ✅ 是 | ❌ 否 | ❌ 否 |
| Web 搜索 | ❌ 否 | ❌ 否 | ❌ 否 | ❌ 否 |
| 代码补丁 | ✅ 是 | ✅ 是 | ✅ 是 | ✅ 是 |

### 工具注册

**位置**: `codex-rs/core/src/tools/registry.rs`

```rust
pub struct ConfiguredToolSpec {
    pub spec: ToolSpec,
    pub supports_parallel_tool_calls: bool,  // 从 ModelInfo 读取
}
```

## 4. MCP (Model Context Protocol) 支持

### 位置
- `codex-rs/core/src/mcp_connection_manager.rs`
- `codex-rs/rmcp-client/`

### 关键考虑

MCP 支持主要依赖于：
1. **工具调用能力** - 模型必须支持 function calling
2. **动态工具注册** - 运行时添加 MCP 服务器提供的工具
3. **工具批准流程** - 用户批准 MCP 工具的使用

**兼容性**: 所有中国模型都支持基本的 function calling，因此 MCP 应该可以工作。但需要测试：
- 工具调用的格式是否完全兼容 OpenAI 格式
- 复杂工具参数的处理
- 错误处理和重试逻辑

## 5. 会话管理 (Session Management)

### 位置
- `codex-rs/core/src/codex.rs`
- `codex-rs/state/src/model/thread_metadata.rs`

### 关键字段

```rust
pub struct ThreadMetadata {
    pub model: String,                           // 模型标识符
    pub reasoning_effort: Option<ReasoningEffort>, // 推理级别
    pub model_provider_id: String,               // 提供商 ID
    // ...
}
```

**兼容性**: 会话管理是模型无关的，只要正确设置 `model` 和 `model_provider_id` 即可。

## 6. 推理能力 (Reasoning Effort)

### 位置
- `codex-rs/protocol/src/openai_models.rs`

```rust
pub enum ReasoningEffort {
    None,
    Low,
    Medium,
    High,
    XHigh,
}
```

### 各模型的推理支持

| 模型 | 支持推理 | 推荐级别 |
|------|---------|---------|
| DeepSeek | ✅ 是 (deepseek-reasoner) | Low, Medium |
| GLM-4 | ❌ 否 | None |
| Kimi | ❌ 否 | None |
| MiniMax | ❌ 否 | None |

**注意**: DeepSeek 有专门的 `deepseek-reasoner` 模型支持推理，但其他中国模型目前不支持类似 o1 的推理模式。

## 7. 上下文管理和压缩

### 位置
- `codex-rs/core/src/codex.rs` - 自动压缩逻辑

```rust
impl ModelInfo {
    pub fn auto_compact_token_limit(&self) -> Option<i64> {
        let context_limit = self.context_window
            .map(|context_window| (context_window * 9) / 10);  // 90% 阈值
        // ...
    }
}
```

### 各模型的上下文窗口

| 模型 | 上下文窗口 | 建议压缩阈值 |
|------|-----------|-------------|
| deepseek-chat | 128K | 115.2K (90%) |
| glm-4-plus | 128K | 115.2K (90%) |
| kimi-k2.5 | 256K | 230.4K (90%) |
| minimax-m2.7 | 205K | 184.5K (90%) |

## 8. Skills 系统

### 位置
- `codex-rs/skills/src/`
- `codex-rs/core/src/skills/`

### 关键考虑

Skills 系统主要依赖于：
1. **提示词模板** - 存储在 `codex-rs/skills/src/assets/samples/`
2. **模型指令** - 从 `ModelInfo.base_instructions` 读取

**兼容性**: Skills 系统是模型无关的，但某些技能可能针对 GPT 模型优化。需要：
- 测试各个技能在中国模型上的表现
- 可能需要调整提示词以适应不同模型的特性

## 9. 需要修改的文件清单

### 必须修改
1. **`codex-rs/core/models.json`** - 添加中国模型的配置
2. **`codex-rs/core/src/memories/mod.rs`** - 将硬编码模型改为配置项
3. **`codex-rs/core/src/config/types.rs`** - 添加记忆系统的模型配置选项

### 建议修改
4. **`codex-rs/core/src/models_manager/model_info.rs`** - 添加中国模型的回退元数据
5. **`codex-rs/core/src/tools/spec.rs`** - 确保工具规范适配中国模型
6. **`docs/chinese-ai-providers.md`** - 更新文档说明模型能力差异

### 可选修改
7. **`codex-rs/skills/src/assets/samples/`** - 为中国模型优化提示词模板
8. **测试文件** - 添加中国模型的集成测试

## 10. 实施步骤

### 阶段 1: 基础配置（已完成）
- ✅ 添加提供商定义到 `model_provider_info.rs`
- ✅ 创建文档和示例配置

### 阶段 2: 模型元数据配置（需要实施）
1. 为每个中国模型创建 `models.json` 条目
2. 配置上下文窗口、工具支持等能力
3. 设置合理的截断策略

### 阶段 3: 记忆系统适配（需要实施）
1. 将硬编码的模型引用改为配置项
2. 在 `config.toml` 中添加记忆系统模型配置
3. 测试记忆提取和合并功能

### 阶段 4: 测试和优化（需要实施）
1. 测试基本的对话功能
2. 测试工具调用（文件操作、shell 命令等）
3. 测试 MCP 集成
4. 测试会话恢复和分支
5. 性能优化和提示词调整

## 11. 风险和限制

### 高风险区域
1. **工具调用格式** - 中国模型可能不完全兼容 OpenAI 的 function calling 格式
2. **推理能力** - 除 DeepSeek 外，其他模型不支持推理模式
3. **图像支持** - 只有 GLM-4 支持图像输入

### 已知限制
1. **WebSocket 支持** - 中国模型不支持 WebSocket 传输
2. **搜索工具** - 中国模型不支持内置的 web 搜索
3. **记忆系统** - 需要配置才能使用中国模型进行记忆处理

### 缓解策略
1. 在文档中明确说明各模型的能力差异
2. 提供回退机制（例如，记忆系统可以继续使用 OpenAI 模型）
3. 添加详细的错误消息和故障排除指南

## 12. 总结

切换到中国模型提供商是可行的，但需要：

1. **必须**: 为每个模型配置 `ModelInfo` 元数据
2. **必须**: 修改记忆系统的硬编码模型引用
3. **建议**: 全面测试工具调用和 MCP 集成
4. **建议**: 为不同模型优化提示词

最大的挑战是确保工具调用格式的兼容性和处理各模型能力的差异。建议采用渐进式方法，先支持基本功能，然后逐步添加高级特性。
