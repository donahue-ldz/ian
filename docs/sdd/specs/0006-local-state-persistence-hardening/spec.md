# Spec: Local State Persistence Hardening

## 状态

已实现，已验证。

## 问题 / 目标

0001-0005 已形成桌面生命的基础闭环，但本地状态仍偏“能用”而不是“可长期演进”。0006 目标是收紧本地配置、窗口位置、基础互动事件和 schema migration，保证后续 Mood、Bond、Reminder 不需要重做存储边界。

## 当前产品阶段

P0 收口 / v0.1.x 准备。

## 产品范围

- Ian 重启后恢复窗口位置、当前资源包和基础行为配置。
- 记录最小互动事件，用于后续简单 Mood/Bond，不展示给用户。
- 提供清晰的默认配置和损坏配置恢复策略。

## 明确不做什么

- 不实现长期记忆。
- 不展示历史记录。
- 不把互动事件用于主动提醒或画像。
- 不做云同步。

## 用户体验

用户拖动 Ian 后，下一次启动应出现在上次位置附近。配置损坏时 Ian 应回到默认位置，而不是启动失败。

## 架构约束

- Rust Core / Storage 层拥有持久化逻辑。
- React 只发送窗口内拖拽结束事件和执行状态同步。
- SQLite migration 必须可重复运行。

## 数据 / 协议变化

- 可新增或收紧 `settings_kv`、`interaction_events`、`resource_packs` 的 repository 方法。
- 如需同步状态，仍通过 `IanAction::StateSync`。

## 隐私与安全边界

只写入本地配置和用户主动与 Ian 交互产生的轻量事件，不记录输入全文之外的敏感环境信息。

## 验收标准

- [ ] 拖动 Ian 后重启应用，窗口位置恢复到保存坐标。
- [ ] 缺失或损坏的 `config.toml` 会被安全恢复为默认配置。
- [ ] SQLite migration 可在空库和已有库上重复执行且不报错。
- [ ] 基础互动事件通过 repository 写入，不由 React 直接写存储。
- [ ] `npm run desktop:test`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage` 通过。

## 验证方式

- Rust storage repository 测试。
- 手动或 Playwright/Tauri smoke 验证拖动后重启位置恢复。
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
