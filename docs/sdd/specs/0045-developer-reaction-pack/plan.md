# 实现计划: Developer Reaction Pack

## 对应规格

`docs/sdd/specs/0045-developer-reaction-pack/spec.md`

## 实现步骤

1. 为 reaction kind 和 cooldown 写 Rust 失败测试。
2. 扩展 DeveloperRhythmPolicy 的 reaction selection。
3. 将短句统一经过 DialoguePolicy。
4. 接入 quiet hours / snooze 降级接口。
5. 做 mock event burst smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/developer_rhythm_policy.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_policy.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `docs/sdd/specs/0045-developer-reaction-pack/*`

## 接口 / 兼容性

不新增外部协议。新增 reaction kind 仅内部使用。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
```

## 风险和回滚

反应过多会打扰用户。回滚方式是只保留成功和连续失败两类低频反应。
