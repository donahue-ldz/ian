# 0155 · 验证记录

## 2026-05-21

### 本轮覆盖

- 设置面板：Demo / 自带模型模式、无 key 禁用 BYOM、提醒间隔、勿扰、权限中心、隐私 onboarding 中文文案。
- Rust Core / Storage / Security：BYOM 无 key 拒绝启用、secret store 保存/读取/删除/脱敏、本地 config 不写 API key、提醒设置和勿扰 round-trip、reminder_records repository、adapter permission registry、路径/代码 payload 拒绝。
- 未来能力 skeleton：继续保持默认关闭、摘要事件和低敏 payload，未新增默认联网或默认高敏能力。

### 已执行命令

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run test --workspace @ian/desktop -- src/renderer/SettingsPanel.view.test.tsx`
  - 结果：通过，1 个测试文件，10 个测试。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过，126 个 Rust 测试。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`
  - 结果：通过，12 个测试文件，86 个测试。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:build`
  - 结果：通过，Vite production build 完成。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。

### 桌面验收

- 启动 dev server：`PATH=/opt/homebrew/bin:$PATH npm run dev --workspace @ian/desktop -- --host 127.0.0.1 --port 5174`
  - 结果：Vite 在 `http://127.0.0.1:5174/` ready。
- 启动真实 Tauri 桌面壳：`PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --no-dev-server-wait --config '{"build":{"beforeDevCommand":"","devUrl":"http://127.0.0.1:5174"}}'`
  - 结果：Rust dev build 完成并运行 `target/debug/ian_desktop`，启动期无错误；本轮启动的 Tauri 和 dev server 已停止。

### 安全 / 隐私验证

- 默认关闭：BYOM 无本地 key 时运行时拒绝启用；高敏 adapter registry 默认关闭；未知 adapter 默认拒绝。
- 脱敏：secret store 状态只返回截断标记，例如 `configured:sk-t...cret`。
- 不保存敏感内容：config 测试确认不包含 `sk-` 或 `api_key`；reminder records 不写用户 text；sanitizer 拒绝文件路径形态 workspace_id 和代码片段 error_kind。

### 剩余风险

- 0112/0113/0131-0139/0140-0149 中属于 future skeleton 的完整产品验收仍需后续 SDD 细化；本轮只锁定默认关闭、权限、存储和安全边界。
- 桌面 smoke 覆盖启动期和真实壳运行，不等同于完整人工交互验收。
