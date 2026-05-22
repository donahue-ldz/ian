# 0266 · 验证记录

## 2026-05-22

### 已执行检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过，覆盖 drag carry、drop settle、拖动期间自主行为抑制和旧 drop 序列兼容。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过。16 个测试文件 / 117 个测试通过。
- 真实 Tauri 桌面壳启动 smoke
  - 结果：通过。

## 验收标准结果

- [x] 拖动开始产出 carry diagnostic、短缩放和抱起反馈。
- [x] 放下后保留位置移动、短 effect、happy 和短文案。
- [x] 长距离放下可记录 `drop_settle`。
- [x] 拖动 / 输入中其他 Moment 会被 context gate 抑制。
- [x] 位置持久化路径未改动，兼容既有 storage/config。

## 剩余风险

- 未执行真实拖动偏移、三屏释放和重启恢复人工验收。
