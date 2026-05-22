# 0128 · Developer Rhythm Policy Engine V1

## 问题 / 目标

建立 Developer Rhythm Policy 的核心边界，把未来 Git/build/keyboard 事件转为低频 Ian 反应策略。

## 当前产品阶段

v0.2 Developer Rhythm Prep。

## 产品范围

- 定义策略输入、冷却、优先级和输出限制。
- 默认仅测试使用。

## 明确不做

- 不接真实 Git / 键盘用户可见行为。
- 不做通知中心。
- 不显示任务面板。

## 用户体验

用户看到的是 Ian 更稳定、更可控、更有生命感；如果本 SDD 属于未来能力准备，则默认不主动暴露成打扰性的用户可见功能。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责渲染、设置 UI、动画播放和执行 IanAction。
- 外部输入必须通过 IanEvent、Security Gate 或既有设置 command 进入系统。
- 高敏 adapter 必须默认关闭，并经过权限中心和 Security Gate。
- Resource Pack 不执行脚本、不访问网络、不携带行为逻辑。
- 不把 v0.2 / v0.3 能力提前做成默认用户可见功能。

## 数据 / 协议变化

默认不新增 IanEvent / IanAction / IanState 字段。若实现发现必须变更协议，必须先更新本 spec、plan 和 decisions.md，再进入实现。

## 隐私与安全边界

- 默认不读取代码正文、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出。
- 只允许本 SDD 明确列出的低敏本地数据进入持久化。
- API key、用户正文和敏感 payload 不得写入普通日志或 config.toml。

## 验收标准

- [ ] 策略输入只接摘要事件。
- [ ] 输出仍是短句 / 动作。
- [ ] 冷却防止频繁打扰。
- [ ] 测试覆盖成功、失败、忙碌等摘要。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行 workspace 类型检查或记录无法运行原因。
- 涉及桌面可见能力时，必须在真实 Tauri 桌面壳中验收。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
