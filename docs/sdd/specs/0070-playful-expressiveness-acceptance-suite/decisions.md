# Decisions: Playful Expressiveness Acceptance Suite

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 高能卖萌能力必须单独验收“可爱”和“可控”两件事。 | 只证明会跑不够，Ian 还必须不烦、不越界、可关闭。 |
| 2026-05-21 | 0070 增加独立 checklist 和 Vitest acceptance test，检查清单结构与 smoke JSON。 | 验收套件本身需要可回归，避免只靠一次性人工描述。 |
| 2026-05-21 | Browser smoke 只记录浏览器可直接观察的设置、bubble 和轻效果；zoomies/safety 字段标注来自 Rust/TS 自动化验证。 | 自发 zoomies 受 tick 窗口和 Core 策略控制，不伪造成浏览器随机触发结果。 |
