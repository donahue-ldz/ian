# 实现计划: Idle Micro Roaming

## 对应规格

`docs/sdd/specs/0023-idle-micro-roaming/spec.md`

## 实现步骤

1. 为 scheduler 写 deterministic roam 测试，覆盖 quiet / normal / lively。
2. 增加 roam cooldown 和抢占保护条件。
3. 让 `TimeTick` 在允许时输出小范围 `MovementMoveTo`。
4. 前端保持纯执行层，确认没有在 React 中生成 roam 目标。
5. 补充本地 smoke 记录。

## 预计改动文件

- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src/state/ianActions.test.ts`
- `docs/sdd/specs/0023-idle-micro-roaming/*`

## 接口 / 兼容性

不新增外部权限。内部 cooldown 不需要持久化，除非实现时发现重启后体验明显异常。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml scheduler
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
```

## 风险和回滚

频率过高会打扰用户。回滚方式是把 roam 候选频率调低或仅在 lively 模式启用。
