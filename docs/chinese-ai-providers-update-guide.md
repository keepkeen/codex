# 中国模型提供商配置指南

## 重要提示

本文档中的模型信息可能不是最新的。请访问各提供商的官方文档获取最新信息：

- **DeepSeek**: https://platform.deepseek.com/api-docs
- **GLM (智谱AI)**: https://open.bigmodel.cn/dev/api
- **Kimi (Moonshot)**: https://platform.moonshot.cn/docs
- **MiniMax**: https://www.minimaxi.com/document/guides/chat-model/V2

## 如何更新模型配置

### 方法 1: 在 config.toml 中直接配置（推荐）

你可以在 `~/.codex/config.toml` 中覆盖默认配置：

```toml
# 配置提供商
[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com"  # 从官方文档获取最新端点
env_key = "DEEPSEEK_API_KEY"
wire_api = "responses"

# 使用提供商
model_provider = "deepseek"
model = "deepseek-chat"  # 从官方文档获取最新模型名称
```

### 方法 2: 更新 chinese_models.json

如果你需要添加新模型或更新模型能力，编辑 `codex-rs/core/chinese_models.json`：

```json
{
  "models": [
    {
      "slug": "your-model-name",
      "display_name": "显示名称",
      "description": "模型描述",
      "context_window": 128000,
      "supports_parallel_tool_calls": true,
      "supports_image_detail_original": false,
      "input_modalities": ["text"],
      "shell_type": "shell_command",
      "visibility": "list",
      "supported_in_api": true,
      "priority": 100,
      "base_instructions": "...",
      "truncation_policy": {
        "mode": "tokens",
        "limit": 10000
      }
    }
  ]
}
```

## 需要从官方文档确认的关键信息

### 1. API 端点
- DeepSeek: `base_url`
- GLM: `base_url`
- Kimi: `base_url`
- MiniMax: `base_url`

### 2. 模型名称
每个提供商的最新模型列表，例如：
- DeepSeek: `deepseek-chat`, `deepseek-reasoner`, 等
- GLM: `glm-4-plus`, `glm-4-flash`, `glm-4-air`, 等
- Kimi: `moonshot-v1-8k`, `moonshot-v1-32k`, `moonshot-v1-128k`, 等
- MiniMax: `abab6.5s-chat`, `abab6.5g-chat`, 等

### 3. 模型能力
- **上下文窗口大小** (`context_window`): 以 tokens 为单位
- **是否支持并行工具调用** (`supports_parallel_tool_calls`): true/false
- **是否支持图像输入** (`input_modalities`): ["text"] 或 ["text", "image"]
- **是否支持推理模式** (`supported_reasoning_levels`): 配置推理级别

### 4. 特殊功能
- 是否支持 function calling
- 是否支持流式输出
- 是否支持 WebSocket（目前已知都不支持）

## 快速验证配置

创建测试配置文件 `test-config.toml`:

```toml
[model_providers.test-provider]
name = "Test Provider"
base_url = "https://api.example.com"
env_key = "TEST_API_KEY"
wire_api = "responses"

model_provider = "test-provider"
model = "test-model"
```

然后测试：
```bash
export TEST_API_KEY="your-key"
codex --config-file test-config.toml
```

## 获取最新信息的步骤

1. **访问官方文档**
   - 查找 API 端点 URL
   - 查找可用的模型列表
   - 查找模型的上下文窗口大小

2. **测试 API**
   ```bash
   # 示例：测试 DeepSeek API
   curl https://api.deepseek.com/v1/models \
     -H "Authorization: Bearer $DEEPSEEK_API_KEY"
   ```

3. **更新配置**
   - 在 `config.toml` 中更新 `base_url` 和 `model`
   - 如需要，更新 `chinese_models.json` 中的能力配置

4. **验证**
   ```bash
   codex
   # 尝试简单的对话，确认模型工作正常
   ```

## 常见问题

### Q: 如何知道模型是否支持并行工具调用？
A: 查看官方文档中关于 function calling 的说明，或通过实际测试验证。

### Q: 如何知道上下文窗口大小？
A: 通常在官方文档的模型列表中会明确说明，例如 "128K context" 表示 128000 tokens。

### Q: 新模型不工作怎么办？
A: 
1. 检查 API key 是否正确
2. 检查模型名称是否正确（区分大小写）
3. 检查 API 端点是否正确
4. 查看错误日志获取详细信息

## 贡献更新

如果你获取了最新的模型信息，欢迎：
1. 更新 `chinese_models.json`
2. 更新文档
3. 提交 Pull Request

或者在 GitHub Issues 中分享最新信息。
