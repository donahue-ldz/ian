# 0276 · 决策记录

## 2026-05-22

- 放下安顿只做短收尾，不做主动重排桌面位置。
- 最终坐标准确性优先于视觉回弹。

## 2026-05-25

- Drop settle story 由 Rust Core 决定，包含 diagnostic、settle movement、轻反馈、短气泡和 idle return。
- 连续 drop 命中 cooldown 时只保留必要位置回写和 idle，不叠加 blush / speech / settle story。
- Life Drive 只影响低敏表现参数，例如 settle 反馈时长和强度，不改变持久化位置格式。
