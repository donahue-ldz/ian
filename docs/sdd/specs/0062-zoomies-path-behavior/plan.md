# 实现计划: Zoomies Path Behavior

## 对应规格

`docs/sdd/specs/0062-zoomies-path-behavior/spec.md`

## 实现步骤

1. 设计 zoomies action 或 run-around reason 的协议变更。
2. 在 Rust Core 生成多 waypoint 路径并接入边界策略。
3. 在 React action executor 中支持连续 waypoint 执行。
4. 增加取消和结束回 idle / anchor 的处理。
5. 用 Browser / Tauri smoke 记录路径表现。

## 预计改动文件

- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/state/*`
- `apps/desktop/src/renderer/*`
- `docs/sdd/specs/0062-zoomies-path-behavior/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

多 waypoint 执行可能和窗口移动节奏冲突。回滚方式是保留 zoomies 触发，但把 waypoint 数量降到 3-4 个。
