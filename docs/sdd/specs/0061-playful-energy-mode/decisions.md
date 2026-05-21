# Decisions: Playful Energy Mode

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 高能卖萌独立成模式，不改写普通 idle roam。 | 0023 的低频游走已经验收，后续高能能力需要新增边界，避免历史规格冲突。 |
| 2026-05-21 | `playful_energy` 使用 `off`、`low`、`normal`、`high` 四档，并保存到本地 creature config。 | 用户需要直接可控的强度；旧配置缺失时可按 `normal` 退化。 |
