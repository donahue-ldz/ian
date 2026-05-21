# 实现计划: Simple Mood State

## Spec

`docs/sdd/specs/0010-simple-mood-state/spec.md`

## 状态

已实现，已验证。

## 概要

实现最小 MoodEngine，让 Ian 有轻微情绪连续性，但不做完整模型或用户可见数值。

## 步骤

1. 写 MoodEngine 单元测试。
2. 定义 `MoodState` / `MoodSignal` 的简单状态机。
3. 将 mood context 接入 DialogueEngine。
4. 在 BehaviorPolicy 中使用 mood 影响候选动作。
5. 同步协议或保持内部状态，并验证边界。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/mood/*`
- `apps/desktop/src-tauri/src/domain/dialogue/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/protocol/generated.ts`（如暴露 mood）

## 接口与边界

MoodEngine 只输出状态，不直接产生 UI action；具体行为仍由 BehaviorEngine / DialoguePolicy 决定。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml mood
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
npm run desktop:typecheck
```

## 风险

- 过早复杂化会拖慢迭代；状态枚举必须少且行为可解释。

## 回滚说明

可将 MoodEngine 固定返回 calm，保留接口。
