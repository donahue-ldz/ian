# 0275 · 决策记录

## 2026-05-22

- 拖动手感优先于装饰效果，不能牺牲命中和位置准确。
- 拖动中必须压制其他 Moment。

## 2026-05-25

- 前端只增加 `data-drag-phase` 表现层状态：`resting`、`pickup`、`carried`、`dropping`；真实拖动坐标和持久化逻辑不变。
- Rust Core 在 drag start 输出 carry diagnostic 和短暂 `PlayfulState::WarmingUp`，用于表达被抱起状态并压制其他 Moment。
- 不引入复杂物理或额外协议类型，避免影响拖动命中和最终落点。
