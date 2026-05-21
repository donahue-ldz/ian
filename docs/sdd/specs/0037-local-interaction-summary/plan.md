# 实现计划: Local Interaction Summary

## 对应规格

`docs/sdd/specs/0037-local-interaction-summary/spec.md`

## 实现步骤

1. 为 summary 聚合写失败测试，覆盖空日志和多事件。
2. 增加 interaction summary service。
3. 将 summary 接入 Bond 或 DialogueContext。
4. 增加保留期或压缩策略。
5. 做 source scan，确认不包含敏感字段。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/life_event_repository.rs`
- `apps/desktop/src-tauri/src/domain/bond/*`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `docs/sdd/specs/0037-local-interaction-summary/*`

## 接口 / 兼容性

如果 life event 不存在或读取失败，summary 使用默认空状态，不能阻塞对话或行为。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml bond
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
```

## 风险和回滚

摘要可能逐渐变成记忆系统。回滚方式是限制字段为计数和时间，不允许自由文本摘要。
