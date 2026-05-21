# 0082 Centered Bubble Text Plan

## 实现步骤

1. 更新 `Bubble.view.test.tsx`，增加气泡文字居中的 CSS 断言。
   - 对应验收标准：1、2、3。
2. 运行局部测试确认 RED。
   - 对应验收标准：1。
3. 更新 `ianStage.css` 的 `.ian-bubble-text`，设置 `text-align: center`。
   - 对应验收标准：1、2、3。
4. 运行验证，保存截图，更新 `verification.md`。
   - 对应验收标准：1、2、3。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0082-centered-bubble-text/spec.md`
- `docs/sdd/specs/0082-centered-bubble-text/plan.md`
- `docs/sdd/specs/0082-centered-bubble-text/decisions.md`
- `docs/sdd/specs/0082-centered-bubble-text/verification.md`

## 接口 / 模块影响

无接口变化。仅 CSS 视觉调整和测试断言更新。

## 兼容性说明

`text-align: center` 对单行和多行文本都稳定；现有 `-webkit-line-clamp` 截断规则继续保留。

## 风险和回滚

- 风险：很长文本居中后阅读节奏略弱，但桌面宠物短句优先，符合 P0。
- 回滚：移除 `.ian-bubble-text` 的 `text-align: center`。

## 执行结果

- 已给 `.ian-bubble-text` 增加 `text-align: center`。
- 已保留 `display: -webkit-box`、`-webkit-box-orient: vertical` 和 `-webkit-line-clamp: 2`。
- 未改变气泡锚点、极细边框、自适应尺寸或输入行为。
