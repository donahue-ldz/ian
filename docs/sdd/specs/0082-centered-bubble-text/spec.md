# 0082 Centered Bubble Text

## 状态

已实现，待用户验收。

## 问题 / 目标

用户希望气泡字体居中。当前气泡文字默认左对齐，在短文本状态泡泡中显得不够像桌面宠物的轻量念头。

目标是让气泡文字在气泡内容区水平居中，同时保留现有气泡尺寸、边框、锚点、尾泡和低干扰回复入口。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 让 `.ian-bubble-text` 文本水平居中。
- 保留两行截断和自适应尺寸。
- 保留 `pet-status` / `thought-cloud` 语义。

## 明确不做什么

- 不改变气泡位置、边框粗细、尾泡形状。
- 不改变 Demo Dialogue 文案。
- 不改变 Rust Core 协议或行为。
- 不新增外部依赖。

## 用户体验

- 短句如“我在这儿。”显示在气泡中间。
- 长句两行仍保持居中排版和截断。
- 回复按钮仍保持低干扰。

## 架构约束

- React 行为不变，只调整 CSS。
- Rust Core 行为不变。

## 数据 / 协议变化

无。

## 隐私与安全边界

无新增隐私或安全影响。

## 验收标准

- [x] 气泡文字 CSS 明确设置 `text-align: center`。
- [x] 气泡文字保留两行截断规则。
- [x] 气泡的自适应尺寸、极细边框和桌面宠物语义保持不变。

## 验证方式

- `npm run desktop:test -- Bubble.view.test.tsx`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- `npm run desktop:build`
- 浏览器视觉检查并保存截图。
