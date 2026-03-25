# Todo

- [x] 建立上游基线：克隆 `openai/codex` 到父目录并确认对比基准。
- [x] 盘点当前分支相对 upstream 的全部改动，并按模块分类。
- [x] 核对 DeepSeek、MiniMax、GLM、Kimi 截至 2026-03-25 的官方模型信息与 API 兼容差异。
- [x] 修正模型映射、默认值、供应商兼容层和潜在 bug。
- [x] 更新必要文档与说明，确保适配路径清晰。
- [x] 运行格式化、lint 和针对性测试，记录验证结论。
- [x] 在 `README.md` 和 `README_CN_ADAPTER.md` 补充中文说明，写清改进点、兼容原理、使用方式和限制。
- [x] 将当前分支推送覆盖到用户 fork：`keepkeen/codex`。

# Review

- 已将四家中国 provider 的内置接入从错误的 Responses API 假设改成 `chat/completions` 兼容层，并新增 `codex-api` 的 chat endpoint / request builder / SSE 解析。
- 修复了生产路径中的关键问题：`ThreadManager` 现在会按当前选中的 provider 初始化 `ModelsManager`，不再固定用 OpenAI 模型目录。
- DeepSeek 内置模型目录改为官方别名 `deepseek-chat` / `deepseek-reasoner`；保留 `deepseek-chat-thinking` 和 `deepseek-thinking` 作为兼容别名，并在模型元数据层同步映射到 `deepseek-reasoner`，避免旧配置落回 fallback metadata。
- Kimi 内置目录改为 `kimi-latest` 并标记图像输入能力；环境变量以 `MOONSHOT_API_KEY` 为准，同时兼容旧的 `KIMI_API_KEY`。
- MiniMax 内置目录改为 `MiniMax-M2.7` / `MiniMax-M2.7-highspeed`，并在 chat 请求里启用 `reasoning_split=true`。
- 删除了已经失效的 `codex-rs/core/chinese_models.json` 和 6 份重复/过期中文文档，只保留一份准确的说明文档和精简版 `README_CN_ADAPTER.md`。
- 已执行：
  - `cargo test -p codex-api`：chat 相关新增测试通过；完整 crate 中 5 个 realtime websocket 测试因沙箱禁止本地 `bind` 失败，与本次修改无关。
  - `cargo test -p codex-core --lib`：完整库测试中存在大量既有沙箱相关失败（wiremock 绑定端口、js_repl sandbox-exec、系统配置 API），不适合作为本次改动的结果判定。
  - 直接相关子集测试通过：`cargo test -p codex-api chat:: --lib`、`cargo test -p codex-core deepseek_provider_uses_provider_specific_bundled_catalog --lib`、`cargo test -p codex-core kimi_provider_marks_built_in_model_as_multimodal --lib`、`cargo test -p codex-core new_seeds_models_manager_from_selected_provider --lib`、`cargo test -p codex-core model_providers_reject_reserved_built_in_ids --lib`、`cargo test -p codex-core test_deserialize_chat_wire_api --lib`。
- 已执行 `just fix -p codex-api`、`just fix -p codex-core`、`just fmt`。
- `just argument-comment-lint` 无法执行：仓库当前缺少 `./tools/argument-comment-lint/run-prebuilt-linter.sh`。
- 本轮补充验证：
  - `cargo test -p codex-core deepseek_legacy_thinking_alias_uses_reasoner_metadata --lib` 通过。
  - `cargo test -p codex-core models_manager::manager::tests:: --lib` 中与本次改动直接相关的 10 个测试通过；其余 6 个失败仍然是 `wiremock` 在沙箱下无法绑定本地端口。
