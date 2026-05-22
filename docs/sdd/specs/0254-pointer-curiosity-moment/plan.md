# 0254 · 实现计划

## 实现步骤

1. 盘点已有 pointer chase 和 mouse near/leave 逻辑。
2. 增加好奇 moment 触发条件和冷却测试。
3. 限制追逐目标、距离、时长和打断条件。
4. 桌面验收真实鼠标靠近、离开和短追。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/state/useIanActions.ts`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：过度追鼠标造成打扰。通过冷却和概率降低。
- 回滚：关闭好奇 moment，保留现有 pointer chase。
