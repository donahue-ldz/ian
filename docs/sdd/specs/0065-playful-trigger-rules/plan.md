# 实现计划: Playful Trigger Rules

## 对应规格

`docs/sdd/specs/0065-playful-trigger-rules/spec.md`

## 实现步骤

1. 定义 playful trigger reason 和规则表。
2. 将连续互动、idle tick、拖拽结束接入候选触发。
3. 增加冷却、日内上限和取消条件。
4. 接入本地诊断摘要。
5. 补充 policy tests 和 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/core/scheduler*`
- `apps/desktop/src-tauri/src/storage/*`
- `docs/sdd/specs/0065-playful-trigger-rules/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml scheduler
```

## 风险和回滚

自发触发太弱会看不到效果，太强会打扰。回滚方式是先只保留用户触发，再逐步放开自发触发。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
