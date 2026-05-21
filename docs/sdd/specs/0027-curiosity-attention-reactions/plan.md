# 实现计划: Curiosity Attention Reactions

## 对应规格

`docs/sdd/specs/0027-curiosity-attention-reactions/spec.md`

## 实现步骤

1. 为 pointer enter / leave 行为写 Rust 和前端失败测试。
2. 如需要，新增 Rust 协议事件并更新 TypeScript 绑定。
3. 在 BehaviorPolicy 中增加 attention cooldown。
4. 在 `IanStage` 只发送窗口内 pointer event，不直接改动画。
5. 做 source scan 和浏览器 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src/protocol/generated.ts`
- `apps/desktop/src/renderer/IanStage.tsx`
- `docs/sdd/specs/0027-curiosity-attention-reactions/*`

## 接口 / 兼容性

新增事件应向后兼容；旧前端只发送 `mouse.near` 时仍能工作。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
rg -n "global.*mouse|listen.*mouse|CGEvent|NSEvent" apps/desktop -S
```

## 风险和回滚

pointer 事件过多会造成动作频繁。回滚方式是保留 enter 反应，关闭 hover 持续反应。
