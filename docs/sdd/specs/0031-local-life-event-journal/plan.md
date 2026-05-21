# 实现计划: Local Life Event Journal

## 对应规格

`docs/sdd/specs/0031-local-life-event-journal/spec.md`

## 实现步骤

1. 为 life event repository 写失败测试，覆盖追加和按时间读取。
2. 增加 migration 和 repository 结构。
3. 在 Rust Core 行为执行后记录低敏 life event。
4. 增加 payload sanitizer 或 allowlist，阻止正文和外部上下文入库。
5. 补充 source scan 和本地 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/mod.rs`
- `apps/desktop/src-tauri/src/storage/migrations.rs`
- `apps/desktop/src-tauri/src/storage/life_event_repository.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `docs/sdd/specs/0031-local-life-event-journal/*`

## 接口 / 兼容性

新增 migration 必须幂等。旧数据库升级失败时应回退到不写日志但应用可运行。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml app
```

## 风险和回滚

日志膨胀可能影响本地存储。回滚方式是保留 schema，暂时关闭写入或限制保留天数。
