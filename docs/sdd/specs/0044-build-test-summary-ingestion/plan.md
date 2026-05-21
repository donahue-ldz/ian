# 实现计划: Build Test Summary Ingestion

## 对应规格

`docs/sdd/specs/0044-build-test-summary-ingestion/spec.md`

## 实现步骤

1. 为 summary ingestion 写 command / adapter 失败测试。
2. 实现本地 ingestion 入口，转换为 `developer.build_test_summary`。
3. 加入权限、workspace、payload size 和敏感字段检查。
4. 接入 DeveloperRhythmPolicy。
5. 做 source scan 和 mock summary smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/adapters/build_test_adapter.rs`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src-tauri/src/security/sanitizer.rs`
- `apps/desktop/src-tauri/src/domain/behavior/developer_rhythm_policy.rs`
- `docs/sdd/specs/0044-build-test-summary-ingestion/*`

## 接口 / 兼容性

现有 event 结构保持兼容。新 command 只接收结构化摘要。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
```

## 风险和回滚

外部工具可能想传长日志。回滚方式是明确拒绝长日志，只接受 error_kind。
