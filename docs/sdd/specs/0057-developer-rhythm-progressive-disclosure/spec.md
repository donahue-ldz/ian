# Spec: Developer Rhythm Progressive Disclosure

## 状态

已实现，待用户验收。

## 问题 / 目标

Developer Rhythm 是 Ian 面向开发者的重要增强，但不应抢占首次体验。0057 目标是把开发者节奏能力放到渐进暴露路径中，让用户先感受到 Ian 活着，再决定是否让 Ian 理解开发节奏。

## 当前产品阶段

Product Feel 体验优化。

## 产品范围

- 默认首屏不主动展示 Git、build/test、keyboard rhythm 等开发术语。
- Developer Rhythm 入口放在高级、隐私或后续引导中。
- 用户完成基础体验后，可以看到一句短引导，而不是强制配置。
- 未开启时不显示开发者节奏反应或空状态噪音。

## 明确不做什么

- 不删除 0041-0050 的能力。
- 不默认启用开发者节奏。
- 不做 IDE 插件。
- 不做项目扫描。

## 用户体验

新用户先看到 Ian 作为桌面生命存在。只有当用户进入设置、高级能力或明确表达开发者需求时，才看到 Developer Rhythm 的说明和授权。

## 架构约束

- Developer Rhythm 的默认关闭策略保持不变。
- 渐进暴露只影响 UI 和 onboarding，不改变 Security Gate。
- React 不根据项目状态自动开启或暗示已开启能力。

## 数据 / 协议变化

可新增本地 onboarding flag，例如是否看过 Developer Rhythm 引导。该字段不得代表权限授权。

## 隐私与安全边界

未授权前不读取 Git、build/test、键盘节奏或活动应用类别。引导文案必须明确这是可选能力。

## 验收标准

- [ ] 默认首次体验不出现 Git、build/test、keyboard rhythm 等主路径术语。
- [ ] Developer Rhythm 可从设置或高级路径找到。
- [ ] 引导不强制开启权限。
- [ ] 未开启时没有开发者节奏空状态提示污染主体验。
- [ ] 开启路径复用 0041 的授权和 0056 的隐私文案。

## 验证方式

- 前端测试覆盖首次体验和设置路径。
- Security tests 确认未授权仍拒绝。
- Browser smoke 截图：首次体验、设置入口、Developer Rhythm 引导。
