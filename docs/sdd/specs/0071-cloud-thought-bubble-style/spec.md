# Spec: Cloud Thought Bubble Style

## 状态

已实现，待用户验收。

## 问题 / 目标

当前 Ian 气泡偏普通圆角面板，和用户给出的参考图相比缺少“云朵想法气泡”的可爱感。0071 目标是把气泡视觉改成白底、黑色粗描边、圆润云朵边缘和小圆尾巴的样式，让点击反馈更像桌面小生命在冒想法。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 调整 Ian bubble 的视觉外观为云朵想法气泡。
- 保留短文本、输入框和回复按钮功能。
- 云朵边缘与尾巴用本地 CSS 实现，不新增远程资源。
- 在桌面小窗口尺寸下保证文字不溢出、不遮挡 Ian 和设置入口。

## 明确不做什么

- 不改 Rust Core、Dialogue、IanEvent / IanAction 协议。
- 不新增长对话、聊天面板或 assistant 风格 UI。
- 不引入图片依赖或图标库。
- 不改 bubble 的文案、触发频率和输入行为。

## 用户体验

用户点击 Ian 后看到的是更像手绘想法云朵的气泡：主体柔软，描边清楚，尾巴指向 Ian，整体更可爱但仍然轻巧。

## 架构约束

- React 仍只渲染 bubble 和输入控件。
- 行为决策仍来自 Rust Core action。
- 视觉改动应限制在 `Bubble` 视图测试和 `ianStage.css`。

## 数据 / 协议变化

无协议和持久化变化。

## 隐私与安全边界

不读取任何新数据，不新增网络访问，不改变用户输入处理。

## 验收标准

- [x] `.ian-bubble` 使用白色主体和黑色粗描边，视觉上不再是普通半透明面板。
- [x] 气泡主体具备云朵边缘，至少包含多处圆形凸起。
- [x] 气泡尾巴由两个小圆点组成，方向靠近 Ian。
- [x] 文本、输入框和回复按钮仍在气泡内部可读可操作。
- [x] Browser smoke 截图记录新气泡外观。

## 验证方式

- `npm run desktop:test -- Bubble.view.test.tsx`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- Browser smoke 截图。
