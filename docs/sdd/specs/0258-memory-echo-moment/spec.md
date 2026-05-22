# 0258 · Memory Echo Moment

## 问题 / 目标

记忆如果只存在设置里，用户感受不到连续陪伴；如果频繁提起，又会像监控。本 SDD 设计低频“记忆回响瞬间”：只使用用户确认过的低敏记忆，在合适时机轻轻提一句。

## 当前产品阶段

v0.1.x / Memory Prep。

## 产品范围

- 只读取 confirmed memory tags。
- 在早安、久别回来、找回或 idle surprise 中极低频使用。
- 文案短、自然、可关闭。
- 用户删除记忆后不再使用。

## 明确不做

- 不使用未确认 candidate。
- 不保存或复述聊天全文。
- 不做用户画像。
- 不联网。

## 用户体验

Ian 偶尔说“我记得你喜欢我安静一点。”这类短句，让用户感到被记住，但不会频繁或冒犯。

## 架构约束

- Rust Core / DialoguePolicy 决定是否使用 confirmed memory。
- MemoryRepository 只返回低敏 tags。
- React 不直接读取数据库。

## 数据 / 协议变化

可能需要新增 memory query command 或 runtime context，但不能暴露原始敏感内容。

## 隐私与安全边界

- 只使用用户确认过的低敏 tags。
- 删除或清空后立即停止使用。
- 不导出、不上传、不拼接原始文本。

## 验收标准

- [ ] 未确认记忆不会被使用。
- [ ] confirmed 记忆可低频进入气泡文案。
- [ ] 删除后不再使用该记忆。
- [ ] 文案短且不显得监控。
- [ ] 测试覆盖隐私边界。

## 验证方式

- Rust memory usage 测试。
- Dialogue 输出策略测试。
- 桌面气泡验收。
