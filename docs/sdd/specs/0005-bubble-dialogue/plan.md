# 实现计划: Bubble Dialogue

## 状态

已实现，已验证。

## 概要

给现有 bubble 增加 P0 级短输入，复用 `DialogueUserMessage` 和 Demo Provider。

## 步骤

1. 添加前端 RED 测试。
   - 空输入不提交。
   - 非空输入调用 submit handler。

2. 改造 Bubble / IanStage。
   - Bubble 接收 `onSubmitMessage`。
   - 添加短输入框，回车提交。
   - 控件保持小尺寸，不做聊天窗口。

3. 连接 App 事件。
   - 提交时调用 `ian.sendEvent({ type: "dialogue.user_message", text })`。

4. 验证并记录。
   - `npm run desktop:test`
   - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue`
   - Playwright 输入验收。

## 预计文件改动

- `apps/desktop/src/renderer/Bubble.tsx`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/renderer/Bubble.test.tsx`
- `docs/sdd/specs/0005-bubble-dialogue/verification.md`

## 风险与回滚

风险是输入框让 Ian 像聊天工具。通过单句输入、紧凑 bubble、不显示历史来控制产品气质。回滚时移除 input 并保留点击短句。

