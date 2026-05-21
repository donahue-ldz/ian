# 实现计划: Developer Rhythm Prep

## Spec

`docs/sdd/specs/0015-developer-rhythm-prep/spec.md`

## 状态

已实现，待验收。

## 概要

只建立 v0.2 Developer Rhythm 的 adapter 和权限骨架，不开放用户可见开发者功能。

## 步骤

1. 定义 developer source 和 sensitivity model。
2. 增加 adapter skeleton / no-op implementation。
3. 让 Security Gate 默认拒绝未授权 developer source。
4. 补测试验证默认关闭。
5. 在 decisions 中列出后续 v0.2 拆分。

## 预计文件改动

- `apps/desktop/src-tauri/src/adapters/*`
- `apps/desktop/src-tauri/src/protocol/event.rs`（如需要内部草案）
- `apps/desktop/src-tauri/src/security/*`
- `docs/sdd/specs/0015-developer-rhythm-prep/*`

## 接口与边界

0015 是准备工作，不实现 Git、build、test 的用户可见反应。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
rg -n "keyboard|terminal|clipboard|screen|git" apps/desktop/src-tauri/src
```

## 风险

- 过早暴露 developer 功能会偏离桌面生命主线；所有入口默认关闭。

## 回滚说明

删除 adapter skeleton 或保持 no-op，不影响 v0.1.x 功能。
