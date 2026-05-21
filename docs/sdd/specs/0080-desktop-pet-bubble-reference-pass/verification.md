# 0080 Desktop Pet Bubble Reference Pass Verification

## 状态

已验证。

## RED

- `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：失败，符合预期。
  - 失败点：缺少 `data-bubble-role="pet-status"`、`data-bubble-shape="thought-cloud"`、低干扰回复标记和桌面宠物锚点 CSS。

## GREEN / 回归

- [x] `npm run desktop:test -- Bubble.view.test.tsx`
  - 结果：通过，1 个文件 / 6 个测试。
- [x] `npm run desktop:test`
  - 结果：通过，11 个文件 / 56 个测试。
- [x] `npm run desktop:typecheck`
  - 结果：通过。
- [x] `npm run desktop:build`
  - 结果：通过，Vite 产物生成成功。
- [x] 浏览器视觉检查
  - URL：`http://127.0.0.1:1420/?v=0080`
  - 截图：`docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/screenshots/0080-desktop-pet-bubble.png`
  - Smoke 数据：`docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/screenshots/0080-desktop-pet-bubble-smoke.json`
  - 结果：`bubbleRole: pet-status`，`bubbleShape: thought-cloud`，`cloudSize: short`，`bottom: 152px`，`replyOpacity: 0.52`，`borderColor: rgba(17, 17, 17, 0.64)`，`borderWidth: 3px`，宽度约 `97px`。

## 剩余风险

- 本轮没有接入真实桌面宠物项目代码，参考只体现在 Ian 自有 React/CSS 设计中。
