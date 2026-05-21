# Spec: Puppy Resource Pack + Micro Life Upgrade

## 状态

已批准并实现；已处理第一轮视觉和速度验收反馈。

## 问题 / 目标

当前 Ian 已有点击、拖拽、跑动、zoomies、亲近反馈和气泡，但整体仍偏“事件触发器”：动作之间缺少小过渡，idle 时缺少持续生命感，Ian 对刚刚发生的事缺少短期记忆。

0082 目标是在不扩大成完整 Mood / Bond System 的前提下，做一个小而完整的灵动性升级：

- 新增一套参考用户提供小狗视觉方向的 `ian-puppy` bundled Resource Pack。
- 把部分单点反应升级为 2-4 步短反应链。
- 增加 5-10 分钟级别的内存态“日常性格”，让 Ian 对刚被摸、刚睡醒、刚跑完、被冷落一会儿有轻微差异化反应。

## 当前产品阶段

v0.1.x Core Life / Resource。

本任务增强基础生命感，不能把 Ian 变成 AI assistant、通知器、开发工具状态面板或完整宠物平台。

## 产品范围

### 1. Puppy Resource Pack

- 新增 `apps/desktop/public/resources/pets/ian-puppy/`。
- 新增 `pet.json`、`animations.json`、`expressions.json`、`sprite.svg`。
- 视觉方向参考用户提供的小柴犬：圆脸、短腿、蓬尾巴、开心但不吵。
- 至少覆盖现有语义动画：
  - `idle`
  - `rest`
  - `walk`
  - `sleep`
  - `run`
  - `zoomies`
  - `happy`
- `idle` 至少包含眨眼 / 呼吸 / 尾巴慢摇中的两类视觉变化。
- `run` / `zoomies` 必须和普通 `walk` 明显不同。
- 新安装默认资源包可切到 `ian-puppy`；已有用户配置不被强制覆盖。

### 2. 微动作 + 短反应链

- 保持 Rust Core 决定行为，React 只执行 `IanAction`。
- idle tick 可低频触发轻微微动作，例如看一眼、伸懒腰、抖耳、尾巴摇。
- 双击跑动或 zoomies 结束前后增加 settle 过渡，例如跑完晃一下、回头、喘一下。
- 拖拽放下后增加短链路：落点移动 -> 站稳/晃一下 -> 短气泡或轻特效。
- 鼠标靠近不总是立即弹文字，可先短动画或特效，再可选短句。

### 3. 轻量“日常性格”内存态

- 在 Rust Core 内部维护短期、非持久、低敏状态。
- 状态只来源于 Ian 自身本地事件：
  - 点击 / 连续点击
  - 拖拽开始 / 结束
  - 睡眠 / 醒来
  - 跑动 / zoomies
  - idle 时间流逝
- 状态生命周期为 5-10 分钟级别，重启后丢失。
- 允许行为策略读取这些短期状态，以调整短句、动画或微动作候选。

## 明确不做什么

- 不做完整 Mood System。
- 不做完整 Bond System。
- 不做长期记忆。
- 不做多宠物同时出现。
- 不做资源包市场、在线下载或插件系统。
- 不引入外部 agent / MCP / Git / build / keyboard / active window 新行为。
- 不读取代码、屏幕、剪贴板、聊天或任意外部文件内容。
- 不让 React 根据短期状态自行决定 Ian 行为。
- 不引入复杂行为树、AI 规划器或新的动画引擎。

## 用户体验

用户看到的是：Ian 更像一只住在桌面上的小狗，而不是一个只会响应点击的图标。

- 静止时也有很轻的生命迹象。
- 跑完、被放下、刚睡醒时不会瞬间切回 idle。
- 连续互动时，Ian 的反馈有一点“刚才你摸过我”的延续感。
- 安静模式、输入中、拖拽中、专注场景下不会突然大幅打扰。

## 架构约束

- `IanEvent` 仍是所有输入入口。
- `IanAction` 仍是所有行为输出。
- `IanState` 可暴露必要的当前状态，但短期日常性格默认不进入持久公开状态，除非实现验证需要极小诊断字段。
- Rust Core 拥有短期状态、行为策略、冷却和反应链选择。
- React 只渲染 Resource Pack、执行 animation / movement / bubble / effect actions。
- Resource Pack 只描述视觉资源和动画帧，不携带脚本，不执行逻辑。

## 数据 / 协议变化

允许但不要求新增语义动画名。优先复用现有：

- `idle`
- `rest`
- `walk`
- `sleep`
- `run`
- `zoomies`
- `happy`

如实现中确需新增 `settle`、`stretch` 或 `look`，必须：

- 从 Rust protocol 类型生成 TypeScript。
- 为旧资源包提供 fallback 到 `idle` / `happy` / `run`。
- 不让旧 `ian-alpaca` / `ian-kitten` 白屏或崩溃。

配置变化：

- 新增 bundled resource pack `ian-puppy`。
- 新安装默认资源包可改为 `ian-puppy`。
- 已存在用户配置不被迁移覆盖。

持久化变化：

- 本任务不新增长期表。
- 短期日常性格不写入 SQLite。

## 隐私与安全边界

- 所有触发只基于本地 Ian 事件、当前 Ian 状态和用户显式配置。
- 资源包加载只读取 Ian bundled resource 目录。
- 不联网、不下载资源、不执行资源包脚本。
- 不记录用户消息全文之外的新敏感内容。
- 微动作和短期状态不得依赖屏幕内容、代码内容、剪贴板、私聊或终端输出。

## 验收标准

- [ ] `ian-puppy` Resource Pack 存在，包含 `pet.json`、`animations.json`、`expressions.json`、`sprite.svg`。
- [ ] `ian-puppy` 通过 resource loader 校验。
- [ ] `ian-puppy` 的 `idle`、`happy`、`walk`、`run`、`zoomies`、`rest`、`sleep` 至少映射到可区分帧或可区分帧序列。
- [ ] 新安装默认资源包可显示 `ian-puppy`；已有用户配置不会被强制覆盖。
- [ ] idle tick 可在 normal / lively 且非 quiet hours、非输入、非拖拽时触发低打扰微动作。
- [ ] quiet mode、quiet hours、bubble input、dragging 状态会阻止自发微动作和高能短链路。
- [ ] 跑动或 zoomies 后存在 settle 过渡，不直接瞬间回到普通 idle。
- [ ] 拖拽放下后存在 2-4 步短反应链。
- [ ] Rust Core 维护 5-10 分钟级别短期日常性格状态，且重启不持久化。
- [ ] 短期状态能影响至少两类反馈：例如刚睡醒短句、刚跑完 settle、刚被摸后的亲近反馈。
- [ ] React 未根据短期状态自行决定行为，只执行 Rust Core 输出的 `IanAction`。
- [ ] 旧 `ian-alpaca` 和 `ian-kitten` 缺少新动画时 fallback 稳定，不白屏、不崩溃。
- [ ] 相关 Rust 和 frontend 测试覆盖资源包、fallback、反应链、阻断条件和短期状态过期。

## 验证方式

- `npm run desktop:test -- resourceLoader.test.ts AnimationPlayer.test.ts ianActions.test.ts`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml protocol`
- Browser / desktop smoke：
  - 默认打开能看到 `ian-puppy`。
  - idle 2-3 分钟内出现低打扰微动作。
  - 双击跑动后有 settle 过渡。
  - 拖拽放下后出现短反应链。
  - quiet mode / 输入中不触发自发微动作。
