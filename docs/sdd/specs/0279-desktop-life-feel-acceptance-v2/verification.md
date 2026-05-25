# 0279 · 验证记录

## 状态

已建立生命感二阶段验收清单，并完成可自动化的桌面启动和可见性验收。交互类桌面验收受当前 macOS Accessibility 限制，需要人工补验。

## 实现结果

- 新增 `docs/sdd/life-feel-acceptance-v2.md`。
- 清单覆盖 SDD 0270-0278，每项都有 `Trigger:`、`Expected:`、`Fail condition:`、`Degrade check:`。
- 清单明确要求真实 Tauri 桌面壳验收，不允许用浏览器替代桌面验收。
- 清单明确隐私边界：不记录屏幕内容、代码正文、剪贴板或窗口标题。
- 新增 `apps/desktop/src/acceptance/lifeFeelAcceptance.test.ts`，防止清单漏掉 0270-0278 或缺少客观验收字段。

## TDD 记录

- RED：`PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run src/acceptance/lifeFeelAcceptance.test.ts` 首次失败，原因是 `docs/sdd/life-feel-acceptance-v2.md` 不存在。
- GREEN：补齐验收清单后，同一命令通过，1 test passed。

## 命令验证

- `rg -n "0270|0271|0272|0273|0274|0275|0276|0277|0278" docs/sdd/life-feel-acceptance-v2.md`：全部覆盖。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run src/acceptance/lifeFeelAcceptance.test.ts`：1 passed。

## 真实桌面验证

- 启动 Vite：`PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite --host 127.0.0.1 --port 1420`。
- 启动真实 Tauri desktop shell：`PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH tauri dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`。
- `pgrep -fl 'target/debug/ian_desktop'` 返回桌面进程：`31762 target/debug/ian_desktop`。
- `screencapture -x /tmp/ian-0279-life-feel-acceptance-v2.png` 已生成截图并确认 Ian 在真实桌面可见。
- `osascript -e 'tell application "System Events" to UI elements enabled'` 返回 `false`。

## 降级模式记录

- 清单要求 DND、quiet、reduced motion 每项都纳入验收。
- 本轮自动化只能验证文档覆盖和既有 Core / 前端测试；由于 Accessibility 为 `false`，无法自动点击设置、拖动 Ian、触发诊断按钮或模拟真实鼠标离开。

## 剩余体验问题和下一轮优先级

1. 补一次人工桌面验收：按 `docs/sdd/life-feel-acceptance-v2.md` 逐项观察追鼠标、拖动、放下、找回、private life 和 Novelty。
2. 若拖动/追鼠标手感仍不够惊喜，下一轮优先拆“桌面交互手感调参 SDD”，只基于真实观察问题改。
3. 若 idle private life 出现频率偏低或偏高，下一轮优先拆“private life cadence tuning SDD”，调整冷却、预算和候选权重。
