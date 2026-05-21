# Spec: Smooth Desktop Window Movement

## Status

Approved

## Context

用户验证桌面移动后反馈 Ian 现在移动很快。当前前端执行 `movement.move_to` 时一次调用 `moveDesktopWindow(target)`，桌面窗口会直接跳到目标点；`durationForSpeed()` 只影响多段动作之间的等待时间，不会让单段移动变慢。

## Goal

让桌面版 `movement.move_to` 以多帧插值方式缓慢移动窗口，避免视觉上瞬移。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline

## Product Scope

- 调整前端 action 执行层，让每个 `movement.move_to` 在桌面版分多帧执行。
- 保留 Rust Core 生成移动目标的职责。
- 保留已有 speed 差异：`fast` 最快，`normal` 居中，`slow` 最慢。

## Non-Goals

- 不改变 Rust Core 自动游走频率、路径或触发规则。
- 不新增用户设置。
- 不修改 `IanEvent` / `IanAction` / `IanState` 协议。
- 不实现屏幕感知或窗口感知策略。

## User Experience

Ian 在桌面版移动时会从当前位置平滑滑到目标位置。自动游走、双击 run-around、追鼠标和 zoomies 仍沿用各自的路径和速度语义，但不再单点跳跃。

## Architecture Constraints

- React 只执行 Rust Core 发出的 `IanAction::MovementMoveTo`。
- React 可以读取当前桌面窗口位置并分帧执行 Tauri `setPosition`，但不得生成新的行为目标。
- 浏览器预览继续使用现有 view state fallback，不依赖 Tauri API。

## Data and Protocol Changes

None.

## Privacy and Security

只读取当前 Ian 窗口位置并设置当前 Ian 窗口位置，不读取屏幕内容、其他窗口内容、键盘、剪贴板或应用数据。

## Acceptance Criteria

- [ ] 桌面 movement 执行计划包含多帧中间坐标，最后一帧等于目标位置。
- [ ] `fast`、`normal`、`slow` 的移动时长保持可区分，且 `slow > normal > fast`。
- [ ] `movement.move_to` 执行链路使用平滑移动 helper，而不是单次 `moveDesktopWindow(target)`。
- [ ] 相关前端测试和类型检查通过。

## Verification Approach

```bash
npm run desktop:test -- ianActions.test.ts
npm run desktop:typecheck
```

## Open Questions

- None.
