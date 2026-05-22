# 0271 · 决策记录

## 2026-05-22

- 惊喜感优先来自短故事编排，而不是更多随机动作。
- Story 必须短、可打断、可降级。
- 本阶段先把 `MomentStory` 作为 Rust Core 内部编排结构落地，输出仍然是既有 `IanAction` 序列，不新增前端故事解释器。
- Pointer Curiosity 保留既有 `happy` 反馈，避免微故事替换后破坏用户已经感知到的靠近反馈。
- reduced motion 先跳过 playful 位移动作，保留低扰动动画和文本反馈。
