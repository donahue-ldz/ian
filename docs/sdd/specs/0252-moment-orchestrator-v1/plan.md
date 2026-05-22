# 0252 · 实现计划

## 实现步骤

1. 盘点现有 `BehaviorEngine`、`IanAction`、`IanState` 和 playful / life rhythm 策略。
2. 增加最小 moment 数据结构和选择器测试。
3. 将找回、鼠标好奇、拖动放下等未来 moment 设计成可注册或可匹配的内部规则。
4. 增加冷却和互斥规则，避免与拖动、输入、设置、睡眠冲突。
5. 只输出已有 `IanAction`，必要协议变化先记录到 decisions。
6. 运行 Rust 测试、前端 typecheck，并更新 verification。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src/protocol/generated.ts`

## 兼容性说明

- 不改变现有点击、拖动、找回、游走的基础语义。
- Moment 只能增强已有动作编排，不应新增默认高频主动行为。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck
```

## 风险和回滚

- 风险：moment 过多导致 Ian 变吵。应通过冷却和预算限制。
- 回滚：关闭 Moment Orchestrator，恢复单事件直接行为策略。
