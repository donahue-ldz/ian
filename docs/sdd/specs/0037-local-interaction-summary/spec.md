# Spec: Local Interaction Summary

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要有一点“记得我们相处过”的连续性，但不能提前做复杂长期记忆。0037 目标是基于 0031 的低敏生活事件生成本地互动摘要，只保留计数、最近互动时间、偏好倾向等受控信息。

## 当前产品阶段

v0.1.x Local State。

## 产品范围

- 从 life events 聚合本地互动摘要。
- 摘要字段包括 interaction_count、last_interaction_at、recent_activity_level 等。
- Bond / Dialogue / Behavior 可读取摘要。
- 提供保留期或压缩策略。

## 明确不做什么

- 不保存用户输入正文。
- 不做向量记忆。
- 不做云同步。
- 不生成用户画像或偏好标签。

## 用户体验

Ian 可以根据“最近经常互动”或“很久没被点过”做轻微差异反馈，但不会展示数据面板。

## 架构约束

- 摘要由 Rust storage / domain 层生成。
- React 不计算长期摘要。
- 摘要只能使用低敏 life event。

## 数据 / 协议变化

可新增 `interaction_summary` repository 或 service。对外只暴露给 Rust Core 内部策略。

## 隐私与安全边界

摘要只包含低敏统计，不包含语义文本、外部应用、代码或终端内容。

## 验收标准

- [ ] 可从 life events 生成本地 interaction summary。
- [ ] summary 不包含用户输入正文或外部上下文。
- [ ] Bond / Dialogue 至少一个模块可消费 summary。
- [ ] 支持保留期或压缩策略。
- [ ] 测试覆盖聚合、空日志、敏感字段排除。

## 验证方式

- Rust storage / domain tests。
- Source scan 检查 summary 字段。
- Dialogue 或 Bond 集成测试。
