# 验证记录: P0 本地数字生命验证版

## Spec

`docs/sdd/specs/0001-p0-local-creature-proof/spec.md`

## 状态

已执行到 0001 当前可验证边界。前端、Rust check、Tauri dev smoke、存储初始化验证均已执行。2026-05-21 验收复查发现 click / double-click 和位置恢复两个 P1 缺口，本记录已补充修复后的回归验证。

## 验证摘要

已完成 `apps/desktop` 的 Tauri v2 + React + TypeScript 骨架、Resource Pack、前端 action 执行层、Rust Core/protocol/storage/security skeleton。

可执行验证中，前端测试、typecheck、build、Rust check 均通过。Tauri dev 可以编译并启动 `ian_desktop`，系统进程列表能看到 `ian_desktop`，并初始化 `~/.ian/config.toml` 与 `~/.ian/ian.db`。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| SDD packet 存在 | `test -f docs/sdd/specs/0001-p0-local-creature-proof/spec.md && test -f docs/sdd/specs/0001-p0-local-creature-proof/plan.md && test -f docs/sdd/specs/0001-p0-local-creature-proof/decisions.md && test -f docs/sdd/specs/0001-p0-local-creature-proof/verification.md` | 通过，exit 0 | packet 四个文件存在。 |
| Dependency install check | `npm install --loglevel=verbose` | 通过，exit 0 | 内部 registry audit endpoint 返回 405，但 npm install 最终成功；生成 `package-lock.json`。 |
| Frontend reducer TDD RED | `npm run desktop:test` | 先失败，exit 1 | 两个测试分别因 bubble 未打开、run 动画未播放而失败，符合预期。 |
| Frontend reducer TDD GREEN | `npm run desktop:test` | 通过，exit 0 | 1 个 test file，2 个 tests passing。 |
| Frontend typecheck | `npm run desktop:typecheck` | 通过，exit 0 | `tsc --noEmit` 无错误。 |
| Frontend build | `npm run desktop:build` | 通过，exit 0 | Vite build 成功，输出 `dist/index.html` 和 assets。 |
| Rust install | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh -s -- -y` | 通过，exit 0 | 安装 stable-aarch64-apple-darwin，`rustc 1.95.0`，`cargo 1.95.0`。 |
| Tauri environment check | `npm run desktop:tauri -- info` | 通过，exit 0 | Tauri CLI 可运行；Rust/Cargo/rustup 已识别。仍提示 Xcode 未安装，但 Xcode Command Line Tools 已安装。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过，exit 0 | `Finished dev profile`。 |
| Rust format | `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | 通过，exit 0 | Rust 源码格式符合 `rustfmt`。 |
| Browser preview smoke test | Vite dev server `http://127.0.0.1:1420/` + Browser DOM checks | 通过 | 初始渲染存在 `button[aria-label="Ian"]`，sprite animation 为 `idle`。 |
| Click bubble smoke test | Browser click `button[aria-label="Ian"]` | 通过 | click 后 bubble 文本为 `我在这儿。`，animation 为 `happy`。 |
| Double-click run-around smoke test | Browser double click `button[aria-label="Ian"]` | 通过 | double-click 后 animation 为 `run`。 |
| P0 desktop app 启动 | `npm run desktop:tauri -- dev` | 通过，进程已启动 | 首次因无效 placeholder icon panic；替换有效 `icons/icon.png` 并设置 `app.macOSPrivateApi=true` 后，Tauri dev 启动 `target/debug/ian_desktop`。 |
| Storage initialization smoke test | 检查 `~/.ian/config.toml`、`~/.ian/ian.db`、SQLite tables | 通过 | `config.toml` 写入 active pet/resource pack/behavior/position；SQLite 存在 `schema_migrations`、`pet_identities`、`resource_packs`、`interaction_events`、`settings_kv`。 |
| Process smoke test | `ps` / `pgrep` / System Events process list | 通过 | Tauri dev 运行时可看到 `target/debug/ian_desktop` 和前台进程 `ian_desktop`。 |
| Manual visible window acceptance | 用户人工查看桌面窗口 | 通过 | 用户确认“能看见了，验收通过”。当前为便于验收，窗口临时调为 360x360 居中显示。 |
| Product window reset | Tauri 热重载 + 用户前一项验收基础 | 通过 | 人工验收后，窗口从临时居中 360x360 调整回右下角产品形态，尺寸为 260x260，并保留淡色轮廓以避免不可见。 |
| Position persistence smoke test | drag 后重启 app | 未执行 | 仍需要人工拖拽窗口后重启验证具体坐标恢复；自动化未控制桌面拖拽。 |
| Click / double-click regression RED | `npm run desktop:test -- dragGesture`；`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml desktop::window::tests -- --nocapture` | 先失败 | 前端因缺少拖拽阈值模块失败；Rust 因缺少启动位置选择函数失败，符合回归测试预期。 |
| Click / double-click regression GREEN | `npm run desktop:test -- dragGesture` | 通过 | 1 file / 2 tests passed；覆盖 stationary click 不触发拖拽，超过阈值才开始拖拽。 |
| Startup position regression GREEN | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml desktop::window::tests -- --nocapture` | 通过 | 2 tests passed；已保存 position 优先于右下角默认位置，缺失保存位置时仍回退右下角。 |
| Physical browser interaction recheck | Vite dev `http://127.0.0.1:1420/` + Browser Playwright `getByRole('button', { name: 'Ian' }).click()` / `dblclick()` | 通过 | click 后 bubble text 为 `我在这儿。`，animation 为 `happy`；double-click 后 animation 为 `run`。 |
| Frontend tests after P1 fix | `npm run desktop:test` | 通过 | 6 files / 16 tests passed。 |
| Frontend typecheck after P1 fix | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` 无错误。 |
| Frontend build after P1 fix | `npm run desktop:build` | 通过 | Vite build 成功。 |
| Rust check after P1 fix | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | `Finished dev profile`。 |
| Rust tests after P1 fix | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 28 tests passed。 |
| Rust format after P1 fix | `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | 未通过 | 失败点在既有 0012 reminder runtime 改动的一处换行格式；本次 P1 修复未改该文件，作为 0006+ 污染面记录。 |
| Tauri dev after P1 fix | `npm run desktop:tauri -- dev` | 通过 | Vite dev 启动，Rust 编译完成并运行 `target/debug/ian_desktop`；随后手动中断进程。 |

