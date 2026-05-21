# Decisions: Zoomies Path Behavior

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 新增 `behavior.zoomies` action，不复用双击 `behavior.run_around`。 | zoomies 需要更长路径、独立 reason、视觉效果和冷却，复用会混淆 P0 双击语义。 |
| 2026-05-21 | zoomies 生成 5 个快速 waypoint 后回 home anchor，全部经过 movement boundary。 | 满足“满屏乱跑”的可见范围，同时保持几秒内收住且不越界。 |
| 2026-05-21 | “满屏乱跑”定义为短时可控 zoomies，不作为普通 idle 行为。 | 用户想要高能表现，但 Ian 必须长期可共处、可关闭、可预测。 |
