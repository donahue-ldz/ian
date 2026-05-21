# 实现计划: Playful Expressiveness Acceptance Suite

## 对应规格

`docs/sdd/specs/0070-playful-expressiveness-acceptance-suite/spec.md`

## 实现步骤

1. 汇总 0061-0069 验收标准，形成 playful checklist。
2. 增加固定 seed 自动化测试或 fixture。
3. 建立 zoomies、cute reaction、safety、visual smoke 场景。
4. 执行 Rust、前端和 Browser / Tauri 验收。
5. 把结果写入 `verification.md`。

## 预计改动文件

- `docs/sdd/specs/0070-playful-expressiveness-acceptance-suite/*`
- `docs/sdd/checklists/*`
- `apps/desktop/*` 测试或 smoke 入口

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:dev
```

## 风险和回滚

体验验收容易主观。回滚方式是把每条判断绑定到明确场景、失败条件、截图或日志 reason。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
