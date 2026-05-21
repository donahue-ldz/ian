# 验证记录: Git Metadata Adapter

## Spec

`docs/sdd/specs/0016-git-metadata-adapter/spec.md`

## 状态

已验证，待用户验收。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Adapter tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters` | 通过 | Git metadata adapter 默认关闭和启用事件转换已覆盖。 |
| Security tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security` | 通过 | 未授权拒绝已覆盖。 |
| Source scan | `rg -n "git diff|git show|Command::new" apps/desktop/src-tauri/src apps/desktop/src` | 通过 | 无匹配；未实现真实 git 命令读取。 |

## 验收标准结果

- [x] Git adapter 默认关闭，未授权时 Security Gate 拒绝事件。
- [x] 启用后只能读取 branch、dirty 状态和短 commit hash 等低敏元数据。
- [x] Adapter 不执行 git 写操作。
- [x] Git 元数据通过 `IanEvent` 进入 Rust Core。
- [x] 测试覆盖未授权拒绝、授权后事件转换、敏感字段不进入 payload。

## 失败或缺口

本批只实现 adapter skeleton 和事件边界，不执行真实 Git 命令。

## 后续

实现后补充实际验证结果。
