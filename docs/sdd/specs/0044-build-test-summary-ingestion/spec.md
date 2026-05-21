# Spec: Build Test Summary Ingestion

## 状态

已实现，待验收。

## 问题 / 目标

0017 已定义 build/test summary event。0044 目标是建立稳定的摘要输入方式，让 Ian 可以接收用户或工具显式上报的构建测试结果，但不读取终端全文、不自动运行命令。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 定义本地 summary ingestion command 或 adapter endpoint。
- 只接收状态、耗时、测试数量、失败数量、短错误类别。
- 未授权、未绑定 workspace、payload 过大时拒绝。
- 成功/失败摘要进入 DeveloperRhythmPolicy。

## 明确不做什么

- 不自动运行测试。
- 不读取 stdout/stderr 全文。
- 不解析 stack trace。
- 不读取源码文件。
- 不上传结果。

## 用户体验

当测试通过或失败时，Ian 可以有一点轻量反应，但用户不会看到长日志或错误墙。

## 架构约束

- 所有摘要以 IanEvent 进入 Rust Core。
- Security Gate 检查权限、大小和敏感字段。
- Adapter / command 不直接控制动画。

## 数据 / 协议变化

复用 `developer.build_test_summary`。如新增 ingestion command，payload 必须等价于该事件结构。

## 隐私与安全边界

只允许摘要字段，不允许 stdout、stderr、stack trace、file content。

## 验收标准

- [ ] 有明确 build/test summary ingestion 入口。
- [ ] 未授权、未绑定 workspace、过大 payload 被拒绝。
- [ ] success / failure 摘要能进入 DeveloperRhythmPolicy。
- [ ] payload 不包含终端全文或 stack trace。
- [ ] 测试覆盖 success、failure、oversized、未授权、敏感字段。

## 验证方式

- Rust command / adapter / security tests。
- Source scan。
- Mock summary smoke。
