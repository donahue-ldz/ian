# 实现计划: Behavior Scheduler

## Spec

`docs/sdd/specs/0009-behavior-scheduler/spec.md`

## 状态

已实现，已验证。

## 概要

在 Rust Core 内增加可控、可测试的轻量行为调度，使 Ian 具备低频自发动作。

## 步骤

1. 为 scheduler 编写 deterministic tests。
2. 实现 scheduler state 和 tick handling。
3. 接入行为模式配置。
4. 确保当前动作未结束时不抢占。
5. 补前端 smoke 验证。

## 预计文件改动

- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/state/*`
- `docs/sdd/specs/0009-behavior-scheduler/*`

## 接口与边界

Scheduler 属于 Rust Core，不进入 React 决策层。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml scheduler
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
```

## 风险

- 自发动作过频会打扰用户；默认频率必须保守，并受设置控制。

## 回滚说明

可禁用 scheduler tick 输出，保留 click/double-click 交互。
