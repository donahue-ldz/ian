# 验证记录: Bubble Dialogue

## 状态

已验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| TDD RED | `npm run desktop:test` | 失败符合预期 | 缺少 `normalizeBubbleMessage`。 |
| Frontend tests | `npm run desktop:test` | 通过 | 3 个测试文件，9 个测试通过。 |
| Frontend typecheck | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` 通过。 |
| Frontend build | `npm run desktop:build` | 通过 | Vite production build 通过。 |
| Rust dialogue tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue` | 通过 | 1 个 dialogue 测试通过。 |
| Playwright | `http://localhost:1420/` | 通过 | 点击出现输入框；输入“喝水”回车后显示“喝水水。”；输入清空；console 无 error。 |

## 验收标准结果

- [x] 点击 Ian 后气泡包含短输入框。
- [x] 输入文本回车会发送 `dialogue.user_message`。
- [x] Rust Core Demo Dialogue 返回短句并显示在气泡中。
- [x] 空输入不会发送事件。
- [x] 气泡输入不会破坏单击短句、双击 run-around。
- [x] Playwright 可验证输入短句后显示 Demo 回复。
