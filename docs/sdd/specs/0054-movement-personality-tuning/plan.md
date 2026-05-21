# 实现计划: Movement Personality Tuning

## 对应规格

`docs/sdd/specs/0054-movement-personality-tuning/spec.md`

## 实现步骤

1. 梳理当前自主移动、run-around、idle action 的优先级。
2. 定义 movement profile 默认参数和互斥状态。
3. 将移动频率、距离、停顿和曲线接入 Core policy。
4. 调整 React 动画插值和边界处理。
5. 增加 5 分钟 idle smoke 记录。

## 预计改动文件

- `apps/desktop/src-tauri/src/behavior/*`
- `apps/desktop/src-tauri/src/state/*`
- `apps/desktop/src/renderer/*`
- `docs/sdd/specs/0054-movement-personality-tuning/*`

## 接口 / 兼容性

已有移动 action 保持兼容。新增 profile 字段缺失时使用 normal 默认参数。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

移动过少会削弱生命感。回滚方式是提高微动作频率，而不是提高大幅移动频率。
