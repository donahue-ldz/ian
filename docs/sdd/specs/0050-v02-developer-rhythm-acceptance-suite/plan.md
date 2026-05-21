# 实现计划: v0.2 Developer Rhythm Acceptance Suite

## 对应规格

`docs/sdd/specs/0050-v02-developer-rhythm-acceptance-suite/spec.md`

## 实现步骤

1. 汇总 0041-0049 的验收标准，形成 v0.2 checklist。
2. 补齐 mock event smoke helper。
3. 整理 source scan 和 privacy audit 命令。
4. 运行完整 Rust / frontend / build 验收命令。
5. 在 verification 中记录通过项、失败项和剩余风险。

## 预计改动文件

- `docs/sdd/v02-developer-rhythm-acceptance.md`
- `docs/sdd/specs/0050-v02-developer-rhythm-acceptance-suite/*`
- `package.json`
- `apps/desktop/package.json`
- 可能新增 `apps/desktop/src-tauri/src/**/tests` 或 mock helper

## 接口 / 兼容性

不改产品协议。新增 script 只追加，不替换现有命令。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
rg -n "git diff|stdout|stderr|key_code|keypress|keydown|window title|document.title|clipboard|screenshot|OCR|read_to_string" apps/desktop/src apps/desktop/src-tauri/src -S
```

## 风险和回滚

v0.2 验收可能被真实环境依赖干扰。回滚方式是使用 mock event smoke 作为主验收，真实 Git/workspace 作为人工补充。
