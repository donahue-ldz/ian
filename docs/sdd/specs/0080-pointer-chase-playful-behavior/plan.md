# 0080 实施计划

## 步骤

1. 新增 `mouse.chase_candidate` 协议事件，字段为 `x`、`y`、`now_ms`。
2. 更新权限门，将该事件视为 Ian 桌面窗口的低敏输入。
3. 在前端桌面模式使用 Tauri `cursorPosition()` 低频读取当前鼠标屏幕坐标并发送事件。
4. 在 Rust 行为引擎中实现追逐判定：
   - 只在普通/活泼且玩闹能量未关闭时考虑。
   - 拖动、输入、跑动、安静时段、冷却中阻断。
   - 距离过近或过远阻断。
   - 触发时朝鼠标方向移动不超过单步上限。
5. 更新浏览器 fallback，只保留 no-op 或安全模拟，不读取桌面鼠标。
6. 添加测试并运行完整验证。

## 预计改动文件

- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src-tauri/src/security/permission.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src/protocol/generated.ts`
- `docs/sdd/specs/0080-pointer-chase-playful-behavior/*`

## 风险和回滚

- 风险：触发太频繁会打扰用户。用低频采样和 Rust 冷却控制。
- 风险：追逐目标太远导致突兀。用距离门槛和边界约束限制。
- 回滚：删除前端采样 effect 和 Rust 对 `mouse.chase_candidate` 的处理即可恢复。

