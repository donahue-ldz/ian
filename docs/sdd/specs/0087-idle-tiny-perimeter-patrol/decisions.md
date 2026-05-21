# Decisions: Idle Tiny Perimeter Patrol

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 使用临时 `appearance.scale_to`，不写入 `surface_scale`。 | 用户设置不应被自动行为覆盖。 | 巡游结束可恢复正常显示大小。 |
| 2026-05-21 | React 只报告 screen bounds，路径由 Rust Core 决定。 | 保持 Rust Core 作为行为大脑。 | 前端不生成巡游路线。 |
| 2026-05-21 | 初始空闲阈值使用 60 秒。 | 用户已接受推荐值，便于验证。 | 60 秒无互动后可进入巡游。 |
| 2026-05-21 | 巡游进入后一次输出完整边缘路径，并让桌面移动时长按距离放大。 | 实机反馈只从左下往上快速移动；根因是 Rust 只发一个角点、前端远距离移动仍用固定 1.8s。 | 进入巡游后会依次走左上、右上、右下、左下、左上；长边移动明显变慢。 |

## Scope Changes

None.

## Deferred Work

- Tauri dev 实机 smoke：录制或观察缩小巡游的真实桌面表现。
- 后续可将空闲阈值和巡游大小做成设置。
