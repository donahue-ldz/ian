# 0090 · 验证记录

## 2026-05-21

### 自动化验证

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`：通过，12 个测试文件 / 84 个测试。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`：通过。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:build`：通过，Vite production build 成功。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，119 个 Rust 测试。

### 桌面 / 预览验收

- 浏览器预览打开设置后确认可以看到宠物选择器和内置资源包预览卡，当前资源包有 selected / pressed 状态。
- 真实 Tauri shell smoke：`npm run tauri --workspace @ian/desktop -- dev --no-dev-server-wait --config '{"build":{"beforeDevCommand":"","devUrl":"http://localhost:1420"}}'` 编译成功并启动 `target/debug/ian_desktop`，运行期间终端无 panic；本机 1420 端口已有预先存在的 Vite 服务，因此跳过重复启动前端服务。

### 剩余风险

- 本次没有通过人工视觉逐项确认真实桌面中的外观切换动画，只完成真实壳启动 smoke 和浏览器预览交互确认。
