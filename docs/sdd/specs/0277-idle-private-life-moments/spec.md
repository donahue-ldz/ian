# 0277 · Idle Private Life Moments

## 问题 / 目标

Ian 需要偶尔表现出“它自己在生活”，而不是只对用户输入做反应。本 SDD 增加低频 Idle Private Life Moments，例如偷偷走两步、张望、整理自己、装乖。

## 当前产品阶段

v0.1.x Life Feel / Private Life。

## 产品范围

- 增加几类低频 idle private life story。
- 由 boredom、curiosity、energy 等 Life Drive 影响选择。
- 行为短、安静、可打断。
- DND、quiet、reduced motion、用户输入中必须抑制或降级。

## 明确不做

- 不做提醒、通知或任务建议。
- 不读取用户工作内容。
- 不长时间占用屏幕。
- 不做复杂作息系统。

## 用户体验

用户偶尔会看到 Ian 自己小小动一下：走两步又回来、偷偷看一眼、伸懒腰、装作没事。这些小瞬间让它像桌面上的生命，而不是工具。

## 架构约束

- Idle Moment 由 Rust Core 决定。
- React 不随机播放私生活动作。
- 所有行为必须通过 Moment 冷却、全局预算、用户控制。

## 数据 / 协议变化

优先复用 Story、Motion Profile、animation、speech、effect action。

## 隐私与安全边界

只使用本地时间、Ian 状态、Life Drive 和用户配置，不读取外部内容。

## 验收标准

- [ ] 至少定义 3 类 idle private life story。
- [ ] 正式模式下每类 story 有冷却和全局预算。
- [ ] 用户交互、设置打开、DND、quiet、reduced motion 会抑制或降级。
- [ ] 单个 story 短于 5 秒或可被立即打断。
- [ ] 真实桌面 long-run 观察中出现频率低且不打扰。

## 验证方式

- Rust Story 选择、冷却、预算测试。
- 前端执行测试。
- 真实 Tauri 桌面诊断触发和 long-run 观察。
