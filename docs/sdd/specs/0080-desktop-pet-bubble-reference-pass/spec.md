# 0080 Desktop Pet Bubble Reference Pass

## 状态

已实现，待用户验收。

## 问题 / 目标

用户希望 Ian 的气泡设计参考开源桌面宠物项目，而不是参考通用聊天气泡或普通漫画 speech bubble。

目标是把 Ian 当前气泡收敛成“桌面宠物状态泡泡”：短、小、靠近宠物头部、完整显示、像宠物冒出的念头，而不是聊天窗口。

## 开源参考

- Shijima-Qt：桌面宠物 runner，参考透明窗口和宠物存在感；GPL-3.0，不拷代码。
- desktop-homunculus：透明桌面 mascot、可拖拽、响应动作；MIT / Apache-2.0，可参考桌面宠物交互边界。
- tama96：虚拟宠物，强调状态与照料循环；MIT，可参考“短状态文本优先于聊天界面”。
- uDesktopMascot：Unity / VRM 桌面 mascot；代码 Apache-2.0，但资产有非商用限制，不使用资产。
- CodexPet Nest：透明浮层跟随宠物；FSL-1.1-MIT，暂不拷代码。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 气泡继续由 React 渲染，Rust Core 只发 `IanAction`。
- 默认气泡语义为桌面宠物状态泡泡，而不是 chat message。
- 气泡保持自适应文本大小、淡边框、窗口安全区。
- 优化头顶锚点、内边距、尾部小泡泡和整体轮廓，让它更像桌面宠物的轻量状态提示。

## 明确不做什么

- 不引入 Shijima、Comical、OpenPets、CodexPet Nest 等项目代码。
- 不使用 GPL、FSL、非商用或 ShareAlike 资产。
- 不改变 Rust 协议。
- 不新增聊天面板、长期记忆、插件系统或开发者节奏可见能力。
- 不让 Ian 变成 AI assistant / ChatGPT 桌面皮肤。

## 用户体验

- 点击 Ian 后出现小而完整的状态泡泡。
- 气泡靠近 Ian 头部，不默认占很大面积。
- 气泡尾部像宠物念头指向 Ian，而不是普通聊天消息尖角。
- 回复入口存在但低干扰，不让界面看起来像聊天软件。

## 架构约束

- React 只负责视觉和输入控件。
- Rust Core 继续决定何时展示气泡和文本内容。
- 不新增外部权限或读取桌面内容。

## 数据 / 协议变化

无。

## 隐私与安全边界

无新增外部输入、权限、存储或网络行为。

## 验收标准

- [x] `Bubble` DOM 标记明确区分桌面宠物状态泡泡语义，避免把默认气泡当作普通聊天消息。
- [x] 气泡 CSS 包含桌面宠物参考后的紧凑锚点和低干扰回复入口。
- [x] 气泡继续保留 `short` / `medium` / `long` / `input` 自适应尺寸。
- [x] 气泡边框继续使用淡描边，不恢复成重黑边。
- [x] SDD 记录参考项目与许可证取舍。

## 验证方式

- `npm run desktop:test -- Bubble.view.test.tsx`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- `npm run desktop:build`
- 浏览器视觉检查并保存截图。
