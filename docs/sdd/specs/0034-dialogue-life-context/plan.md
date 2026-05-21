# 实现计划: Dialogue Life Context

## 对应规格

`docs/sdd/specs/0034-dialogue-life-context/spec.md`

## 实现步骤

1. 为 DialogueContext 上下文差异写失败测试。
2. 扩展 DialogueContext 类型和 builder。
3. 更新 Demo Dialogue Provider 的短句选择逻辑。
4. 确认 DialoguePolicy 对上下文输出仍统一生效。
5. 做 source scan，确认 provider 不接收敏感上下文。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_policy.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/demo.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `docs/sdd/specs/0034-dialogue-life-context/*`

## 接口 / 兼容性

Demo Dialogue 必须继续在无网络、无 API key 时可用。Context 字段缺失时使用默认值。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml app
```

## 风险和回滚

上下文过多会让短句像解释系统状态。回滚方式是只保留 mood、bond、day phase 三类上下文。
