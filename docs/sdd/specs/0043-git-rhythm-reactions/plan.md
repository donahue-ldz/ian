# 实现计划: Git Rhythm Reactions

## 对应规格

`docs/sdd/specs/0043-git-rhythm-reactions/spec.md`

## 实现步骤

1. 为 Git event policy 写失败测试，覆盖 clean、dirty、branch change、cooldown。
2. 接入 workspace 和 capability 授权检查。
3. 在 DeveloperRhythmPolicy 中增加 Git 低频反应规则。
4. 确认 action 输出仍是短句 / 动画 / movement，不包含 Git 细节。
5. 做 source scan 和 mock event smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/adapters/git_metadata_adapter.rs`
- `apps/desktop/src-tauri/src/domain/behavior/developer_rhythm_policy.rs`
- `apps/desktop/src-tauri/src/security/permission.rs`
- `docs/sdd/specs/0043-git-rhythm-reactions/*`

## 接口 / 兼容性

保持现有 Git event 兼容。新增字段必须可选且低敏。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
```

## 风险和回滚

Git 反应容易像通知器。回滚方式是只保留 commit seen 的低频开心反应，关闭 dirty 反应。
