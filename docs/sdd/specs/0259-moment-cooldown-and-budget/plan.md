# 0259 · 实现计划

## 实现步骤

1. 盘点所有 moment 触发点。
2. 定义 budget key、cooldown key 和优先级。
3. 写预算耗尽、冷却、降级测试。
4. 接入 Moment Orchestrator。
5. 记录诊断但不展示敏感内容。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/protocol/state.rs`
- `apps/desktop/src/state/ianActions.ts`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck
```

## 风险和回滚

- 风险：预算过严导致没有惊喜。通过配置常量调节。
- 回滚：禁用新预算，恢复单类冷却。
