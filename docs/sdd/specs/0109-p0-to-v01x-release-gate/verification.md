# 0109 · 验证记录

## 2026-05-21

### 自动化验证

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`：通过，12 个测试文件 / 84 个测试。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`：通过。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:build`：通过，49 modules transformed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，119 个测试。

### P0 DoD 状态

- [x] Tauri 桌面壳可编译并启动 smoke。
- [x] 透明 always-on-top 相关配置仍在 `tauri.conf.json`。
- [x] Ian 从内置 resource pack 渲染，资源包合同测试覆盖 7 个语义动画。
- [x] 点击 / 双击 / 拖动 / 气泡 / Demo Dialogue / 位置持久化均有自动化测试覆盖。
- [x] Rust Core 继续通过 `IanEvent` / `IanAction` 驱动行为，React 只执行 action。
- [ ] 真实桌面人工验收仍需补：点击、拖动、资源切换、气泡位置、移动路径的视觉观察。

### 桌面 / 预览验收

- 浏览器预览：设置面板可打开，资源包预览卡和当前选中状态可见。
- 真实 Tauri shell：首次 `tauri dev` 因 1420 端口已有服务失败；随后使用 `--no-dev-server-wait` 复用现有 `http://localhost:1420` 服务，Rust 编译成功并启动 `target/debug/ian_desktop`，终端无 panic。已停止本轮启动的 Tauri 进程，未终止进入本轮前已存在的 1420 服务。

### v0.1.x 后续建议

- 优先补一次真实桌面人工验收并截图/录屏记录。
- 下一批优先细化提醒确认 UI、桌面资源切换人工验收、长期 idle 体感校准。
- 继续避免把 Git、键盘节奏、Feishu、Pet Visit、插件等 v0.2+ 能力做成 P0 用户可见功能。
