# Decisions: Playful Visual Effects

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 新增 `effect.play` action，由 Rust Core 发出视觉效果意图，React 只执行。 | 保持行为决策在 Core，前端不根据随机或状态自行决定高能表现。 |
| 2026-05-21 | zoomies 使用 `speed_lines`，亲近反馈使用 `heart_pop`、`sparkle_pop`、`blush_puff` 三类轻效果。 | 可明显区分高能跑动和普通互动，又不会遮挡气泡或设置入口。 |
| 2026-05-21 | 高能视觉效果必须受 reduced motion 和 quiet mode 控制。 | 可爱表现不能牺牲长期桌面舒适度和用户控制。 |
