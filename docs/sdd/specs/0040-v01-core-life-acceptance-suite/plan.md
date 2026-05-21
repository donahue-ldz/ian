# 实现计划: v0.1 Core Life Acceptance Suite

## 对应规格

`docs/sdd/specs/0040-v01-core-life-acceptance-suite/spec.md`

## 实现步骤

1. 汇总 0021-0039 的验收标准，写成 core life checklist。
2. 补齐缺失的自动化测试入口或 npm script。
3. 编写 Browser / Tauri smoke 步骤和期望结果。
4. 运行完整验收命令并记录结果。
5. 明确剩余风险和下一阶段建议。

## 预计改动文件

- `docs/sdd/specs/0040-v01-core-life-acceptance-suite/*`
- `docs/sdd/v01-core-life-acceptance.md`
- `package.json`
- `apps/desktop/package.json`
- 可能新增 `apps/desktop/src/**/__tests__` 或 smoke helper

## 接口 / 兼容性

不改产品协议。若新增 script，保持现有命令不变，只追加。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

## 风险和回滚

端到端 smoke 可能受桌面环境限制。回滚方式是保留自动化单元/集成检查，并把桌面视觉验收标记为人工步骤。
