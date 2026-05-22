# 0255 · 实现计划

## 实现步骤

1. 确认 0190 拖动稳定性已收口。
2. 增加 drag_start 进入 carry 状态测试。
3. 增加拖动期间自动动作抑制测试。
4. 前端添加轻视觉状态，不影响原生拖动。
5. 桌面验收拖动和放下恢复。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml drag
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：视觉状态影响拖动命中区。需桌面验收。
- 回滚：关闭 carry 视觉，保留拖动互斥。
