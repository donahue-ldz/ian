# Spec: Dialogue Provider Boundary

## 状态

已实现，已验证。

## 问题 / 目标

0001 已有 Demo Dialogue skeleton，但 provider 边界仍偏薄：对话输入、上下文、策略裁剪和 provider 选择没有形成清晰协议。0003 的目标是把 Dialogue Provider Boundary 固化为 Rust Core 内的稳定边界，让 P0 继续使用本地 Demo Provider，同时为未来 BYOM 留出不暴露用户可见功能的接口。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- Rust Core 定义 `DialogueProvider` trait。
- Demo Provider 通过 trait 被 `DialogueEngine` 调用。
- `DialogueContext` 显式携带当前动画、行为和输入来源。
- `DialoguePolicy` 继续负责短句裁剪和身份约束。
- 前端行为不在 0003 扩展，用户可见变化只允许是回复更稳定。

## 明确不做什么

- 不实现 BYOM UI。
- 不请求 API Key。
- 不访问网络。
- 不实现流式回复。
- 不实现长期记忆或完整 Mood/Bond 上下文。

## 用户体验

点击或后续输入触发 Demo Dialogue 时，Ian 仍以短句气泡回应，不出现 AI assistant 口吻。

## 架构约束

- `DialogueUserMessage` 进入 Rust Core。
- `DialogueEngine` 负责 provider 编排。
- React 只发送事件和渲染 `IanAction`。
- Provider 输出必须经过 `DialoguePolicy`。

## 数据 / 协议变化

不新增外部 IPC 类型。Rust 内部新增 dialogue context/provider trait。

## 隐私与安全边界

0003 只处理 Ian 窗口内输入，不访问网络、不读取文件、不读剪贴板。

## 验收标准

- [ ] Rust Core 存在 `DialogueProvider` trait，Demo Provider 实现该 trait。
- [ ] `DialogueEngine` 通过 provider trait 生成回复，而不是直接调用具体 demo 方法。
- [ ] `DialogueContext` 至少包含当前 behavior、animation 和 source。
- [ ] Demo 回复经过 `DialoguePolicy` 裁剪，最大长度可测试。
- [ ] `DialogueUserMessage` 仍返回 `speech.show` 和 `animation.play`。
- [ ] 前端测试、Rust 检查通过。

## 验证方式

- Rust 单元测试覆盖 provider dispatch 和 policy trimming。
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`

