# Spec: Idle Rest Visual Comfort

## 状态

草稿，待确认。

## 问题 / 目标

Ian 不能靠频繁移动证明自己活着。0058 目标是优化 idle、rest、sleep 的视觉舒适度，让 Ian 在少动时仍有生命感，并且适合长时间停留在桌面。

## 当前产品阶段

Product Feel 体验优化。

## 产品范围

- 为 idle、rest、sleep 定义清晰视觉状态和切换规则。
- 增加或调优呼吸、眨眼、姿态、小幅表情等微动作。
- 支持 reduced motion 或 quiet mode 下的更低动效。
- 避免睡眠、休息和气泡出现时的视觉冲突。

## 明确不做什么

- 不新增完整 Mood System。
- 不做全天候作息推理。
- 不做复杂睡眠养成系统。
- 不用大幅移动替代微动作。

## 用户体验

用户不操作 Ian 时，Ian 仍然像是在安静陪伴：有呼吸、偶尔眨眼、可能休息或睡着，但不会一直跑来跑去，也不会突然大量弹话。

## 架构约束

- 状态切换由 Rust Core 调度。
- React 执行动画状态和过渡，不能自行发明休息策略。
- reduced motion / quiet mode 应由统一设置或 policy 影响。

## 数据 / 协议变化

可扩展 action 或 state 中的 rest visual state。已有 idle / sleep 状态优先复用。

## 隐私与安全边界

不读取外部内容。若休息策略参考时间或 quiet hours，只使用本地配置。

## 验收标准

- [ ] idle、rest、sleep 至少有可区分视觉表现。
- [ ] 5 分钟 idle 不出现空白帧、卡死或过度移动。
- [ ] reduced motion / quiet mode 明显降低动效强度。
- [ ] 睡眠和气泡不会互相遮挡或产生状态冲突。
- [ ] 用户交互可自然唤醒或打断休息。

## 验证方式

- Rust behavior tests 覆盖 idle / rest / sleep 状态切换。
- Browser smoke 记录 idle 5 分钟。
- 截图或录屏检查 reduced motion / quiet mode。
