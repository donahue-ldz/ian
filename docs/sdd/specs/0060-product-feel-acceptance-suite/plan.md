# 实现计划: Product Feel Acceptance Suite

## 对应规格

`docs/sdd/specs/0060-product-feel-acceptance-suite/spec.md`

## 实现步骤

1. 汇总 0051-0059 的验收标准，形成 Product Feel checklist。
2. 补充自动化测试入口或 smoke 脚本说明。
3. 建立截图记录规范：默认态、气泡、触摸、设置、隐私、idle。
4. 执行 Rust、前端和 Tauri / Browser smoke。
5. 将结果写入本 SDD 的 `verification.md`。

## 预计改动文件

- `docs/sdd/specs/0060-product-feel-acceptance-suite/*`
- `docs/sdd/checklists/*`
- `apps/desktop/*` 测试或 smoke 入口

## 接口 / 兼容性

不改变产品接口。新增测试和文档应可重复运行。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:dev
```

## 风险和回滚

体验验收可能变成主观描述。回滚方式是把每条主观判断绑定到可观察场景、截图或明确失败条件。
