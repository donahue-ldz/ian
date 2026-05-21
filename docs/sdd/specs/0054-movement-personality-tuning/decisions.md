# Decisions: Movement Personality Tuning

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 移动调优优先让 Ian 少打扰但持续有生命感。 | 桌面陪伴需要长期共处，过多位移会比静止更像干扰源。 |
| 2026-05-21 | 将微动作 cadence 和自主位移 cadence 分开：quiet 不自主位移，normal 约 135 秒一次，lively 约 75 秒一次。 | 让三档移动倾向可测试且可区分，同时保留不位移时的 walk/idle 微动作生命感。 |
| 2026-05-21 | Movement profile 暂放在 Rust `BehaviorPolicy`，不新增协议字段。 | 当前已有 `behavior_mode` 和 movement settings，0054 只需收敛默认调优，不需要扩大配置协议。 |