## 验收标准结果

- [x] 仓库中存在 `apps/desktop` 下的 Tauri v2 + React + TypeScript 桌面应用骨架。
- [x] 应用可以通过文档化的 dev command 在 macOS 上启动。
- [x] 桌面右下角附近出现透明 always-on-top Ian 窗口。
- [x] Ian 从 Resource Pack 目录或 placeholder Resource Pack 渲染，路径位于 `public/resources/pets/ian-alpaca`。
- [x] Ian 至少可以播放 idle 和一个 active animation。
- [x] 点击 Ian 后，前端发出 `MouseClick` 风格的 `IanEvent`，Rust Core 返回 `IanAction`，React 渲染气泡。
- [x] Demo Dialogue 不依赖网络或 API key，也能返回短小、有角色感的回复。
- [x] 双击 Ian 后，前端发出 `MouseDoubleClick` 风格的 `IanEvent`，Rust Core 返回 `BehaviorRunAround` 或等价动作，React 播放 run 行为。
- [x] 基础位置或配置可以在应用重启后恢复。
- [x] Rust 定义 `IanEvent`、`IanAction`、`IanState` 的源头协议类型。
- [x] 前端不拥有核心行为决策，只负责渲染 Rust Core 输出的 action。
- [x] Adapter、storage、dialogue provider、behavior policy、security skeleton 存在，但不暴露未来阶段用户可见功能。
- [x] P0 未实现 Git、全局键盘监听、完整 Mood/Bond、长期记忆、Feishu、Pet Visit、plugin system 或 window-aware behavior。

## 失败或缺口

- `src/protocol/generated.ts` 目前是明确标注的 generated placeholder；后续应接通 `ts-rs` 生成脚本并核对一致性。
- Resource Pack 当前使用 `sprite.svg` + CSS placeholder creature；正式像素 sprite sheet 可在后续视觉任务中替换。
- 窗口可见性已人工验收通过；验收后已切回右下角产品形态，尺寸为 260x260，并保留淡色轮廓以避免不可见。
- `cargo fmt --check` 当前被既有 0012 reminder runtime 改动的格式问题阻塞；需要在清理 0006+ 污染面时一并处理。
- 真实桌面“拖拽窗口后退出再启动”的人工验收仍建议复跑；代码路径已补齐为保存 position -> Rust 启动定位读取 position -> 前端 state 加载二次应用 position。

## 后续

后续建议：

1. 接通并运行 Rust -> TypeScript 类型生成，替换 placeholder。
2. 复跑一次真实桌面 drag 后重启位置恢复检查，并清理 0006+ 污染面。
3. 将 placeholder `sprite.svg` 替换为正式 sprite sheet。
