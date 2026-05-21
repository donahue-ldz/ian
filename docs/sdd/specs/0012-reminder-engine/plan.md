# 实现计划: Reminder Engine

## Spec

`docs/sdd/specs/0012-reminder-engine/spec.md`

## 状态

已实现，待验收。

## 概要

实现本地低频提醒能力，保持 Ian 的陪伴感和用户控制。

## 步骤

1. 写 ReminderPolicy 冷却和开关测试。
2. 实现 ReminderEngine。
3. 接入 config/settings。
4. 将 reminder 输出为现有 `IanAction`。
5. 增加 Playwright smoke。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/reminder/*`
- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`

## 接口与边界

Reminder 不接外部 adapter；只通过 time tick 和本地配置运行。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml reminder
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml config
npm run desktop:test
```

## 风险

- 提醒容易变成生产力工具；默认文案、频率和关闭入口必须克制。

## 回滚说明

关闭 reminder config 或让 ReminderEngine 返回 no-op。
