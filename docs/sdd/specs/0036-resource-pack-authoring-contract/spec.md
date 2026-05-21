# Spec: Resource Pack Authoring Contract

## 状态

已实现，待验收。

## 问题 / 目标

随着动画、表情和生命动作增加，资源包需要更稳定的作者契约。0036 目标是为默认资源包和未来资源包作者提供 schema、校验、示例和错误提示，避免每次新增动作都改渲染逻辑。

## 当前产品阶段

v0.1.x Resource。

## 产品范围

- 固化 `pet.json`、`animations.json`、`expressions.json` 的作者约束。
- 增加本地校验命令或测试工具。
- 提供最小示例和错误信息。
- Resource Pack 缺字段时有明确 fallback。

## 明确不做什么

- 不做资源包市场。
- 不做在线下载或远程更新。
- 不做复杂编辑器。
- 不要求最终美术资源。

## 用户体验

开发者或设计者可以更容易替换 Ian 资源，不会因为缺少某个动画导致应用空白或崩溃。

## 架构约束

- 资源包仍是 React 渲染层的视觉来源。
- Rust Core 只输出语义动作，不关心具体帧。
- 校验工具不能引入网络依赖。

## 数据 / 协议变化

扩展资源 schema 文档和测试 fixture。协议层保持语义动画名。

## 隐私与安全边界

只读取本地资源目录，不扫描任意用户文件。

## 验收标准

- [ ] 资源包 schema 文档存在并覆盖必填字段。
- [ ] 默认资源包通过校验。
- [ ] 缺失动画或表达式时 fallback 可预测。
- [ ] 校验错误信息能定位到具体文件和字段。
- [ ] 测试覆盖有效 pack、缺字段 pack、未知动画名。

## 验证方式

- Resource loader tests。
- Resource pack validation tests。
- `npm run desktop:test`
