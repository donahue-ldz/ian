# 0277 · 决策记录

## 2026-05-22

- Ian 的私生活 Moment 必须低频，不变成干扰。
- 行为只来自本地低敏状态，不根据用户工作内容生成。

## 2026-05-25

- 0277 首批私生活 Moment 先复用现有 animation / movement / speech / effect action，不新增资源包动画，避免把本 SDD 扩成美术资源工作。
- 自动触发窗口按低频 tick 进入 Rust Core，由 Life Drive 调整候选顺序，再经过 Moment 冷却、全局预算和 Novelty 选择。
- 诊断触发保持直达，用于桌面验收，不受 Novelty 历史影响。
