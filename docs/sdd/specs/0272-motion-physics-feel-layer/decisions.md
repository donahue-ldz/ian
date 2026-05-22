# 0272 · 决策记录

## 2026-05-22

- 选择轻量 motion profile，不引入物理引擎。
- 最终位置仍以现有 position / safe zone 为准，motion 只改善过渡感。
- `MotionProfile` 由 Rust 协议定义并导出 TypeScript binding，前端只按 profile 执行表现，不决定行为意图。
- `playful` 使用轻微弧线和更活泼的 CSS body feel；`settle` 使用落地缓冲；`gentle` 保持默认稳定过渡。
