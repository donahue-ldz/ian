# Spec: Resource Pack Contract

## 状态

已实现，已验证。

## 问题 / 目标

0002 已让 Ian 从 Resource Pack 渲染，但资源包契约仍缺少校验、fallback 策略和版本说明。0008 目标是把默认资源包从“能加载”提升为“可扩展契约”，为后续更多表情、动作和音效做准备。

## 当前产品阶段

v0.1.x Product Iteration + Architecture Baseline hardening。

## 产品范围

- 校验 `pet.json`、`animations.json`、`expressions.json` 的最小 schema。
- 缺失动画或资源时 fallback 到 idle/default。
- 增加资源包版本与兼容性检查。
- 可增加少量非侵入式表情映射。

## 明确不做什么

- 不做资源包导入 UI。
- 不做资源市场。
- 不播放声音。
- 不引入 PixiJS/WebGL。

## 用户体验

Ian 的默认资源包加载更稳定；资源缺失时不会白屏或崩溃，而是回到默认 idle。

## 架构约束

- Resource Pack 描述视觉资源，React 只按 manifest 渲染。
- Rust 可保留 registry skeleton，但不直接操控前端动画帧。
- 校验错误必须可记录、可 fallback。

## 数据 / 协议变化

可新增前端资源 manifest 类型和 Rust registry 校验类型；不改变 `IanEvent` / `IanAction` 主协议。

## 隐私与安全边界

只加载 bundled resource，不读取任意用户文件。

## 验收标准

- [ ] Resource Pack loader 对缺失 manifest 字段返回可处理错误。
- [ ] 缺失请求动画时 fallback 到 idle。
- [ ] 资源包版本字段被读取并记录。
- [ ] 默认 `ian-alpaca` 通过校验。
- [ ] 测试覆盖正常加载、缺失动画、资源缺失 fallback。

## 验证方式

- 前端 resource loader 单元测试。
- Rust resource registry 测试。
- Browser smoke 确认默认资源仍显示。
