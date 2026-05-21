# 实现计划: Dialogue Provider 边界收敛

## Spec

`docs/sdd/specs/0004-dialogue-provider-boundary/spec.md`

## 状态

草稿，等待用户确认。

## 概要

收敛 Dialogue Provider、DialogueEngine、DialoguePolicy 的职责，让 Ian 的语言能力短句化、角色化，并为后续 BYOM 留边界。

## 步骤

1. 梳理当前 dialogue 代码路径。
2. 定义 provider trait 或等价边界。
3. 让 Demo Dialogue 通过 provider boundary 返回候选回复。
4. 强化 DialoguePolicy 的长度裁剪和禁用话术过滤。
5. 确保 `DialogueUserMessage` 与 click phrase 都走 Rust Core。
6. 补充测试。
7. 更新 verification。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/dialogue/*`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/state/*.test.ts`
- `docs/sdd/specs/0004-dialogue-provider-boundary/verification.md`
- `docs/sdd/specs/0004-dialogue-provider-boundary/decisions.md`

## 验证命令

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

## 风险

- 如果把 provider 抽象做太重，会超过 P0/v0.1.x 需要。
- 如果回复规则太复杂，容易让 Ian 偏向聊天助手。

## 回滚说明

回退 dialogue domain 相关文件即可恢复 0001 行为。

