# 0275 · Drag Body Language V2

## 问题 / 目标

拖动 Ian 时，如果只是窗口跟随鼠标，会缺少被抱起或被拎着的感觉。本 SDD 优化拖动中的身体语言，让 Ian 被拖时有轻微反应、延迟和姿态变化。

## 当前产品阶段

v0.1.x Life Feel / Drag Feel。

## 产品范围

- 优化拖动开始、拖动中、拖动结束前的身体语言。
- 增加轻微延迟跟随、倾斜、缩起、眨眼或短气泡。
- 拖动中抑制其他 Moment。
- 保持拖动命中和位置准确。

## 明确不做

- 不做复杂物理模拟。
- 不做跨应用拖放。
- 不读取桌面内容。
- 不让视觉延迟破坏最终窗口位置。

## 用户体验

用户拖起 Ian 时，它会像被抱起来一样有一点身体反应；拖动仍然可控，不飘、不乱跳、不难放。

## 架构约束

- Drag start / drag move / drag end 事件进入 Rust Core 或现有低敏事件路径。
- Core 决定 carry 状态和动作。
- React 负责 pointer capture、窗口移动和视觉姿态。

## 数据 / 协议变化

可复用 motion profile 和 carry state。如需新增 drag visual intent，必须由 Rust 类型定义。

## 隐私与安全边界

只处理 Ian 窗口内拖动事件和窗口位置，不记录其他窗口内容。

## 验收标准

- [ ] 拖动开始有可感知 carry 反馈。
- [ ] 拖动中 Ian 跟随稳定，不出现明显偏移或跳屏。
- [ ] 视觉延迟不会影响最终落点和持久化。
- [ ] 拖动中其他 idle / chase / surprise Moment 被抑制。
- [ ] 真实桌面拖动至少 5 次没有出现无法拖动或命中丢失。

## 验证方式

- 前端 pointer capture 和 position 测试。
- Rust interaction gate 测试。
- 真实 Tauri 桌面连续拖动验收。
