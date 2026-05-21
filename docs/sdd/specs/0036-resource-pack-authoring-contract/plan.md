# 实现计划: Resource Pack Authoring Contract

## 对应规格

`docs/sdd/specs/0036-resource-pack-authoring-contract/spec.md`

## 实现步骤

1. 为有效和无效 resource pack fixture 写失败测试。
2. 补充 schema 类型和校验函数。
3. 更新默认资源包文档和示例。
4. 改善 loader 错误信息和 fallback。
5. 运行前端测试并记录结果。

## 预计改动文件

- `apps/desktop/src/resources/*`
- `apps/desktop/src/resources/resourceLoader.test.ts`
- `apps/desktop/public/resources/pets/ian-alpaca/*`
- `docs/resources/resource-pack-authoring.md`
- `docs/sdd/specs/0036-resource-pack-authoring-contract/*`

## 接口 / 兼容性

旧资源包缺少新增字段时应 fallback，不应阻塞启动。schema 版本变化需要记录在文档中。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

校验过严会阻塞占位资源。回滚方式是把新增字段设为 optional，并通过 warning 而不是 error 提示。
