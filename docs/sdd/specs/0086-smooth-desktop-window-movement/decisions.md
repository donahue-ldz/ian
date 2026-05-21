# Decisions: Smooth Desktop Window Movement

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 在前端桌面执行层做插值，不改 Rust Core 路径策略。 | 用户反馈的是桌面移动观感过快，不是行为目标不对。 | 保持 Rust Core 作为行为大脑，React 只执行动作。 |
| 2026-05-21 | 保留 speed 语义差异。 | run-around、自动游走、zoomies 仍需要不同节奏。 | `fast` 不再瞬移，但仍快于 `normal` 和 `slow`。 |

## Scope Changes

None.

## Deferred Work

- 后续可增加 Tauri dev 实机 smoke，录制或记录窗口坐标随时间变化。
