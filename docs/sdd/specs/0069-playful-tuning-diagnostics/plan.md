# 实现计划: Playful Tuning Diagnostics

## 对应规格

`docs/sdd/specs/0069-playful-tuning-diagnostics/spec.md`

## 实现步骤

1. 复用或扩展 0039 本地诊断结构。
2. 为 playful trigger、cooldown、safety gate 输出 reason。
3. 增加隐私字段过滤测试。
4. 提供开发期查看入口或日志格式。
5. 记录调参和 smoke 结果。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/diagnostics*`
- `docs/sdd/specs/0069-playful-tuning-diagnostics/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml diagnostics
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
```

## 风险和回滚

诊断过多会增加噪音。回滚方式是默认只记录最近 N 条 playful 摘要，并放在开发诊断入口。
