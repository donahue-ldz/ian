# 0262 · 验证记录

## 2026-05-22

### 执行结果

- 当前可执行队列扫描：发现 `0262-0269` 共 8 个 SDD 的 `verification.md` 处于未完成状态。
- 真实 Tauri 桌面壳：`PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`
  - 结果：通过。桌面壳编译并启动到 `target/debug/ian_desktop`；随后关闭并确认无残留进程。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过。16 个测试文件 / 117 个测试通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`
  - 结果：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过。

### Moment 基线观察

- 找回：Rust Core 可通过 `system.shortcut_triggered` 和诊断事件产出找回 beacon、短气泡、可见区域移动。
- 鼠标好奇：正式路径保留冷却；诊断路径可稳定触发 `pointer_curiosity`。
- 拖动抱起：拖动开始仍产出 carry diagnostic、短缩放和抱起文案。
- 放下安顿：短距离保留既有四步序列；长距离和诊断路径可产出 drop settle。
- Idle 惊喜：正式路径保持低频；诊断路径可稳定触发。
- 记忆回响：诊断路径只使用内置低敏短句，不展示原文。

## 剩余风险

- 本轮完成真实 Tauri 启动 smoke 和确定性诊断覆盖；未进行长时间人工桌面录屏观察。
- 多屏、真实鼠标拖动偏移和重启后位置恢复仍建议按 `docs/sdd/moment-desktop-acceptance.md` 做人工补验。
