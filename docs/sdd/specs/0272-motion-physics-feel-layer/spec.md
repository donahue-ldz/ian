# 0272 · Motion Physics Feel Layer

## 问题 / 目标

Ian 的移动如果像 UI 元素直线平移，会削弱生命感。本 SDD 增加轻量 Motion Feel Layer，让移动具备起步、惯性、缓停、回弹、朝向等身体感。

## 当前产品阶段

v0.1.x Life Feel / Motion Feel。

## 产品范围

- 为移动动作增加语义化 motion profile。
- 支持起步前蓄力、移动中倾斜、停下回弹、朝向变化。
- 支持被拖动时延迟跟随和放下缓冲。
- reduced motion 下自动降级。
- 不改变 Rust Core 作为行为大脑的边界。

## 明确不做

- 不做真实物理引擎。
- 不新增大型动画库。
- 不做复杂碰撞、重力、地图或游戏玩法。
- 不让 React 自己决定行为时机。

## 用户体验

Ian 移动时更像有身体：起步前轻微准备，跑动有方向感，停下不会僵硬，被拖动和放下有一点重量。

## 架构约束

- Rust Core 决定 movement intent 和 motion profile。
- React 负责将 profile 渲染为 CSS / animation 执行。
- Motion Feel 必须遵守 reduced motion 和用户控制。

## 数据 / 协议变化

可能新增 `movement_profile`、`motion_intent` 或等价字段。Rust 类型为协议源头。

## 隐私与安全边界

不涉及外部输入和隐私数据。

## 验收标准

- [ ] 移动 action 能携带语义化 motion profile。
- [ ] React 根据 profile 执行至少 3 类移动手感：gentle、playful、settle。
- [ ] reduced motion 下移动幅度、倾斜、回弹降低。
- [ ] 快速连续移动不会造成动画队列堆积。
- [ ] 真实桌面观察 Ian 起步、移动、停下比直线平移更自然。

## 验证方式

- Rust 协议和策略测试。
- 前端 action executor / CSS class 测试。
- 真实 Tauri 桌面移动手感验收。
