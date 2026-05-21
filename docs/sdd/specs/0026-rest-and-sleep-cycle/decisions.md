# 决策记录: Rest And Sleep Cycle

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 休息循环只使用 Ian 自身状态和本地 tick。 | 避免把休息做成工作状态监控。 | 保持隐私边界和生命感方向。 |
| 2026-05-21 | sleep/rest 与 roam 共用 interaction-active 抢占保护。 | 休息不能在用户拖拽、输入或 run-around 期间抢占。 | `time.tick` 在 active interaction 期间返回空自主动作；sleep 候选高于 roam 候选。 |
