# Verification: Soft Cloud Bubble Border

## 状态

已验证。

## 需要记录的验证

- [x] RED：`npm run desktop:test -- Bubble.view.test.tsx`
- [x] GREEN：`npm run desktop:test -- Bubble.view.test.tsx`
- [x] `npm run desktop:test`
- [x] `npm run desktop:typecheck`
- [x] Browser smoke 截图

## 结果

- RED：`npm run desktop:test -- Bubble.view.test.tsx` 失败，原因是旧样式仍为 `--ian-cloud-bubble-ink: #111` 和 4px 主描边。
- GREEN：`npm run desktop:test -- Bubble.view.test.tsx` 通过，1 file / 4 tests。
- `npm run desktop:test`：通过，11 files / 50 tests。
- `npm run desktop:typecheck`：通过。
- `npm run desktop:build`：通过，Vite build 完成。
- Browser smoke：气泡主描边为 `rgba(17, 17, 17, 0.64)`，宽度为 `3px`；短句气泡仍为 `short`，文本、回复按钮和两个尾巴圆点可见。
- 截图：`docs/sdd/specs/0078-soft-cloud-bubble-border/screenshots/0078-soft-cloud-bubble-border.png`。
- Smoke JSON：`docs/sdd/specs/0078-soft-cloud-bubble-border/screenshots/0078-soft-cloud-bubble-border-smoke.json`。
