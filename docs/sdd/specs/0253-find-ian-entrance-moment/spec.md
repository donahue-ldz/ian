# 0253 · Find Ian Entrance Moment

## 问题 / 目标

当前找回 Ian 容易变成机械移动。这个 SDD 将找回设计成“入场小瞬间”：Ian 被叫到时短暂出现、回应、定位，而不是只瞬移。

## 当前产品阶段

v0.1.x / Desktop Creature Usability。

## 产品范围

- 找回 Ian 时播放短入场 moment。
- 越界或不可见时移动到安全区。
- 已可见时只轻回应，不大幅移动。
- 复用现有 `find_ian` 事件、移动、气泡、动画和 effect。

## 明确不做

- 不改变全局快捷键隐私边界。
- 不做复杂路径动画编辑器。
- 不播放声音。

## 用户体验

用户按快捷键或设置入口找回 Ian 后，Ian 从边缘或安全区小跑/探头出现，短暂亮一下，说一句“我在这儿。”。连续触发时动作降级，避免烦。

## 架构约束

- `find_ian` 事件进入 Rust Core，由 Moment Orchestrator 决定动作序列。
- React 只执行移动、动画、气泡和 effect。
- 必须尊重 reduced motion 和勿扰。

## 数据 / 协议变化

优先复用 `movement.move_to`、`animation.play`、`speech.show`、`effect.play`。如需要新增 window show / focus action，必须记录兼容策略。

## 隐私与安全边界

不采集键盘输入内容；快捷键只产生低敏 `find_ian` 动作事件。

## 验收标准

- [ ] 找回 Ian 触发入场 moment。
- [ ] Ian 越界时回到可见安全区域。
- [ ] Ian 已可见时不突兀瞬移。
- [ ] 连续触发受冷却控制。
- [ ] 真实 Tauri 桌面验收覆盖快捷键和设置入口。

## 验证方式

- Rust find_ian moment 测试。
- 前端动作执行测试。
- 真实 Tauri 桌面端快捷键 / 设置入口验收。
