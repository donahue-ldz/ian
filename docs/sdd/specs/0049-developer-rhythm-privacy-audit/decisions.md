# 决策记录: Developer Rhythm Privacy Audit

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | Developer Rhythm 必须有可重复隐私审计。 | v0.2 触达开发者上下文，信任必须靠验证。 | 每次验收都需要 source scan 和 sanitizer 测试。 |
| 2026-05-21 | 敏感字段拒绝放在事件反序列化和 sanitizer 双层边界。 | Tauri 入参可能携带额外字段，typed event 进入 Runtime 前就应拒绝。 | 新增 `IanEvent` wire 反序列化测试，拒绝 `stdout`、`window_title`、`key` 等字段。 |
