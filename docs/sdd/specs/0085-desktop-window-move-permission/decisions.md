# Decisions: Desktop Window Move Permission

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 只补齐 `core:window:allow-set-position`，不调整游走策略。 | 用户指出重点是桌面真实能力缺口，不是重新设计自动游走。 | 最小化变更范围，避免影响已有行为调度。 |
| 2026-05-21 | 用 capability 配置测试覆盖桌面移动权限。 | 浏览器预览无法证明 Tauri 原生窗口 API 授权。 | 后续删除权限会被前端测试捕获。 |

## Scope Changes

None.

## Deferred Work

- 后续可增加 Tauri dev 实机 smoke，记录 `movement.move_to` 后原生窗口坐标变化。
