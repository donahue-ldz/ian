# 0191 · Find Ian Global Shortcut

## 问题 / 目标

用户有时会在桌面上找不到 Ian：Ian 可能跑到屏幕边缘、被其他窗口遮挡、处于不可见区域，或者用户只是想快速把 Ian 叫回来。

本 SDD 建立一个低敏、用户可控的“呼叫 Ian / 找回 Ian”能力：用户可以通过托盘 / 设置入口，或用户明确开启的全局快捷键，快速让 Ian 回到当前可见桌面区域并做出有生命感的回应。

## 当前产品阶段

v0.1.x / Desktop Creature Usability。

该能力服务于桌面生命体的可发现性和可控性，不属于 Keyboard Rhythm、Developer Rhythm 或全局键盘监控。

## 产品范围

- 新增“找回 Ian”动作入口。
- 支持一个默认关闭或明确授权的全局快捷键。
- 快捷键触发后只产生“find Ian”低敏事件。
- Ian 回到当前可见屏幕的安全区域。
- Ian 做出短促、可爱的回应：动画、轻微高亮 / 呼吸、短气泡。
- 快捷键注册失败时给出可理解反馈。

## 明确不做

- 不记录用户键盘输入。
- 不统计输入节奏。
- 不接入 Keyboard Rhythm。
- 不读取当前窗口内容、屏幕 OCR、剪贴板、代码、终端输出或私聊内容。
- 不做复杂快捷键编辑器。
- 不做跨设备唤醒。
- 不让 React 直接决定 Ian 的行为策略。

## 用户体验

第一版推荐体验：

- 设置或托盘里提供 `找回 Ian`。
- 设置里提供 `启用全局快捷键` 开关，默认快捷键为 `Cmd+Shift+I`。
- 如果快捷键被占用，设置里提示“快捷键被其他应用占用”，并保留托盘 / 设置入口可用。
- 触发后：
  - 如果 Ian 不可见或位置越界，Ian 被移动到当前鼠标所在屏幕或主屏幕的右下 / 中下安全区域。
  - 如果 Ian 已经可见，不强制瞬移，只播放回应动画和短气泡。
  - 气泡文案从短句池选择，例如：`我在这儿。`、`叫我呀？`、`差点迷路了。`
  - Ian 播放 `run`、`happy`、`wave` 或资源包可用的等价动画。
  - 可选播放轻微 `effect.play`，帮助用户定位 Ian，但不能刺眼或长期闪烁。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责注册 UI 层输入、执行 IanAction、渲染动画 / 气泡和调用 Tauri window API。
- 全局快捷键只作为 `ShortcutAdapter` 或等价边界输入，转换为 `IanEvent` 后进入 Rust Core。
- 行为策略由 Rust Core / BehaviorPolicy 决定，输出 `IanAction`。
- 窗口显示、聚焦、定位只能作为 IanAction 的执行结果，不得由快捷键回调直接绕过核心行为策略。
- Tauri `global-shortcut` 插件必须显式权限授权；未授权或注册失败时不得静默失败。
- 桌面窗口坐标必须做屏幕边界 clamp，避免再次移动到不可见区域。

## 数据 / 协议变化

允许新增最小协议：

- `IanEvent::SystemShortcutTriggered { action: String, now_ms: i64 }`，其中 `action` 第一版只允许 `find_ian`。
- 如现有动作不足以表达窗口显示 / 聚焦，可新增 `IanAction::WindowShow` 或 `IanAction::WindowBringToFront`。
- 可复用已有 `movement.move_to`、`animation.play`、`speech.show`、`effect.play`。

允许新增配置：

- `find_ian_shortcut_enabled: bool`
- `find_ian_shortcut: string`

配置默认必须是安全值。若全局快捷键能力默认开启，必须只注册固定低敏快捷键；若默认关闭，设置入口必须说明其用途和隐私边界。

## 隐私与安全边界

- 快捷键事件 payload 只能包含动作名和时间，不包含按键序列之外的输入内容。
- 不记录快捷键触发历史，除非后续 SDD 明确需要低敏诊断计数。
- 不采集全局键盘文本或节奏。
- 不读取活动应用、窗口标题或屏幕内容。
- 快捷键能力必须可以关闭。
- 快捷键注册、注销和失败路径必须有测试或手动验收记录。

## 验收标准

- [ ] 设置或托盘中存在可触发的 `找回 Ian` 入口。
- [ ] 用户开启全局快捷键后，应用只注册 `找回 Ian` 用途的快捷键。
- [ ] 快捷键触发后进入 Rust Core 的事件是低敏 `find_ian` 事件，而不是 Keyboard Rhythm 事件。
- [ ] Rust Core 返回窗口显示 / 定位 / 动画 / 气泡相关 IanAction，React 只负责执行。
- [ ] Ian 在位置越界或不可见时会回到当前可见屏幕安全区域。
- [ ] Ian 已经可见时不会突兀瞬移，只做短回应。
- [ ] 快捷键注册失败时，UI 能提示占用或不可用状态。
- [ ] 关闭快捷键后不会继续响应全局快捷键。
- [ ] 测试覆盖事件协议、权限 / 配置、注册失败和关闭路径。
- [ ] 真实 Tauri 桌面端验收覆盖快捷键触发、窗口找回、动画 / 气泡显示和无明显运行错误。

## 验证方式

- 运行本 SDD 相关的 Rust 单元测试，覆盖 `SystemShortcutTriggered(find_ian)` 到 IanAction 的决策链路。
- 运行前端测试，覆盖设置开关、注册失败提示、关闭后注销。
- 运行协议生成或类型检查，确保 Rust / TypeScript 类型一致。
- 检查 Tauri capability，确认只开启必要 `global-shortcut` 权限和窗口执行权限。
- 启动真实 Tauri 桌面端：
  - 手动把 Ian 拖到屏幕边缘或移到难找位置。
  - 触发托盘 / 设置入口，观察 Ian 是否回到安全区域。
  - 开启快捷键后按 `Cmd+Shift+I`，观察 Ian 是否出现、动画和气泡是否播放。
  - 关闭快捷键后再次按快捷键，确认不再触发。
  - 记录控制台 / 运行日志是否有明显错误。
- `verification.md` 必须记录实际命令、桌面验收步骤、结果、失败项、跳过项和剩余风险。
