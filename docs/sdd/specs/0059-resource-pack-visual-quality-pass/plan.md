# 实现计划: Resource Pack Visual Quality Pass

## 对应规格

`docs/sdd/specs/0059-resource-pack-visual-quality-pass/spec.md`

## 实现步骤

1. 盘点默认资源包 manifest 和关键状态资源。
2. 建立资源质量检查清单和 fallback 规则。
3. 调整尺寸、锚点、阴影或透明边界问题。
4. 补充资源加载测试和状态截图。
5. 记录资源包质量验收结果。

## 预计改动文件

- `apps/desktop/resources/*`
- `apps/desktop/src/renderer/*Resource*`
- `apps/desktop/src/renderer/*Animation*`
- `docs/sdd/specs/0059-resource-pack-visual-quality-pass/*`

## 接口 / 兼容性

manifest 新字段必须可选。旧资源包缺少字段时使用默认锚点、尺寸和 fallback。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

资源调整可能改变已有截图基线。回滚方式是保留新校验清单，逐项恢复视觉资源改动。
