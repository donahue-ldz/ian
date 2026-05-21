# Spec: Animation Expression Upgrade

## 状态

已实现，待验收。

## 问题 / 目标

Ian 已有基础动画，但生命感还依赖动作过渡、表情差异和情绪反馈。0025 目标是在 Resource Pack 约束内增强动画与表情表现，为移动、休息、亲近互动提供更自然的视觉基础。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 增强 idle、walk、run、happy、sleep 的表现或 fallback。
- 增加 transition / expression 的资源契约。
- Mood / Bond 可影响表情选择，但不展示数值。
- 资源缺失时有稳定 fallback。

## 明确不做什么

- 不做最终美术资产。
- 不引入复杂动画引擎。
- 不把表情逻辑写死在 React 分支里。
- 不做音效系统。

## 用户体验

Ian 的跑动、停留、开心、休息能看出差异，动作之间不会显得突兀。

## 架构约束

- Resource Pack 是视觉来源。
- Rust Core 输出语义动作或状态，React 根据资源包播放。
- React 不根据 Mood / Bond 自行决定长期行为。

## 数据 / 协议变化

可扩展 `animations.json` / `expressions.json` schema。协议层优先保持语义动画名，不暴露具体帧实现。

## 隐私与安全边界

纯本地资源加载，不读取外部内容，不联网。

## 验收标准

- [ ] Resource Pack schema 支持 expression 或 transition 描述。
- [ ] 资源缺失时 fallback 稳定。
- [ ] idle / walk / run / happy / sleep 至少有可区分表现。
- [ ] Mood 或 Bond context 可影响表情选择边界。
- [ ] 前端测试覆盖资源解析和 fallback。

## 验证方式

- Resource loader tests。
- Animation player tests。
- Browser smoke：切换主要动画无空白帧、无 console error。
