# 实现计划: Resource Pack 渲染收敛

## Spec

`docs/sdd/specs/0003-resource-pack-renderer/spec.md`

## 状态

草稿，等待用户确认。

## 概要

将 Ian 视觉渲染进一步收敛到 Resource Pack manifest，而不是散落在组件常量或 CSS 假设中。

## 步骤

1. 梳理当前 Resource Pack 加载链路。
2. 明确 `PetManifest`、`AnimationManifest`、`ExpressionManifest` 类型。
3. 调整 `AnimationPlayer` / `IanSprite`，让动画识别来自 manifest。
4. 添加资源加载 fallback。
5. 为资源加载和 action -> animation 映射补测试。
6. 运行前端 typecheck / build / test。
7. 更新 verification。

## 预计文件改动

- `apps/desktop/src/resources/resourceLoader.ts`
- `apps/desktop/src/resources/animationTypes.ts`
- `apps/desktop/src/renderer/AnimationPlayer.ts`
- `apps/desktop/src/renderer/IanSprite.tsx`
- `apps/desktop/public/resources/pets/ian-alpaca/*`
- `apps/desktop/src/state/*.test.ts`
- `docs/sdd/specs/0003-resource-pack-renderer/verification.md`
- `docs/sdd/specs/0003-resource-pack-renderer/decisions.md`

## 验证命令

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
```

## 风险

- 过早抽象资源包系统会拖慢 P0；只收敛渲染边界，不做资源生态。
- 如果引入 bitmap sprite sheet，需要额外视觉资产和截图验证。

## 回滚说明

可回退前端 resource loader / renderer 相关文件，保留 manifest 文件结构。

