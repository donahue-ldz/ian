# 实现计划: Simple Bond State

## Spec

`docs/sdd/specs/0011-simple-bond-state/spec.md`

## 状态

已实现，已验证。

## 概要

建立内部简单羁绊状态，让 Ian 的陪伴感有本地、克制、可解释的累积。

## 步骤

1. 写 BondEngine 单元测试。
2. 定义简单 BondStateView 和更新规则。
3. 接入 interaction events 或 bond repository。
4. 将 bond view 输入 DialogueContext。
5. 验证 UI 不暴露数值化成长。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/bond/*`
- `apps/desktop/src-tauri/src/storage/repositories/bond_repo.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/*`
- `docs/sdd/specs/0011-simple-bond-state/*`

## 接口与边界

Bond 是内部状态，不是前端游戏化系统。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml bond
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
```

## 风险

- 关系阶段如果暴露得太直接，会变成游戏化指标；0011 必须保持内部使用。

## 回滚说明

可固定 BondStateView 为 `New`，保留接口和 repository。
