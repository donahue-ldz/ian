# Spec: Local Life Event Journal

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要长期存在感，但不能一开始就做复杂长期记忆。0031 目标是建立本地生活事件日志，只记录 Ian 自身的低敏行为事件，用于连续性、调试和后续策略输入。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 记录 Ian 自身事件：启动、移动、休息、醒来、点击互动、回 anchor。
- 事件追加写入本地 SQLite 或现有 repository。
- 事件 payload 只包含低敏摘要和时间戳。
- 提供按时间范围读取的内部接口，供后续摘要和验收使用。

## 明确不做什么

- 不记录用户输入正文。
- 不记录窗口标题、URL、Git、终端或代码内容。
- 不做语义长期记忆。
- 不做云同步或上传。

## 用户体验

用户不直接看到日志，但 Ian 的后续行为可以更稳定地基于“自己刚刚做过什么”。

## 架构约束

- Rust Core / storage repository 负责写入和读取。
- React 不直接写生活日志。
- 日志写入失败不能导致 Ian 无法启动。

## 数据 / 协议变化

新增本地 `life_events` repository 或扩展现有 interaction event repository。事件类型应是枚举或受控字符串。

## 隐私与安全边界

只记录 Ian 自身低敏事件，不记录用户文本、外部应用信息或敏感来源 payload。

## 验收标准

- [ ] 本地 life event repository 存在。
- [ ] 移动、休息、点击、启动等核心事件可追加记录。
- [ ] payload 不包含用户输入正文或外部上下文。
- [ ] repository 读取失败或写入失败不会阻塞应用启动。
- [ ] 测试覆盖写入、读取、低敏字段和失败恢复。

## 验证方式

- Rust storage tests。
- Source scan 检查无敏感字段写入。
- 本地 smoke：触发几类行为后能读取事件摘要。
