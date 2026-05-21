# Spec: P0 本地数字生命验证版

## 状态

草稿，等待用户确认。

## 背景

Ian 是一个 local-first 的桌面数字生命。当前项目阶段是：

```txt
P0 / MVP + v0.1 Architecture Baseline
```

这份 spec 定义第一版可运行验证目标：做出一个足够有生命感、值得停留在用户桌面上的小型桌面生命，同时建立后续迭代可以延展的长期架构边界。

参考文档：

- `AGENTS.md`
- `docs/codex/ian-current-stage.md`
- `docs/codex/ian-architecture-rules.md`
- `docs/codex/sdd-workflow.md`
- `ian.md`

## 目标

构建最小可运行的 Ian 桌面体验：

- Ian 以透明、置顶的小桌面生命形式出现。
- Ian 能待机，并能响应简单用户互动。
- 点击 Ian 后弹出短句气泡，回复来自本地 Demo Dialogue。
- 双击 Ian 后触发短暂乱跑彩蛋。
- 实现同时建立 Rust Core / React / protocol 的架构骨架。

## 当前阶段

```txt
P0 / MVP + v0.1 Architecture Baseline
```

## 产品范围

P0 用户可见范围：

- 透明 always-on-top 桌面窗口。
- Ian 默认出现在桌面右下角附近。
- 使用占位图或简单 sprite 渲染 Ian。
- 基础动画状态：idle、walk、happy、run。
- 点击后打开气泡。
- Demo Dialogue 返回短小、有角色感的回复。
- 双击后触发乱跑彩蛋。
- 支持基础拖拽和位置持久化。

v0.1 Architecture Baseline 范围：

- Tauri v2 桌面壳。
- React + Vite + TypeScript 前端。
- `src-tauri` 内的 Rust Core Runtime。
- Rust 源头协议类型：`IanEvent`、`IanAction`、`IanState`。
- TypeScript 协议类型生成路径。
- Resource Pack 目录和 manifest 结构。
- Adapter trait，以及最小 Time / Mouse / Dialogue adapters。
- Behavior policy 边界。
- Dialogue provider 边界。
- SQLite + config skeleton。
- migration 和 repository skeleton。
- Security / Permission / Sanitizer / Rate Limiter skeleton。

## 非目标

本 packet 不得把以下能力实现为用户可见功能：

- Git / build / test / CI 集成。
- 全局键盘监听。
- 活跃窗口感知。
- 代码读取。
- Terminal 输出读取。
- 完整 Mood System。
- 完整 Bond System。
- 长期记忆。
- 主动提醒系统。
- Feishu。
- Pet Visit。
- Plugin system。
- 多宠物 / 多 Ian。
- 复杂设置 UI。
- 把 BYOM UI 作为 P0 核心体验。

允许为未来能力建立架构 skeleton，但必须保持边界干净，并且不能变成 P0 用户可见功能。

## 用户体验

应用启动后，Ian 出现在桌面右下角附近，以一个小型透明 always-on-top 窗口存在。

Ian 通过最小动画表现生命感：

- idle：安静存在。
- walk：小范围移动。
- happy：短暂正向反馈。
- run：有活力的彩蛋反应。

点击 Ian 后，弹出一个短句气泡，Demo Dialogue 可以返回类似：

```txt
我在这儿。
喝水水。
才不是担心你。
哞？
```

双击 Ian 后，在当前允许的桌面 / 窗口活动边界内触发短暂乱跑彩蛋。

第一体验应该传达：

> 有一个小生命住在这里。

而不是：

> 打开了一个聊天工具。

## 架构约束

Everything is an Event：

- 用户互动以 `IanEvent` 进入 Rust Core。
- Rust Core 输出 `IanAction`。
- React 执行 `IanAction`。
- 前端可见状态用 `IanState` 表示。

职责边界：

