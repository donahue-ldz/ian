# 实现计划: Adaptive Cloud Bubble Size

## 对应规格

`docs/sdd/specs/0072-adaptive-cloud-bubble-size/spec.md`

## 实现步骤

1. 在 Bubble view test 中增加短句 / 长句尺寸分档断言，并先观察失败。
2. 在 `Bubble.tsx` 中基于 `formatBubbleText` 后的文本长度设置 `data-cloud-size`。
3. 在 CSS 中为 `short`、`medium`、`long` 和输入状态设置不同宽度、padding 与云朵凸起位置。
4. 运行前端测试和 typecheck。
5. 用 Browser smoke 截图记录短句紧凑气泡，并更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.tsx`
- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0072-adaptive-cloud-bubble-size/*`

## 接口 / 兼容性

不改变对外 props、协议和存储。新增 DOM data attribute 仅用于样式和测试。

## 验证命令

```bash
npm run desktop:test -- Bubble.view.test.tsx
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

过度紧凑可能让云朵边缘压住文字。回滚方式是提高 `short` 的最小宽度或只保留 `medium`/`long` 两档。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
