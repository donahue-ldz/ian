# 0081 Extra Thin Bubble Border Plan

## 实现步骤

1. 更新 `Bubble.view.test.tsx`，要求云朵气泡相关描边为 `1.5px`，并确认不再包含 `3px solid var(--ian-cloud-bubble-ink)`。
   - 对应验收标准：1、2、3、4。
2. 运行局部测试确认 RED。
   - 对应验收标准：1、2、3。
3. 更新 `ianStage.css`，将主气泡、云朵瓣、尾泡、回复按钮和输入框描边改成 `1.5px`。
   - 对应验收标准：1、2、3、4、5。
4. 微调云朵瓣 `box-shadow` 的描边扩散值，让极细描边保持连贯。
   - 对应验收标准：2。
5. 运行验证，保存浏览器截图，更新 `verification.md`。
   - 对应验收标准：1、2、3、4、5。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0081-extra-thin-bubble-border/spec.md`
- `docs/sdd/specs/0081-extra-thin-bubble-border/plan.md`
- `docs/sdd/specs/0081-extra-thin-bubble-border/decisions.md`
- `docs/sdd/specs/0081-extra-thin-bubble-border/verification.md`

## 接口 / 模块影响

无接口变化。仅 CSS 视觉调整和测试断言更新。

## 兼容性说明

CSS `1.5px` 描边在现代 WebView 中可用；在不同缩放比例下会由渲染引擎做抗锯齿处理。

## 风险和回滚

- 风险：极细描边在浅色背景上可能不够明显。
- 回滚：恢复气泡相关 `border` 和云朵瓣扩散值到 0080 状态。

## 执行结果

- 已将主气泡、顶部云朵瓣、尾部小泡泡、回复按钮和输入框描边统一调为 `1.5px`。
- 已保留 `rgba(17, 17, 17, 0.64)` 淡色描边。
- 已微调云朵瓣 `box-shadow` 扩散值，让细描边保持云朵轮廓。
- 未改变气泡锚点、语义、文本尺寸分档或输入行为。
