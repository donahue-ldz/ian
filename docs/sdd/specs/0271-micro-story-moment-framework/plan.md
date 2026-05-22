# 0271 · 实施计划

## 步骤

1. 梳理当前 `IanAction` 是否已能表达 sequence / delay / movement / speech。
2. 在 Rust Core 新增 Micro Story 数据结构和 beat builder。
3. 将 Story Framework 接入 Moment Orchestrator。
4. 选择一个低风险 Moment 改写为 story，例如 find Ian entrance 或 pointer curiosity。
5. 增加打断、降级和 action 顺序测试。
6. 在真实 Tauri 桌面观察 story 是否按节奏播放。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_story.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src/protocol/generated.ts`
- `apps/desktop/src/state/ianActions.ts`
- `docs/sdd/specs/0271-micro-story-moment-framework/verification.md`

## 受影响接口或模块

- Moment Orchestrator
- IanAction sequence 执行
- React action executor

## 兼容性说明

如果新增 action 字段，必须保证旧 action 仍可执行。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面 story Moment 观察。

## 风险和回滚

- 风险：sequence 机制导致动作排队过长。
- 缓解：所有 story 设定最大时长和中断条件。
- 回滚：Moment 继续返回普通 action 序列，不使用 Story Framework。
