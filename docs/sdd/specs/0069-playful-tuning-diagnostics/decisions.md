# Decisions: Playful Tuning Diagnostics

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | playful diagnostics 只包含 `timestamp_ms`、`reason`、`result`、`cooldown_key`、`chosen_reaction_key`。 | 这些字段足够解释触发/拒绝原因，并且不包含窗口标题、URL、路径、代码或输入文本。 |
| 2026-05-21 | 诊断通过 `playful.diagnostic` action 进入 state/runtime，不做普通用户主界面。 | 支持本地调参和验收，同时避免把 Ian 做成调试面板。 |
| 2026-05-21 | 高能随机行为必须配套本地低敏诊断。 | 只有可解释，后续才能把随机和卖萌调到稳定好用。 |