- Rust Core 负责行为决策、状态流转、policy 边界、scheduler、dialogue orchestration、storage skeleton 和 security skeleton。
- React 负责渲染、动画播放、气泡 UI、拖拽交互表面，以及未来可能出现的设置界面。
- React 不拥有 Ian 的长期行为逻辑。
- Adapter 只把外部信号转换成 `IanEvent`，不能直接控制动画或持久化状态。

协议：

- Rust 类型是协议源头。
- TypeScript 类型应由 Rust 类型生成；如果 P0 暂时无法接通生成链路，必须有明确标注的临时 generated placeholder。

## 数据与协议变化

预期协议类型：

- `IanEvent`
  - `AppStarted`
  - `TimeTick`
  - `MouseClick`
  - `MouseDoubleClick`
  - `MouseDragStart`
  - `MouseDragEnd`
  - `DialogueUserMessage`
- `IanAction`
  - `AnimationPlay`
  - `MovementMoveTo`
  - `SpeechShow`
  - `BubbleOpen`
  - `BubbleClose`
  - `BehaviorRunAround`
  - `StateSync`
- `IanState`
  - active pet id
  - current behavior
  - current animation
  - position
  - active resource pack
  - behavior mode

预期 storage / config skeleton：

- 本地 app 目录初始化。
- config 文件路径和默认值。
- SQLite database connection skeleton。
- migration runner skeleton。
- pet identity / settings / interaction events 的 repository skeleton。

P0 持久化要求：

- 基础位置或配置在重启后能够恢复。

## 隐私与安全

P0 只使用低敏本地输入：

- app lifecycle
- Ian 窗口内 click
- Ian 窗口内 double-click
- Ian drag start / end
- local time tick
- local Demo Dialogue text

P0 不得读取：

- 全局键盘输入
- clipboard
- code content
- private chat content
- screen OCR
- current file content
- terminal output

Security skeleton 应存在，但 P0 不应请求高敏权限。

## 验收标准

- [ ] 仓库中存在 `apps/desktop` 下的 Tauri v2 + React + TypeScript 桌面应用骨架。
- [ ] 应用可以通过文档化的 dev command 在 macOS 上启动。
- [ ] 桌面右下角附近出现透明 always-on-top Ian 窗口。
- [ ] Ian 从 Resource Pack 目录或 placeholder Resource Pack 渲染，路径位于 `public/resources/pets/ian-alpaca`。
- [ ] Ian 至少可以播放 idle 和一个 active animation。
- [ ] 点击 Ian 后，前端发出 `MouseClick` 风格的 `IanEvent`，Rust Core 返回 `IanAction`，React 渲染气泡。
- [ ] Demo Dialogue 不依赖网络或 API key，也能返回短小、有角色感的回复。
- [ ] 双击 Ian 后，前端发出 `MouseDoubleClick` 风格的 `IanEvent`，Rust Core 返回 `BehaviorRunAround` 或等价动作，React 播放 run 行为。
- [ ] 基础位置或配置可以在应用重启后恢复。
- [ ] Rust 定义 `IanEvent`、`IanAction`、`IanState` 的源头协议类型。
- [ ] 前端不拥有核心行为决策，只负责渲染 Rust Core 输出的 action。
- [ ] Adapter、storage、dialogue provider、behavior policy、security skeleton 存在，但不暴露未来阶段用户可见功能。
- [ ] P0 未实现 Git、全局键盘监听、完整 Mood/Bond、长期记忆、Feishu、Pet Visit、plugin system 或 window-aware behavior。

## 验证方式

验证应包括：

- dependency install check
- 前端 typecheck 或 build
- Rust check 或 Tauri build check
- protocol generation 或 placeholder 一致性检查
- 手动启动桌面应用的 smoke test
- 手动点击和双击互动检查
- 重启后的持久化 smoke check

具体命令会在 `plan.md` 中根据最终脚手架、包管理器和 scripts 确认。

## 开放问题

- P0 第一版视觉应使用生成的 placeholder sprite sheet、简单 CSS creature，还是最小 checked-in bitmap asset？
- BYOM provider 实现是否完全延后到 v0.1.x，P0 只保留 dialogue provider boundary？

