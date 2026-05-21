# Decisions: Playful Safety And User Control

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 关闭和降低高能通过 `playful_energy` 完成，暂停通过 `playful_snoozed_until_ms` 进入 state/config。 | 用同一个本地控制面覆盖用户偏好和临时安静，不新增外部权限。 |
| 2026-05-21 | quiet mode、quiet hours、拖拽和气泡输入优先于 spontaneous zoomies。 | 用户控制和低打扰必须比“可爱惊喜”优先。 |
| 2026-05-21 | 高能能力必须先有关闭、暂停和取消路径。 | 满屏乱跑只有在用户可控时才是可爱，不可控时就是打扰。 |
