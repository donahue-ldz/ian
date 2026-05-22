# 0242 · 决策记录

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-22 | 新增 v0.1 baseline readiness map v2。 | v0.1 skeleton 多，需要区分 ready、skeleton、default-off 和 not product-ready。 | 防止把架构存在误解为产品启用。 |
| 2026-05-22 | 创建本 SDD，纳入 0232-0251 post-0231 readiness / hardening 批次。 | 用户明确要求继续完成接下来 20 个 SDD，仓库没有现成 0232-0251 目录。 | 本批次由 agent 生成并执行，范围限定在 P0 生命感、v0.1 骨架和默认关闭边界。 |

## Deferred Work

- 不在本 SDD 中启用未来阶段产品能力。
- 发现超出范围的需求时，拆到后续 SDD。
