# 0223 · 决策记录

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-22 | 创建本 SDD，按真实缺口推进：小冒险家反应包 V2。 | 用户要求基于已实现 SDD 和产品设计仔细拆分，不按编号猜测。 | 后续实现必须先读本 spec 和 plan，避免重复做已完成 skeleton。 |
| 2026-05-22 | 为 `ian-adventurer` 补 `find`、`wake`、`wave`、`affection`、`tantrum` 语义动作映射。 | 小冒险家是当前像素资源包，需要覆盖更多桌面可见反应，但不引入新图片或视频资产。 | 复用现有 sprite frame，manifest capability 与 `animations.json` 保持一致。 |

## Deferred Work

- 实现过程中发现超出本 SDD 的产品能力时，必须拆到后续 SDD。
- 不得在本 SDD 中顺手启用社交、插件、读屏、读代码或全局键盘采集能力。
