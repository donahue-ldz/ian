# Decisions: 多显示器拖动稳定性

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-22 | 桌面端拖动统一使用 Tauri 原生 `startDragging()`，不再同时用前端 `setPosition()` 驱动窗口。 | 当前三屏混合 DPI 环境中，原生拖动和手动物理坐标移动会争用窗口位置，是已确认的跨屏拖动不稳定根因。 | 拖动路径更简单；React 不再负责跨显示器物理坐标换算。 |

## Scope Changes

None.

## Deferred Work

- 多显示器自动漫游和安全边界策略不在本 packet 中实现。
- 如果原生拖动仍无法覆盖某些 macOS 透明窗口边界场景，再单独设计基于 `onMoved` 或 Rust 侧诊断的后续 packet。
