# 0271 · Micro Story Moment Framework

## 问题 / 目标

单个动作很难产生惊喜感。Ian 需要把 Moment 从“触发一个动作”升级为 3-5 秒的微型故事：发现、犹豫、行动、收尾。本 SDD 建立 Micro Story Moment Framework。

## 当前产品阶段

v0.1.x Life Feel / 生命感二阶段。

## 产品范围

- 定义 Moment Story 的 beat 结构。
- 支持发现、犹豫、行动、收尾四类 beat。
- 允许每个 beat 输出动画、移动、气泡、特效等已有 `IanAction`。
- 支持可打断、超时、冷却、reduced motion 降级。
- 为追鼠标、找回、拖动、idle 私生活 Moment 提供统一编排。

## 明确不做

- 不做复杂剧情系统。
- 不做长对话、任务、通知或插件流程。
- 不做跨天叙事和长期剧情分支。
- 不绕过现有 Moment 冷却和预算。

## 用户体验

Ian 的反应会更像小段落：先注意到，再犹豫一下，然后行动，最后收尾。用户会感觉它有反应节奏，而不是瞬间切动画。

## 架构约束

- Story 编排属于 Rust Core / Moment Orchestrator。
- React 只按 `IanAction` 执行 beat，不决定故事顺序。
- Story 必须可中断，用户拖动、点击、设置打开等高优先级交互可打断。

## 数据 / 协议变化

优先复用现有 `IanAction` 序列。如果需要新增 delay / sequence 元信息，必须由 Rust 协议定义并生成 TypeScript 类型。

## 隐私与安全边界

Story 只使用低敏 Ian 事件、Life Drive 和确认过的低敏 memory tag，不读取高敏来源。

## 验收标准

- [ ] Rust Core 能表达至少 4 个 beat 类型：discover、pause、act、settle。
- [ ] Story 输出为有序 `IanAction` 或等价可执行序列。
- [ ] 用户交互能打断 active story。
- [ ] reduced motion 下 story 会缩短或替换为低动效版本。
- [ ] 至少一个现有 Moment 使用 Story Framework 改写并通过测试。

## 验证方式

- Rust 单元测试覆盖 beat 顺序、打断、降级。
- 前端测试覆盖 action sequence 执行不会重叠失控。
- 真实 Tauri 桌面观察一个 story Moment。
