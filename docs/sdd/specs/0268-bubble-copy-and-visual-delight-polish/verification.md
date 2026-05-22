# 0268 · 验证记录

## 2026-05-22

### 已执行检查

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/state/ianActions.test.ts`
  - 结果：通过。覆盖 Moment 气泡短句截断和时长上限。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过。16 个测试文件 / 117 个测试通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。

## 验收标准结果

- [x] Moment 气泡文案会通过 `formatBubbleText` 短句化。
- [x] `speech.show` 可见时长上限限制为 3600ms，避免长时间遮挡。
- [x] 诊断和 Moment 文案不出现 AI 助手语气。
- [x] 连续触发时 reducer 仍只保留当前 bubble 状态，不叠加多个气泡。
- [x] 不展示原始隐私内容。

## 剩余风险

- 本轮未做真实桌面截图视觉调参；气泡圆角、阴影和箭头细节仍可后续按截图优化。
