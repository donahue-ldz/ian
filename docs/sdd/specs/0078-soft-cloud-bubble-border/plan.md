# 实现计划: Soft Cloud Bubble Border

## 对应规格

`docs/sdd/specs/0078-soft-cloud-bubble-border/spec.md`

## 实现步骤

1. 更新 Bubble view test，要求云朵气泡使用柔和 ink token 和 3px 主描边，并先观察失败。
2. 调整 `ianStage.css` 的 cloud ink、主 border、云朵凸起 spread 和尾巴圆点 border。
3. 确认短句自适应尺寸和尾巴节点仍存在。
4. 运行前端测试、typecheck 和 Browser smoke。
5. 更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0078-soft-cloud-bubble-border/*`

## 接口 / 兼容性

不改变 DOM 结构、React props、协议或存储。仅改变 CSS token 和边框数值。

## 验证命令

```bash
npm run desktop:test -- Bubble.view.test.tsx
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

描边过淡可能在浅色背景上不清楚。回滚方式是保留 3px 宽度，把 ink alpha 从 0.64 提高到 0.78。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
