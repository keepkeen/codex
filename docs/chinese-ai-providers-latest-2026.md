# 中国模型提供商 - 2026年3月最新信息

## 最新模型信息（通过 Tavily 搜索获取）

### DeepSeek (2026年3月)
- **deepseek-chat**: DeepSeek-V3.2 非思考模式
  - 上下文: **128K** tokens
  - 并行工具调用: ✅ 支持
  - 图像输入: ❌ 不支持
  - 价格: $0.28/1M input, $0.42/1M output

- **deepseek-reasoner**: DeepSeek-V3.2 思考模式
  - 上下文: **128K** tokens
  - 推理级别: low/medium/high
  - 并行工具调用: ✅ 支持
  - 价格: $0.55/1M input, $2.19/1M output

### GLM (智谱AI, 2026年)
- **glm-4-plus**: 最强多模态模型
  - 上下文: **128K** tokens
  - 并行工具调用: ✅ 支持
  - 图像输入: ✅ 支持
  - Function calling: ✅ 支持

- **glm-4-flash**: 快速响应模型
  - 上下文: **128K** tokens
  - 并行工具调用: ✅ 支持
  - 图像输入: ❌ 不支持

- **glm-4.6v**: 视觉模型 (106B)
  - 上下文: **128K** tokens
  - 多模态: ✅ 文本+图像+视频
  - 开源: ✅ HuggingFace/ModelScope

### Kimi (Moonshot AI, 2026年)
- **kimi-k2.5**: 最新旗舰模型
  - 上下文: **256K** tokens (业界领先)
  - 并行工具调用: ❌ 不支持
  - 图像输入: ✅ 支持
  - Agent swarm: ✅ 100个子代理
  - 价格: $0.60/1M input, $2.50/1M output

- **moonshot-v1-128k**: 旧版模型
  - 上下文: **128K** tokens
  - 并行工具调用: ❌ 不支持

### MiniMax (2026年3月)
- **MiniMax-M2.7**: 最新旗舰
  - 上下文: **204,800** tokens (~205K)
  - 并行工具调用: ✅ 支持
  - 递归自我改进: ✅ 支持
  - 输出速度: ~60 tps

- **MiniMax-M2.7-highspeed**: 高速版本
  - 上下文: **204,800** tokens
  - 输出速度: ~100 tps

- **MiniMax-M2.5**: 上一代旗舰
  - 上下文: **204,800** tokens
  - SWE-Bench: 80.2%

## 关键发现

### 上下文窗口对比
1. **Kimi K2.5**: 256K (最长)
2. **MiniMax M2.7**: 205K
3. **DeepSeek V3.2**: 128K
4. **GLM-4**: 128K

### 并行工具调用支持
- ✅ DeepSeek (chat & reasoner)
- ✅ GLM-4 (plus & flash)
- ❌ Kimi (所有版本)
- ✅ MiniMax (所有版本)

### 多模态支持
- DeepSeek: ❌ 纯文本
- GLM-4 Plus: ✅ 文本+图像
- GLM-4.6V: ✅ 文本+图像+视频
- Kimi K2.5: ✅ 文本+图像
- MiniMax: ❌ 纯文本

### 推理能力
- DeepSeek Reasoner: ✅ 支持 (low/medium/high)
- 其他: ❌ 不支持专门的推理模式

## 价格对比 (每百万 tokens)

| 模型 | 输入 | 输出 | 上下文 |
|------|------|------|--------|
| DeepSeek Chat | $0.28 | $0.42 | 128K |
| DeepSeek Reasoner | $0.55 | $2.19 | 128K |
| Kimi K2.5 | $0.60 | $2.50 | 256K |
| GPT-5.2 (对比) | $1.75 | $14.00 | 128K |

## 配置建议

### 通用编程任务
```toml
model_provider = "deepseek"
model = "deepseek-chat"  # 性价比最高
```

### 复杂推理任务
```toml
model_provider = "deepseek"
model = "deepseek-reasoner"
model_reasoning_effort = "high"
```

### 多模态任务（图像）
```toml
model_provider = "glm"
model = "glm-4-plus"  # 或 kimi-k2.5
```

### 超长上下文任务
```toml
model_provider = "kimi"
model = "kimi-k2.5"  # 256K context
```

### 高性能代码生成
```toml
model_provider = "minimax"
model = "minimax-m2.7"  # SWE-Bench 80.2%
```

## 注意事项

1. **Kimi 不支持并行工具调用** - 配置中已自动处理，会顺序执行工具
2. **所有提供商都不支持 WebSocket** - 自动回退到 HTTP SSE
3. **上下文窗口已更新** - DeepSeek 从 64K 升级到 128K
4. **Kimi K2.5 是新模型** - 比 moonshot-v1-128k 更强大
5. **MiniMax M2.7 是最新版本** - 比 abab6.5 系列更新

## 更新日期

本信息通过 Tavily MCP 搜索获取，更新于 **2026年3月25日**。
