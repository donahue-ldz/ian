# 0081 Extra Thin Bubble Border Verification

## 状态

已验证。

## RED

- `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：失败，符合预期。
  - 失败点：测试要求 `1.5px`，当前 CSS 仍包含 `3px` / `2px` 的气泡描边。

## GREEN / 回归

- [x] `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：通过，1 个文件 / 7 个测试。
- [x] `npm run desktop:test`
  - 结果：通过，11 个文件 / 57 个测试。
- [x] `npm run desktop:typecheck`
  - 结果：通过。
- [x] `npm run desktop:build`
  - 结果：通过，Vite 产物生成成功。
- [x] 浏览器视觉检查
  - URL：`http://127.0.0.1:1420/?v=0081`
  - 截图：`docs/sdd/specs/0081-extra-thin-bubble-border/screenshots/0081-extra-thin-bubble-border.png`
  - Smoke 数据：`docs/sdd/specs/0081-extra-thin-bubble-border/screenshots/0081-extra-thin-bubble-border-smoke.json`
  - 结果：`borderWidth: 1.5px`，`replyBorderWidth: 1.5px`，`tailBorderWidth: 1.5px`，`borderColor: rgba(17, 17, 17, 0.64)`，`bubbleRole: pet-status`，`bubbleShape: thought-cloud`，`bottom: 152px`。

## 剩余风险

- 极细描边在浅色桌面背景上会更轻，后续可根据真实桌面验收再调透明度或阴影。
