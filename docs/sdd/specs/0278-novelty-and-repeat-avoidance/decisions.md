# 0278 · 决策记录

## 2026-05-22

- Novelty 只做短期枚举去重，不做用户画像。
- 诊断触发优先可复现，不受去重策略限制。

## 2026-05-25

- Novelty History 只保存在内存中，记录低敏 ID：Moment kind、story variant、phrase ID，不持久化，不记录原始文本。
- 候选不足时允许 fallback 到最近项，保证 Ian 不会因为去重而完全失去行为。
- 本轮先把 Novelty 接入 private life Moment；Demo dialogue 原有重复回复测试保持通过，后续若扩展通用 phrase policy 再单独拆 SDD。
