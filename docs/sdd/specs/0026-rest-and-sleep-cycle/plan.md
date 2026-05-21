# 实现计划: Rest And Sleep Cycle

## 对应规格

`docs/sdd/specs/0026-rest-and-sleep-cycle/spec.md`

## 实现步骤

1. 为 sleep/rest tick 和 wake interaction 写 Rust 失败测试。
2. 在 scheduler 或 behavior policy 中增加 rest 候选和 cooldown。
3. 在 BehaviorEngine 中处理 sleep 状态下的 click / near 唤醒。
4. 前端确认 sleep / idle / happy 切换执行路径。
5. 补充 smoke 并记录验证结果。

## 预计改动文件

- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/state/ianActions.test.ts`
- `docs/sdd/specs/0026-rest-and-sleep-cycle/*`

## 接口 / 兼容性

尽量复用现有 behavior / animation 字段。如需新增 `resting`，必须同步 Rust 和 TypeScript 协议。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml scheduler
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
```

## 风险和回滚

sleep 频率过高会显得迟钝。回滚方式是只在 quiet / normal 的长间隔 tick 中启用。
