# 决策记录: P0 本地数字生命验证版

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 将 P0 视为产品验证，同时建立 v0.1 Architecture Baseline。 | Ian 需要快速验证产品生命感，但不能形成一次性架构。 | 第一版实现会包含架构 skeleton，但用户可见功能保持克制。 |
| 2026-05-21 | BYOM 保持可选，不作为 P0 核心卖点。 | P0 应验证桌面生命存在感，而不是变成聊天产品。 | Demo Dialogue 必须实现；BYOM 可延后到 v0.1.x，同时先准备 provider boundary。 |
| 2026-05-21 | 未来能力只做 skeleton，不做用户可见功能。 | 防止 P0 吸收 v0.2 / v0.3 范围。 | Git、keyboard rhythm、Feishu、Pet Visit、plugin、Mood/Bond、long-term memory 不进入 P0 行为。 |
| 2026-05-21 | P0 视觉使用 Resource Pack manifest + `sprite.svg` + CSS placeholder creature。 | 当前没有正式 sprite sheet，美术不应阻塞 P0 架构和交互闭环。 | `public/resources/pets/ian-alpaca` 结构完整；后续可替换为真正 `sprite.png` / sprite sheet，不影响协议和前端边界。 |
| 2026-05-21 | `src/protocol/generated.ts` 暂时作为已标注 generated placeholder 提交。 | 当前本机缺少 Rust/Cargo，无法运行 `ts-rs` 生成链路。 | Rust 仍是协议源头；安装 Rust 后需要补运行类型生成并替换 placeholder。 |
| 2026-05-21 | 本轮不自动安装 Rust 工具链。 | 安装 Rust/rustup 会修改用户全局开发环境，应由用户明确执行或确认。 | Tauri dev、Rust check、桌面 smoke test 暂时记录为环境阻塞。 |
| 2026-05-21 | 用户确认后安装 Rust stable toolchain 并继续 Tauri 验证。 | `cargo check` 和 Tauri dev 是 0001 的必要验证。 | Rust/Cargo 已可用；Tauri dev 已能启动 `ian_desktop`。 |
| 2026-05-21 | 为 Tauri dev 添加有效 `src-tauri/icons/icon.png` 并启用 `app.macOSPrivateApi=true`。 | Tauri 运行期需要有效图标；macOS 透明窗口需要 private API 配置。 | 修复 Tauri setup panic，并支持 P0 透明窗口目标。 |
| 2026-05-21 | 人工验收后将窗口从临时居中 360x360 调回右下角 260x260。 | 临时居中用于解决“找不到窗口”的验收问题；收尾需要回到 P0 常驻桌面形态。 | 产品形态仍在右下角，保留淡色轮廓以降低不可见风险。 |

## 范围变化

暂无超出 0001 的产品范围变化。

## 延后工作

- OpenAI-compatible BYOM UI
- 主动喝水 / 休息提醒
- 完整 Mood System
- 完整 Bond System
- 长期记忆
- Git / build / test 集成
- keyboard rhythm
- active window awareness
- Feishu Relay
- Pet Visit
- plugin system
