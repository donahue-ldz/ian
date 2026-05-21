# 决策记录: Idle Micro Roaming

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 自主游走只做低频微移动。 | 生命感要克制，不能打扰用户。 | 安静模式优先保护用户控制。 |
| 2026-05-21 | 自主 roam 必须服从 interaction-active 抢占保护。 | 验收发现 tick 只保护 run / quiet / night，未保护拖拽和气泡输入。 | 新增 `is_dragging` / `is_bubble_input_active` 状态；拖拽、输入、run-around 期间 `time.tick` 不输出 roam movement。 |
