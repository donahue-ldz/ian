# 实现计划: Developer Rhythm Privacy Audit

## 对应规格

`docs/sdd/specs/0049-developer-rhythm-privacy-audit/spec.md`

## 实现步骤

1. 编写 Developer Rhythm privacy audit checklist。
2. 为 sanitizer 敏感字段拒绝补测试。
3. 增加或记录 source scan 命令。
4. 检查 diagnostics / life event 只写 allowlist 字段。
5. 更新 verification 记录审计结果。

## 预计改动文件

- `docs/sdd/developer-rhythm-privacy-audit.md`
- `apps/desktop/src-tauri/src/security/sanitizer.rs`
- `apps/desktop/src-tauri/src/storage/repositories/life_event_repo.rs`
- `docs/sdd/specs/0049-developer-rhythm-privacy-audit/*`

## 接口 / 兼容性

不改产品协议。测试和文档只增强审计能力。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
rg -n "git diff|stdout|stderr|key_code|keypress|keydown|window title|document.title|clipboard|screenshot|OCR|read_to_string" apps/desktop/src apps/desktop/src-tauri/src -S
```

## 风险和回滚

source scan 可能有误报。回滚方式是记录误报原因，并把真正敏感路径纳入 allowlist / denylist。
