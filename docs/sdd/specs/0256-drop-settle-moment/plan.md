# 0256 · 实现计划

## 实现步骤

1. 增加 drag distance 或等价低敏状态记录。
2. 为 drag_end 后 settle moment 写测试。
3. 增加短距离 / 长距离 / 冷却分支。
4. 桌面验收拖动放下和位置恢复。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/momentary_life_state.rs`
- `apps/desktop/src/state/useIanActions.ts`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml drag
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：频繁拖动导致气泡过多。通过冷却控制。
- 回滚：保留位置保存，关闭 settle moment。
