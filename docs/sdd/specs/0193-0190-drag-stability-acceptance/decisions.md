# 0193 · 决策记录

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-22 | 创建本 SDD，按真实缺口推进：0190 拖动稳定性验收收口。 | 用户要求基于已实现 SDD 和产品设计仔细拆分，不按编号猜测。 | 后续实现必须先读本 spec 和 plan，避免重复做已完成 skeleton。 |
| 2026-05-22 | 本轮不重写拖动实现，只用现有 desktop drag 链路和统一 smoke 清单收口。 | 代码已通过 `startDesktopWindowDrag`、`getDesktopWindowPosition` 和 `resolveSavedDragPosition` 表达 0190 设计，风险集中在人工多屏验收。 | verification 记录自动测试和真实 Tauri 启动；三屏手动拖动仍是剩余风险。 |

## Deferred Work

- 实现过程中发现超出本 SDD 的产品能力时，必须拆到后续 SDD。
- 不得在本 SDD 中顺手启用社交、插件、读屏、读代码或全局键盘采集能力。
