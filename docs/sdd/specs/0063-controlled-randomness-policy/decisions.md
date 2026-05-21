# Decisions: Controlled Randomness Policy

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 随机必须可复现、可测试，并由 Rust Core 决策。 | Ian 需要生命感，但 SDD 验收不能被不可控随机破坏。 |
| 2026-05-21 | 先实现轻量 `ControlledRandom` helper，而不是全局随机框架。 | 当前只需要路径扰动和候选选择的可复现性，过早抽象会拖慢 P0。 |
