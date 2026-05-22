# 0195 · 验证记录

状态：已执行，待后续桌面 SDD 复用。

## 实际结果

| 验证项 | 命令 / 检查 | 结果 | 说明 |
| --- | --- | --- | --- |
| RED | `test -f docs/sdd/desktop-smoke-checklist.md` | 失败符合预期 | 清单文件不存在。 |
| GREEN | `test -f docs/sdd/desktop-smoke-checklist.md` | 通过 | 清单已覆盖启动、基础存在感、点击气泡、拖动、位置恢复、找回、快捷键、设置隐私、资源包和记录要求。 |
| Frontend tests | `PATH=/opt/homebrew/bin:$PATH npm run desktop:test` | 通过 | 12 files / 91 tests。 |
| Typecheck / build | `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`; `PATH=/opt/homebrew/bin:$PATH npm run desktop:build` | 通过 | 类型检查和 Vite build 成功。 |
| Tauri launch smoke | `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait` | 通过启动 | 复用已有 dev server，Tauri app 编译并运行。 |

## 剩余风险

- 清单优先人工可执行；尚未把所有桌面交互自动化。
- 后续 SDD 仍必须把真实桌面执行结果写入各自 `verification.md`。
