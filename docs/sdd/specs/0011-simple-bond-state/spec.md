# Spec: Simple Bond State

## 状态

已实现，已验证。

## 问题 / 目标

Ian 的陪伴感需要逐步累积，但不能做成 RPG 等级或经验条。0011 目标是建立简单 Bond State，用本地互动次数和最近互动时间形成内部关系阶段，影响语气和少量行为。

## 当前产品阶段

v0.1.x Product Iteration。

## 产品范围

- Rust Core 定义简单 BondStateView。
- 根据本地互动事件更新内部关系阶段。
- Bond 影响 Demo Dialogue 的称呼和亲近程度。
- Bond 可持久化，但不显示数值。

## 明确不做什么

- 不显示等级、经验条、亲密度数值。
- 不做成就系统。
- 不使用云端账户。
- 不读取外部社交数据。

## 用户体验

多互动几次后，Ian 的回复可以更熟悉一点，但用户看不到“等级系统”。

## 架构约束

- BondEngine 属于 Rust Core。
- BondSignal 来自明确的本地 Ian 互动事件。
- React 不计算 Bond，不展示数值。

## 数据 / 协议变化

可新增本地 bond repository 或复用 interaction events 聚合；如 `IanState` 暴露 bond view，需同步 TS 类型。

## 隐私与安全边界

只基于本地 Ian 互动记录，不采集外部内容。

## 验收标准

- [ ] Rust Core 存在简单 BondEngine 和 BondStateView。
- [ ] 本地互动事件可更新 bond view，且可测试。
- [ ] Bond 状态可持久化或由 interaction events 可重复推导。
- [ ] DialogueContext 可接收 bond view。
- [ ] UI 不显示等级、经验条或亲密度数值。

## 验证方式

- Rust bond 单元测试。
- Storage/repository 测试。
- Dialogue context 测试。
