# 实现计划: Touch And Affection Language

## 对应规格

`docs/sdd/specs/0053-touch-and-affection-language/spec.md`

## 实现步骤

1. 梳理已有点击、双击和拖拽事件路径。
2. 在 Core 中定义触摸反应矩阵和冷却策略。
3. 调整 React 事件发送，确保 drag start / end 清晰。
4. 接入动作、表情和短句资源。
5. 补充行为测试、前端测试和 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/behavior/*`
- `apps/desktop/src-tauri/src/protocol/*`
- `apps/desktop/src/renderer/*`
- `apps/desktop/resources/*`
- `docs/sdd/specs/0053-touch-and-affection-language/*`

## 接口 / 兼容性

保留已有 click / double-click 行为语义。新增 rapid click 时旧客户端可以退化为普通 click。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

连点降频过强会让用户觉得没反馈。回滚方式是保留轻微动作反馈，只限制气泡和高强度动作。
