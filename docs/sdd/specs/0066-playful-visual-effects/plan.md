# 实现计划: Playful Visual Effects

## 对应规格

`docs/sdd/specs/0066-playful-visual-effects/spec.md`

## 实现步骤

1. 定义 playful 视觉状态和 fallback 规则。
2. 扩展资源包 manifest 或 animation mapping。
3. 实现 React visual effect 执行层。
4. 接入 reduced motion / quiet mode。
5. 截图或录屏验证效果和层级。

## 预计改动文件

- `apps/desktop/resources/*`
- `apps/desktop/src/renderer/*Animation*`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/protocol/generated.ts`
- `docs/sdd/specs/0066-playful-visual-effects/*`

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:dev
```

## 风险和回滚

视觉效果过强会降低桌面舒适度。回滚方式是默认只启用轻效果，把强效果挂到 high playful energy。
