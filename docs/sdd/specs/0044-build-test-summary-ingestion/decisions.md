# 决策记录: Build Test Summary Ingestion

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | build/test 只接受结构化摘要，不读取终端全文。 | 保护隐私并避免 Ian 变成日志阅读器。 | 外部工具需要主动提供低敏摘要。 |
| 2026-05-21 | 新增 Tauri command 作为 ingestion 入口。 | 入口必须仍然转换为 `IanEvent`，不能直接控制动画。 | `ingest_build_test_summary` 复用 Security Gate 和 Runtime 流程。 |
