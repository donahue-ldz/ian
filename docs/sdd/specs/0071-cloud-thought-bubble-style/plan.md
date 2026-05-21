# 实现计划: Cloud Thought Bubble Style

## 对应规格

`docs/sdd/specs/0071-cloud-thought-bubble-style/spec.md`

## 实现步骤

1. 增加 Bubble view / CSS 验收测试，先验证当前样式不满足云朵气泡要求。
2. 调整 `.ian-bubble` 的 CSS：白底、黑色粗描边、云朵圆形凸起、小圆尾巴。
3. 调整内部文字、输入框和回复按钮层级，保证内容位于云朵主体上方。
4. 运行前端测试和 typecheck。
5. 用 Browser smoke 截图记录新样式，并更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0071-cloud-thought-bubble-style/*`

## 接口 / 兼容性

不改变 React props、Rust protocol、持久化或资源包结构。旧 bubble 行为保持兼容。

## 验证命令

```bash
npm run desktop:test -- Bubble.view.test.tsx
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

云朵边缘可能占用更多空间或遮挡内容。回滚方式是保留白底黑描边，减少圆形凸起和尾巴尺寸。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
