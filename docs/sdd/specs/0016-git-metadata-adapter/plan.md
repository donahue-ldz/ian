# 实现计划: Git Metadata Adapter

## Spec

`docs/sdd/specs/0016-git-metadata-adapter/spec.md`

## 状态

已实现，待验收。

## 概要

实现默认关闭的 Git 低敏元数据 adapter，为 Developer Rhythm 提供第一批安全事件源。

## 步骤

1. 定义 Git metadata 事件和权限项。
2. 编写 Security Gate 未授权拒绝测试。
3. 实现只读 Git metadata collector。
4. 将 metadata 转为 `IanEvent`。
5. 增加敏感字段扫描和 smoke 验证。

## 预计文件改动

- `apps/desktop/src-tauri/src/adapters/*`
- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src-tauri/src/storage/config.rs`
- `docs/sdd/specs/0016-git-metadata-adapter/*`

## 接口与边界

Git adapter 不直接控制动画，不执行 git 写操作，不读取代码内容。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
rg -n "git diff|git show|read_to_string|Command::new" apps/desktop/src-tauri/src
```

## 风险

- Git 信息可能包含项目敏感信息；payload 必须只保留最小元数据。

## 回滚说明

禁用 Git permission 或移除 adapter 注册，不影响 v0.1.x 功能。
