# Spec: Dialogue Provider 边界收敛

## 状态

草稿，等待用户确认。

## 背景

0001 已实现 Demo Dialogue 的最小短句回复，并保留 OpenAI-compatible BYOM 的后续方向。为了避免 P0 变成聊天壳，需要先把 Dialogue Provider 边界、DialoguePolicy 和气泡回复规则收敛清楚。

0004 聚焦语言能力边界，不做完整 BYOM UI。

## 目标

- 明确 `DemoDialogueProvider`、未来 `OpenAICompatibleProvider` 与 `DialogueEngine` 的职责。
- 强化 DialoguePolicy：短句、角色语气、气泡友好、不像 AI Assistant。
- 让点击回复和用户文本回复都经过统一 dialogue boundary。
- 为未来 BYOM 接入保留接口，但不在 P0 暴露成核心体验。

## 当前阶段

```txt
P0 / MVP + v0.1.x Product Iteration
```

## 产品范围

- Demo Dialogue 可返回更多短句。
- 气泡回复保持短小、克制、有角色感。
- 支持基本输入文本到回复的本地规则。
- 保留 provider trait / interface，为后续 BYOM 做准备。

## 非目标

- 不做 BYOM 设置 UI。
- 不请求或保存 API key。
- 不接入真实网络 LLM。
- 不做流式输出。
- 不做长期对话记忆。
- 不做复杂 prompt builder。

## 用户体验

点击或输入简单文本后，Ian 只返回短句。

Ian 不应说：

```txt
作为 AI 助手……
我可以帮你完成任务……
```

Ian 应更像：

```txt
我在这儿。
慢慢来。
喝水水。
才不是担心你。
```

## 架构约束

- Dialogue 是 Rust Core 能力，不是 React 能力。
- React 只展示 `SpeechShow`。
- DialogueProvider 只产出候选文本。
- DialoguePolicy 负责裁剪、语气边界和气泡适配。
- 未来 BYOM 必须通过 provider boundary 和安全存储，不得直接从 React 调 API。

## 数据与协议变化

预期不新增协议类型。

允许调整：

- `DialogueUserMessage` 处理路径。
- Dialogue provider trait。
- Demo provider response table。
- DialoguePolicy 裁剪规则。

## 隐私与安全

P0 / 0004 不发送用户文本到网络。

不保存完整对话历史。

不读取代码、剪贴板、屏幕、私聊内容。

## 验收标准

- [ ] Demo Dialogue 通过 provider boundary 调用。
- [ ] DialoguePolicy 对回复长度和角色语气有明确约束。
- [ ] 点击回复和 `DialogueUserMessage` 回复都经过 Rust Core。
- [ ] React 只渲染 `SpeechShow`，不生成核心回复。
- [ ] 默认回复不依赖网络或 API key。
- [ ] 回复不会出现“作为 AI 助手”等 assistant 话术。
- [ ] BYOM 仅保留接口或后续记录，不作为 0004 用户可见功能。
- [ ] Rust check 通过。
- [ ] 前端 typecheck / build 通过。

## 验证方式

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

可补充 Rust 单元测试或前端 reducer 测试。

## 开放问题

- 0004 是否需要增加 Rust 单元测试覆盖 DialoguePolicy？
- Demo Dialogue 的短句池是否应先写在 Rust 代码中，还是放入 Resource Pack / 配置？

