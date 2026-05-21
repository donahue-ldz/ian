# 0083 Bubble Text Full Center Verification

## 状态

已验证。

## RED

- `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：失败，符合预期。
  - 失败点：`.ian-bubble` 缺少 `display: grid` / `place-items: center`，`.ian-bubble-text` 未清除底部 margin，`.ian-bubble-reply` 仍参与正文排版。

## GREEN / 回归

- [x] `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：通过，1 个文件 / 8 个测试。
- [x] `npm run desktop:test`
  - 结果：通过，12 个文件 / 62 个测试。
- [x] `npm run desktop:typecheck`
  - 结果：通过。
- [x] `npm run desktop:build`
  - 结果：通过，Vite 产物生成成功。
- [x] 浏览器视觉检查
  - URL：`http://127.0.0.1:1420/?v=0083`
  - 截图：`docs/sdd/specs/0083-bubble-text-full-center/screenshots/0083-bubble-text-full-center.png`
  - Smoke 数据：`docs/sdd/specs/0083-bubble-text-full-center/screenshots/0083-bubble-text-full-center-smoke.json`
  - 结果：`display: grid`，`placeItems: center`，`textAlign: center`，`textMarginBottom: 0px`，`replyPosition: absolute`，文字中心相对气泡中心偏移约 `x: 0` / `y: 0.5px`。

## 剩余风险

- 回复按钮绝对定位后在极短文本气泡中更靠近正文，后续可按真实桌面观感微调右下角位置。
