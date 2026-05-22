# 0270 · Life Drive Model

## 问题 / 目标

Ian 现在有较多事件反应，但缺少持续的内部动机，导致行为像“触发动画”而不是“活着的小生命”。本 SDD 建立轻量 Life Drive Model，用好奇、安全、无聊、亲近、精力等内部驱动力影响 Moment 选择。

## 当前产品阶段

v0.1.x Life Feel / 生命感二阶段。

## 产品范围

- 在 Rust Core 中建立轻量内部驱动力模型。
- 驱动力只影响行为选择，不直接展示数值。
- 初始驱动力包括 curiosity、comfort、boredom、affection、energy。
- 驱动力由低敏 Ian 事件更新，例如点击、拖动、找回、idle tick、放下。
- 为后续 Micro Story 和 Motion Feel 提供决策上下文。

## 明确不做

- 不做完整 Mood System 或 Bond System。
- 不显示数值等级、经验条、好感度条。
- 不读取代码、屏幕、剪贴板、私聊或终端内容。
- 不让驱动力绕过用户控制、DND、reduced motion。

## 用户体验

用户不会看到一组参数，但会感觉 Ian 有“心情倾向”：太久没人理会会更好奇，被拖动后会更想安顿，连续互动后会更亲近，低精力时动作更慢。

## 架构约束

- Life Drive Model 属于 Rust Core。
- React 不能计算或维护驱动力。
- 驱动力输入必须来自 `IanEvent`、`IanState`、本地低敏配置和时间 tick。
- 输出只能通过 `IanAction` 间接影响动画、移动、气泡和 Moment。

## 数据 / 协议变化

允许新增内部 Rust 类型。默认不新增前端可见协议字段；如需要诊断字段，必须保持低敏并仅用于开发 / 验收。

## 隐私与安全边界

驱动力不得基于高敏内容推断用户状态，不记录用户画像，不持久化原始文本。

## 验收标准

- [ ] Rust Core 有明确的 Life Drive 类型或模块，包含 curiosity、comfort、boredom、affection、energy。
- [ ] 低敏事件能更新驱动力，且更新规则有单元测试。
- [ ] 驱动力不会直接暴露为用户可见分数。
- [ ] DND、quiet mode、reduced motion 下驱动力不能强行触发打扰行为。
- [ ] Moment 决策能读取驱动力上下文，但 React 不参与计算。

## 验证方式

- Rust 单元测试覆盖驱动力更新、边界夹取和策略读取。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 真实 Tauri 桌面 smoke，确认无启动回归。
