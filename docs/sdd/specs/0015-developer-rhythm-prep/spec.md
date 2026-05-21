# Spec: Developer Rhythm Prep

## 状态

已实现，待验收。

## 问题 / 目标

v0.2 的 Developer Rhythm 需要让 Ian 感知开发节奏，但不能突然读取代码、键盘或终端。0015 目标是为 v0.2 做低敏 adapter 准备：定义事件边界、权限需求和 no-op skeleton，不开放高敏用户可见功能。

## 当前产品阶段

v0.2 准备，不属于 P0/v0.1.x 用户可见功能。

## 产品范围

- 定义 Developer Rhythm adapter trait / event 分类。
- 增加 Git/build/test 等低敏事件的协议草案或内部 skeleton。
- 所有 adapter 默认关闭。
- 建立权限说明和后续 SDD 切分建议。

## 明确不做什么

- 不读取代码正文。
- 不读取终端输出全文。
- 不监听全局键盘。
- 不感知活动窗口。
- 不做 Git 集成用户功能。

## 用户体验

用户在 0015 不应看到新的 Developer Rhythm 功能；最多看到设置里相关能力仍未启用。

## 架构约束

- Adapter 只能把外部信号转换为 `IanEvent`。
- Security Gate 必须在 adapter 事件进入 Runtime 前检查。
- 行为仍由 Rust Core 决定。

## 数据 / 协议变化

可新增内部事件草案、adapter trait 实现 skeleton、权限枚举；不要求前端展示新行为。

## 隐私与安全边界

默认不读取项目内容。任何未来 Git/Build/Test 事件都只能从明确授权、低敏元数据开始。

## 验收标准

- [ ] Developer Rhythm adapter skeleton 存在且默认关闭。
- [ ] 新事件边界明确区分低敏元数据和高敏内容。
- [ ] Security Gate 默认拒绝未授权 developer source。
- [ ] 没有用户可见 Git/build/test 行为。
- [ ] 文档列出后续 v0.2 SDD 切分建议。

## 验证方式

- Rust adapter/security 单元测试。
- `rg` 检查无代码正文读取、全局键盘监听、终端全文读取。
- 前端 smoke 确认无新入口抢占 P0/v0.1.x 体验。
