# 实现计划: Life Rhythm Composition

## 对应规格

`docs/sdd/specs/0030-life-rhythm-composition/spec.md`

## 实现步骤

1. 梳理 0021-0029 已有行为入口和 action 输出。
2. 为组合优先级写 Rust integration 失败测试。
3. 新增或收敛 Life Rhythm Policy，统一 action 候选选择。
4. 移除重复的局部优先级判断，保留模块边界。
5. 做长时 smoke，确认没有动作冲突和 console error。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/domain/behavior/life_rhythm_policy.rs`
- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src/state/ianActions.test.ts`
- `docs/sdd/specs/0030-life-rhythm-composition/*`

## 接口 / 兼容性

优先做内部策略收束，不新增外部协议。若协议变化不可避免，必须更新 bindings 和 TypeScript 类型。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml scheduler
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

组合策略可能过早抽象。回滚方式是保留测试，先用 BehaviorPolicy 内部函数表达优先级，等规则稳定后再独立成模块。
