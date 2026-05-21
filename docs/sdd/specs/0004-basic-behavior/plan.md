# 实现计划: Basic Behavior

## 状态

已实现，已验证。

## 概要

在 P0 安全边界内增加低频 time tick 和窗口内 mouse near，让 Rust Core 输出更像生命的基础行为。

## 步骤

1. 添加 Rust RED 测试。
   - `mouse.near` 返回 happy 轻反应。
   - `time.tick` 可在固定时间输入下返回可预测动作。

2. 更新协议。
   - 修改 Rust `IanEvent`。
   - 同步 `src/protocol/generated.ts` placeholder。

3. 实现 BehaviorPolicy/BehaviorEngine 决策。
   - 不引入随机不可测逻辑；使用 `now_ms` 分段制造可测试的微动作。

4. 前端发送事件。
   - `IanStage` pointer enter 发送 `mouse.near`。
   - `useIanActions` 或 `App` 低频发送 `time.tick`。

5. 验证并记录。
   - Rust / frontend tests。
   - Playwright 验证 pointer enter 和 tick 行为。

## 预计文件改动

- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src/protocol/generated.ts`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/App.tsx`
- `docs/sdd/specs/0004-basic-behavior/verification.md`

## 风险与回滚

风险是 idle 行为过于频繁打扰用户。P0 使用低频 tick 和短动画，必要时可回滚前端 tick 发送。

