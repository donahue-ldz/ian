# 实现计划: Animation Expression Upgrade

## 对应规格

`docs/sdd/specs/0025-animation-expression-upgrade/spec.md`

## 实现步骤

1. 为 resource loader 写 schema / fallback 测试。
2. 扩展 `animations.json` 或 `expressions.json` 的解析模型。
3. 更新默认 `ian-alpaca` resource pack 的占位表现。
4. 在 AnimationPlayer 中接入 transition / expression fallback。
5. 做浏览器 smoke，确认主要状态无空白帧。

## 预计改动文件

- `apps/desktop/public/resources/pets/ian-alpaca/animations.json`
- `apps/desktop/public/resources/pets/ian-alpaca/expressions.json`
- `apps/desktop/src/resources/*`
- `apps/desktop/src/renderer/AnimationPlayer.tsx`
- `apps/desktop/src/renderer/AnimationPlayer.test.ts`
- `docs/sdd/specs/0025-animation-expression-upgrade/*`

## 接口 / 兼容性

Resource Pack schema 需要版本兼容。旧 pack 缺少 expression 时必须回退到当前动画。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

## 风险和回滚

资源 schema 变更可能影响现有 pack。回滚方式是保留 parser fallback，并把新增字段设为可选。
