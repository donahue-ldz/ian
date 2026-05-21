# 0083 Bubble Text Full Center Plan

## 实现步骤

1. 更新 `Bubble.view.test.tsx`，增加默认气泡上下左右居中的 CSS 断言。
2. 运行局部测试确认 RED。
3. 更新 `ianStage.css`：默认气泡使用 grid 居中，文字去掉底部 margin，回复按钮改为 absolute。
4. 为 input 尺寸气泡保留 stretch 布局和文字下间距。
5. 运行验证、截图并更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0083-bubble-text-full-center/spec.md`
- `docs/sdd/specs/0083-bubble-text-full-center/plan.md`
- `docs/sdd/specs/0083-bubble-text-full-center/decisions.md`
- `docs/sdd/specs/0083-bubble-text-full-center/verification.md`

## 风险和回滚

- 风险：回复按钮 absolute 后可能更靠近文字，视觉上需要浏览器验收。
- 回滚：恢复 `.ian-bubble-text` margin 和 `.ian-bubble-reply` flow 布局。

## 执行结果

- 默认 `.ian-bubble` 改为 `display: grid` + `place-items: center`。
- `.ian-bubble-text` 设置 `align-self: center`、`justify-self: center`、`margin: 0`。
- `.ian-bubble-reply` 改为绝对定位，避免参与正文排版。
- `input` 状态气泡保留 `place-items: stretch` 和正文下间距，输入框仍按原布局显示。
