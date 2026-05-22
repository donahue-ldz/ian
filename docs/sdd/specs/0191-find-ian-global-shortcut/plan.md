# 0191 · 实现计划

## 实现步骤

1. 收口当前工作树和前序 SDD 状态，确认本轮只实现 `找回 Ian`，不处理未提交的其他能力。
2. 增加或确认 Tauri `global-shortcut` 插件依赖和 capability 权限，只开放注册、查询、注销所需能力。
3. 在 Rust 协议中新增低敏 `find_ian` 事件，必要时新增窗口显示 / 置前动作，并重新生成 TypeScript 类型。
4. 在 Rust Core / BehaviorPolicy 中实现 `find_ian` 决策：
   - 判断 Ian 当前可见位置是否需要找回。
   - 选择安全目标位置。
   - 输出窗口显示 / 定位 / 动画 / 气泡 / 轻微效果动作。
5. 在前端 / Tauri 执行层实现动作执行：
   - 注册 / 注销全局快捷键。
   - 快捷键回调只发送 `find_ian` 事件。
   - 执行窗口显示、聚焦或置前、移动到安全位置。
6. 在设置或托盘入口加入 `找回 Ian` 与 `启用全局快捷键` 控制。
7. 增加测试：
   - Rust 事件到动作测试。
   - 前端注册 / 注销 / 失败提示测试。
   - capability 合同测试。
   - 安全回归测试，确保没有 Keyboard Rhythm 或键盘文本采集。
8. 启动真实 Tauri 桌面端完成手动验收，并更新 `verification.md`。
9. 如实现中出现范围变化，更新 `decisions.md`。

## 预计改动文件

- `apps/desktop/src-tauri/Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/capabilities/default.json`
- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/core/creature_state.rs`
- `apps/desktop/src-tauri/src/desktop/window.rs`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/state/useIanActions.ts`
- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/IanStage.tsx`
- 相关测试文件

实际实现时可以根据已有代码边界调整文件列表，但不得扩大到 Developer Rhythm、Keyboard Rhythm 或窗口内容感知。

## 受影响接口或模块

- `IanEvent`：新增低敏系统快捷键事件或等价 find action 事件。
- `IanAction`：优先复用现有 movement / speech / animation / effect；如必须新增 window action，保持语义窄。
- `Config`：新增快捷键启用状态和快捷键字符串。
- `Security / Permission`：将全局快捷键声明为低敏、用户可关闭、用途单一的系统输入。
- `React / Tauri Executor`：负责注册快捷键和执行窗口动作，不负责决定行为策略。

## 兼容性说明

- 不改变已有点击、双击、拖动、自动游走、鼠标追逐和位置持久化语义。
- 已有用户配置必须兼容；新增配置必须有默认值。
- 如果平台不支持全局快捷键或快捷键被占用，Ian 仍可通过托盘 / 设置入口找回。
- 如果资源包缺少 `wave` 动画，回退到 `happy` 或 `run`。

## 验证命令和桌面验收

- 前端目标测试：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test -- SettingsPanel.view.test.tsx IanStage.test.tsx
```

- 类型检查：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck
```

- Rust 目标测试：

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml find_ian
```

- 桌面验收：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

桌面验收必须覆盖：

- 托盘 / 设置入口触发找回。
- 开启快捷键后 `Cmd+Shift+I` 触发找回。
- Ian 越界或难找时回到可见安全区域。
- Ian 已经可见时只回应不突兀瞬移。
- 关闭快捷键后不再响应。
- 快捷键被占用或注册失败时 UI 有提示。

## 风险和回滚

- 风险：全局快捷键被误解为全局键盘监听。
  - 应对：文案明确“只监听一个找回 Ian 快捷键，不记录输入内容”，并有测试锁定事件类型。
- 风险：不同 macOS 屏幕、缩放和多显示器坐标导致移动位置不准。
  - 应对：复用现有 monitor / safe bounds helper，桌面验收覆盖多显示器或记录无法覆盖的风险。
- 风险：快捷键与其他应用冲突。
  - 应对：注册失败可见提示，托盘 / 设置入口保底。
- 风险：窗口置前过强，打扰用户。
  - 应对：只在用户主动触发找回时执行，动作短促，不常驻闪烁。
- 回滚：移除快捷键注册和配置入口，保留托盘 / 设置手动找回或恢复为关闭状态；若协议已新增但无害，可保留默认不触发并记录兼容原因。
