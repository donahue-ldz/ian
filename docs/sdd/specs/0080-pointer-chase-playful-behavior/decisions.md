# 0080 决策记录

## 2026-05-21

- 采用 Tauri `cursorPosition()` 读取当前桌面鼠标坐标，不新增 Rust command。
- 坐标事件只包含 `x/y/now_ms`，不携带窗口、应用、点击对象或轨迹。
- 追逐不是实时跟随，而是低频候选事件 + Rust Core 决策 + 冷却。
- 追逐动作复用现有 `movement.move_to`、`animation.play`、`effect.play` 和 `playful.state`。

