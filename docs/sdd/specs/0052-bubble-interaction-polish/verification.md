# Verification: Bubble Interaction Polish

## 状态

已实现，待用户验收。

## 实际验证

- [x] TDD RED：`npm run desktop:test -- Bubble.test.ts Bubble.view.test.tsx ianActions.test.ts` 失败符合预期，缺少短句格式化、输入折叠和输入 active 替换规则。
- [x] `npm run desktop:test`：通过，8 个 test files / 30 个 tests。
- [x] `npm run desktop:typecheck`：通过，`tsc --noEmit` 退出 0。
- [x] Rust protocol / behavior tests：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 通过，76 个 Rust tests。
- [x] Browser smoke：点击 Ian 后出现短反馈气泡，默认仅展示 `打开气泡输入` 按钮；展开后出现 `对 Ian 说一句话` 输入框。
- [x] 截图记录：`docs/sdd/specs/0052-bubble-interaction-polish/screenshots/bubble-feedback.jpg`、`docs/sdd/specs/0052-bubble-interaction-polish/screenshots/bubble-input.jpg`。

## 结果

默认反馈气泡经过 `formatBubbleText` 收敛为短句并用 CSS 限制两行。输入框不再默认展开，用户点击低干扰输入按钮后才进入输入态；输入 active 时覆盖性 speech 不替换当前文本，Core 结束输入后的用户回复仍可替换。

## 失败或缺口

Browser 后续 submit smoke 中 Codex Browser 多次出现 locator / CDP timeout，未把“提交后回复文本变化”作为截图验收项。该路径已由 `ianActions.test.ts` 覆盖：输入 active 时继续 active 的 speech 被忽略，输入结束后的 speech 允许替换。

## 2026-05-21 视觉补丁复验

- [x] `npm run desktop:test`：通过，10 个 test files / 41 个 tests。
- [x] `npm run desktop:typecheck`：通过，`tsc --noEmit` 退出 0。
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，90 个 Rust tests。
- [x] `npm run desktop:build`：通过，Vite production build 成功。

本次只调整 `ianStage.css` 中气泡视觉：更圆润的气泡圆角、暖白半透明背景、更轻的边框和阴影、圆形输入入口、圆角输入框。未改变 `IanEvent` / `IanAction`、输入 active 行为或气泡文案策略。
