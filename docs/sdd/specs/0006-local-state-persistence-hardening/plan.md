# 实现计划: Local State Persistence Hardening

## Spec

`docs/sdd/specs/0006-local-state-persistence-hardening/spec.md`

## 状态

已实现，已验证。

## 概要

把 P0 已有本地存储从 skeleton 收紧为可验证的 v0.1.x 基线：配置恢复、位置保存、migration 幂等和最小互动事件 repository。

## 步骤

1. 补 Rust storage 测试，先覆盖 config 恢复和 migration 幂等。
2. 收紧 `storage/config.rs` 和 migration 逻辑。
3. 增加基础互动事件 repository 写入路径。
4. 确认 `save_window_position` 只通过 Rust command 修改状态。
5. 更新 verification 记录。

## 预计文件改动

- `apps/desktop/src-tauri/src/storage/*`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `docs/sdd/specs/0006-local-state-persistence-hardening/*`

## 接口与边界

不新增用户可见功能。React 继续只发送事件，Rust Core 负责持久化和状态同步。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
```

## 风险

- 桌面窗口位置恢复涉及平台行为，自动化可能不足；需要补一次真实 Tauri smoke。

## 回滚说明

保留旧 config schema 兼容路径；如 migration 异常，可回滚 repository 新逻辑并保留默认配置恢复。
