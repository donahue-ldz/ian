# Spec: Curiosity Attention Reactions

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要对用户在自身窗口内的靠近、停留、离开有更自然的注意力反应。0027 目标是建立轻量好奇反应：转头、开心、后退一点或短句反馈，但仍只使用 Ian 窗口内 pointer 事件。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 扩展窗口内 pointer enter / hover / leave 的事件表达。
- BehaviorEngine 根据短时窗口内 pointer 状态输出注意力动作。
- 可结合 MovementMoveTo 做小幅靠近或后退。
- 反应受 cooldown 控制，避免刷屏。

## 明确不做什么

- 不做全局鼠标监听。
- 不读取窗口外鼠标轨迹。
- 不做眼球追踪或系统辅助权限。
- 不把 hover 变成频繁气泡。

## 用户体验

鼠标靠近 Ian 时，它会像注意到用户一样轻微反应；停留太久可能好奇地动一下；离开后回到 idle。

## 架构约束

- Pointer 事件只从 Ian 窗口内进入 `IanEvent`。
- Rust Core 决定反应类型和冷却。
- React 不根据 hover 自行决定动画。

## 数据 / 协议变化

可复用 `MouseNear`，必要时新增 `mouse.leave` 或 pointer presence 事件。新增协议必须从 Rust 生成 TypeScript。

## 隐私与安全边界

只处理 Ian 窗口内 pointer event，不请求系统级监听权限。

## 验收标准

- [ ] Ian 窗口内 pointer enter / near 可触发 attention action。
- [ ] pointer leave 后可回到 idle 或降低 attention。
- [ ] hover 反应有 cooldown，不频繁弹气泡。
- [ ] 不存在全局鼠标监听代码路径。
- [ ] 测试覆盖 enter、leave、cooldown 和 React 不决策。

## 验证方式

- Rust behavior tests。
- Frontend pointer event tests。
- Source scan 检查无 global mouse API。
- Browser smoke：pointer enter/leave 无 console error。
