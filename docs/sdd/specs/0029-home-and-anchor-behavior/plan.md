# 实现计划: Home And Anchor Behavior

## 对应规格

`docs/sdd/specs/0029-home-and-anchor-behavior/spec.md`

## 实现步骤

1. 为 config anchor round-trip 写失败测试。
2. 增加 anchor 字段、默认值和 migration 兼容。
3. 在拖拽结束处理链路中保存 anchor。
4. 在 run-around / roam 策略中加入回 anchor 候选。
5. 做 Tauri smoke：拖动、重启、回 anchor 附近。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/core/creature_state.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/lib/tauriBridge.ts`
- `docs/sdd/specs/0029-home-and-anchor-behavior/*`

## 接口 / 兼容性

旧配置没有 anchor 时，使用当前 position 或右下角默认位置作为 fallback。损坏配置继续恢复默认值。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
```

## 风险和回滚

如果 anchor 与 position 语义冲突，先把 anchor 作为 position 的派生值，不单独暴露设置。
