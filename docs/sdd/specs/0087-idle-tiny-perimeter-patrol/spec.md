# Spec: Idle Tiny Perimeter Patrol

## Status

Approved

## Context

用户希望在没有点击 Ian 时，Ian 可以自动缩小，并从屏幕左上角开始沿屏幕边缘慢慢转圈。当前自动游走只是在当前位置附近短路径移动，不知道真实屏幕边界，也不会临时改变视觉大小。

## Goal

当用户一段时间没有互动时，Ian 进入小型屏幕边缘巡游：临时缩小，沿当前桌面屏幕边缘慢速移动；用户互动后恢复正常大小和常规行为。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline

## Product Scope

- 前端桌面层提供当前主窗口所在屏幕的安全边界给 Rust Core。
- Rust Core 在空闲 tick 上输出屏幕边缘巡游路径。
- 进入巡游时输出临时缩小 action，退出或用户互动时恢复正常视觉大小。
- 使用现有 `movement.move_to` 执行链路和慢速平滑移动。

## Non-Goals

- 不实现复杂窗口感知、应用内容读取、OCR 或全局输入监听。
- 不把 `surface_scale` 用户设置改成持久的小尺寸。
- 不新增设置 UI。
- 不实现多显示器智能路径规划；本轮使用前端报告的当前屏幕 bounds。

## User Experience

用户不点击、不拖拽、不输入、不靠近 Ian 一段时间后，Ian 会缩小到很小，移动到屏幕左上附近，然后沿上边、右边、下边、左边缓慢巡游。用户点击、拖拽、输入、双击或靠近时，Ian 退出巡游并恢复正常大小。

## Architecture Constraints

- Rust Core 继续作为行为大脑，决定是否进入巡游和下一个巡游目标。
- React 只收集低敏屏幕 bounds、发送 `screen.bounds` 事件，并执行 `IanAction`。
- 缩小是临时 action，不写入持久 `surface_scale` 设置。
- 所有移动继续通过 `IanAction::MovementMoveTo`。

## Data and Protocol Changes

- 新增 `IanEvent::ScreenBounds`，携带当前屏幕安全区域。
- 新增 `IanAction::AppearanceScaleTo`，携带临时 scale 和持续时间。
- TypeScript checked-in protocol mirror 同步更新。

## Privacy and Security

屏幕 bounds 只包含位置和尺寸，不包含屏幕内容、窗口标题、应用名称、文本、剪贴板或键盘信息。

## Acceptance Criteria

- [ ] Rust Core 在收到有效 `screen.bounds` 后保存屏幕边界到当前状态。
- [ ] 空闲超过阈值且不在 quiet、拖拽、输入、run、night 等保护场景时，`time.tick` 可输出 `appearance.scale_to` 和屏幕边缘 `movement.move_to`。
- [ ] 巡游路径从左上附近开始，并沿屏幕边缘产生后续目标。
- [ ] 用户互动事件会恢复视觉 scale 到 `1.0`。
- [ ] React reducer 能执行 `appearance.scale_to`，且不会覆盖用户持久 `surface_scale` 设置。
- [ ] Rust 行为测试、前端 reducer 测试、类型检查通过。

## Verification Approach

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test -- ianActions.test.ts
npm run desktop:typecheck
```

## Open Questions

- None.
