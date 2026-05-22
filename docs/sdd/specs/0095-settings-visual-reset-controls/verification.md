# 0095 · 验证记录

## 2026-05-21

### 自动化验证

- RED：新增设置面板重置入口测试后失败，缺少“恢复默认外观”和“重置桌面位置”按钮。
- GREEN：定向前端测试通过；全量 `desktop:test` 84/84、`desktop:typecheck`、`desktop:build` 通过。
- Rust：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 119/119 通过，既有窗口默认位置 / 越界夹取测试继续通过。

### 桌面 / 预览验收

- 浏览器预览确认设置面板可打开；高级区重置按钮由静态渲染测试确认。
- 真实 Tauri shell smoke 编译并启动成功；`reset_window_position` 命令未在真实桌面中点击执行。

### 剩余风险

- 需要后续人工在真实桌面点击“重置位置”，确认窗口立即回到当前显示器右下安全区域。
