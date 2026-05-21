# 实现计划: Run Around Path Behavior

## 对应规格

`docs/sdd/specs/0022-run-around-path-behavior/spec.md`

## 实现步骤

1. 为 Rust `MouseDoubleClick` 写路径动作测试，先确认当前只返回单一 run action 的不足。
2. 在 `BehaviorPolicy` 增加短路径生成规则，使用当前位置和安全偏移。
3. 在 `BehaviorEngine` 把双击转换为 run 动画、movement waypoint、结束状态的动作序列。
4. 在前端执行层支持同一事件返回的连续 movement action。
5. 补充双击 smoke，记录真实位移和回 idle。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/state/ianActions.test.ts`
- `docs/sdd/specs/0022-run-around-path-behavior/*`

## 接口 / 兼容性

优先不新增协议类型。若必须新增动作序列字段，需同步 Rust 绑定和 TypeScript 生成文件。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

路径执行可能与拖拽产生冲突。回滚方式是保留 `BehaviorRunAround` 动画行为，关闭 waypoint 输出。
