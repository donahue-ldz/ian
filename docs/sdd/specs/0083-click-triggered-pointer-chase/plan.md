# 0083 实施计划

## 实现步骤

1. 为点击追逐候选逻辑补前端测试。
   - 对应验收标准：点击保留 `mouse.click`、点击后布防、鼠标离开时 100% 发送 `mouse.chase_candidate`、短延迟兜底发送候选、浏览器预览不读取坐标。
2. 在 `App.tsx` 增加点击布防和离开采样逻辑。
   - 只在桌面模式执行。
   - 点击时只发送 `mouse.click` 并记录布防状态。
   - 鼠标离开 Ian 时调用 `getDesktopCursorPosition()` 并发送 `mouse.chase_candidate`。
   - 点击后设置短延迟兜底；如果离开事件已触发则取消兜底，避免重复发送。
   - 当前体验验证阶段，离开后的候选触发率为 100%。
3. 保持 Rust Core 不变。
   - 点击后的追逐候选继续走 0080 的 `IanEvent::MouseChaseCandidate`。
   - 不新增 Rust 点击追逐分支。
4. 更新 verification。

## 预计改动文件

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/App.test.tsx`
- `docs/sdd/specs/0083-click-triggered-pointer-chase/spec.md`
- `docs/sdd/specs/0083-click-triggered-pointer-chase/plan.md`
- `docs/sdd/specs/0083-click-triggered-pointer-chase/decisions.md`
- `docs/sdd/specs/0083-click-triggered-pointer-chase/verification.md`

## 接口和模块影响

- 不新增协议类型。
- 不新增 Rust Core 行为分支。
- 前端新增一个点击后的候选事件发送路径，但最终行为仍由 Rust Core 判定。

## 兼容性说明

- 浏览器预览保持现状，不读取桌面鼠标。
- 桌面端需要 Tauri `cursorPosition()` 可用；不可用时不触发候选事件。
- 现有 10 秒低频自动追逐候选不变。

## 验证命令

- `npm run desktop:test -- App.test.tsx`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`
- `npm run desktop:typecheck`
- `npm run desktop:test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`

## 风险和回滚

- 风险：离开后追逐过频繁会打扰用户。当前为体验验证临时 100% 候选触发，并继续依赖 Rust Core 冷却；后续可调回低概率。
- 风险：测试中需要稳定控制随机值。测试中 mock `Math.random()` 和 `getDesktopCursorPosition()`。
- 回滚：删除点击后的追逐候选发送逻辑即可恢复 0080 原有低频追逐。
