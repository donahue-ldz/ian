# 0256 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。长距离放下缺少 drop settle moment diagnostic，短距离放下与既有 settle 行为边界未区分。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖长距离 `drop_settle`，并保留既有短距离 drop 测试。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。

## 验收标准结果

- [x] 放下后仍执行位置移动、blush puff、happy 和短 speech。
- [x] 长距离放下额外记录 `drop_settle` moment diagnostic。
- [x] 短距离放下保持既有动作序列，不破坏旧测试。
- [x] 与 drag carry 分离，避免拖放期间重复 surprise。

## 剩余风险

- 边界 clamp 依赖既有 movement/window 测试覆盖；本批未新增真实多显示器拖放人工验收。
