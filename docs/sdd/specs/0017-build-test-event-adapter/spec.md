# Spec: Build Test Event Adapter

## 状态

已实现，待验收。

## 问题 / 目标

Developer Rhythm 需要感知构建和测试结果，但 Ian 不应读取完整终端输出。0017 目标是接入用户显式上报或本地工具产生的 build/test summary 事件，只处理成功/失败、耗时、测试数量等低敏摘要。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 定义 build/test summary 事件。
- 支持本地命令或开发工具把摘要发送给 Ian。
- Ian 可对 success/failure 做轻量反应。
- 事件默认需要权限开启。

## 明确不做什么

- 不读取终端全文。
- 不自动运行测试或构建。
- 不解析错误堆栈全文。
- 不读取源码路径之外的文件内容。
- 不上传结果。

## 用户体验

启用后，测试通过时 Ian 可以开心一下；测试失败时 Ian 可以短暂担心，但不展示长错误日志。

## 架构约束

- 外部 build/test 结果必须先转为 `IanEvent`。
- Security Gate 检查 source、payload size 和 permission。
- BehaviorPolicy 决定反应强度。

## 数据 / 协议变化

新增 build/test summary event，payload 只允许状态、耗时、数量、工具名和可选短错误类别。

## 隐私与安全边界

默认关闭。不接收完整 stdout/stderr，不存储长日志。

## 验收标准

- [ ] build/test summary event 类型存在。
- [ ] Security Gate 拒绝未授权或过大 payload。
- [ ] 成功/失败摘要可驱动不同 IanAction。
- [ ] 不读取终端全文或错误堆栈全文。
- [ ] 测试覆盖 success、failure、oversized payload。

## 验证方式

- Rust protocol/security/behavior 测试。
- source scan 检查无终端全文读取。
- 手动发送 mock build/test summary 做 smoke。
