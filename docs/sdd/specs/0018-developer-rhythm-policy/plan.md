# 实现计划: Developer Rhythm Policy

## Spec

`docs/sdd/specs/0018-developer-rhythm-policy/spec.md`

## 状态

已实现，待验收。

## 概要

为 v0.2 开发者事件增加统一策略层，让 Ian 的反应像生命节奏，而不是通知列表。

## 步骤

1. 定义 DeveloperRhythmPolicy 输入和输出。
2. 编写 cooldown、burst、连续失败测试。
3. 接入 BehaviorEngine。
4. 增加默认频率配置。
5. 做连续事件 smoke。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/core/*`
- `apps/desktop/src-tauri/src/protocol/event.rs`
- `docs/sdd/specs/0018-developer-rhythm-policy/*`

## 接口与边界

Policy 只消费已脱敏事件，不接触 adapter 原始数据。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml policy
npm run desktop:test
```

## 风险

- 反应过频会破坏陪伴感；默认策略应保守。

## 回滚说明

将 DeveloperRhythmPolicy 固定返回 no-op，保留 adapter 事件入口。
