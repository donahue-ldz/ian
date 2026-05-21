# Spec: Bubble Dialogue

## 状态

已实现，已验证。

## 问题 / 目标

P0 要求 Demo Dialogue 可用，但当前点击只展示固定短句，没有用户输入。0005 目标是在不变成聊天产品的前提下，让用户能在气泡里输入一句话，发送 `DialogueUserMessage`，由 Rust Core Demo Dialogue 返回短句回复。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 气泡打开时提供一个紧凑输入框。
- 用户输入一句短文本并提交。
- 前端发送 `dialogue.user_message`。
- Rust Core 返回 `speech.show` 和轻动画。
- 气泡显示 Ian 回复。

## 明确不做什么

- 不做多轮聊天窗口。
- 不做消息历史。
- 不做 Markdown。
- 不做 BYOM / 网络模型。
- 不做复杂设置 UI。

## 用户体验

点击 Ian 后出现小气泡和输入框；输入“你在干嘛”并回车后，Ian 用短句回复。整个体验仍像和桌面小生物互动，而不是打开聊天应用。

## 架构约束

- React 只采集输入并发送 `DialogueUserMessage`。
- 回复文本由 Rust Core DialogueEngine 决定。
- 输出继续通过 `IanAction::SpeechShow`。

## 数据 / 协议变化

不新增协议类型，复用 `DialogueUserMessage`。

## 隐私与安全边界

只处理用户主动在 Ian 气泡输入的文本；输入经过现有 Security/Sanitizer 入口。

## 验收标准

- [ ] 点击 Ian 后气泡包含短输入框。
- [ ] 输入文本回车会发送 `dialogue.user_message`。
- [ ] Rust Core Demo Dialogue 返回短句并显示在气泡中。
- [ ] 空输入不会发送事件。
- [ ] 气泡输入不会破坏单击短句、双击 run-around。
- [ ] Playwright 可验证输入“你在干嘛”后显示 Demo 回复。

## 验证方式

- 前端 reducer / UI 测试。
- Rust dialogue 测试。
- Playwright 本地验收。

