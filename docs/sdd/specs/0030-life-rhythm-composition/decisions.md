# 决策记录: Life Rhythm Composition

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 0030 只做组合收束，不新增外部能力。 | 前 9 个 SDD 已覆盖核心生命感切片，需要防止行为互相抢占。 | 提升稳定性，降低后续迭代漂移。 |
| 2026-05-21 | Life Rhythm 优先级先收敛在 `BehaviorEngine` 内部 helper，而不新增过早抽象。 | 当前规则仍小，独立 policy 文件会增加跳转成本。 | 统一执行顺序：用户 active 交互阻止自主 tick；click/double-click 直接处理；run 阻止自主 tick；sleep 胜过 roam；roam 只在最低优先级追加。 |
