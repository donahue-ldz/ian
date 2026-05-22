# Moment Desktop Acceptance

本清单用于 0252-0260 moment 批次。浏览器测试只能补充；通过判断必须来自真实 Tauri 桌面壳，命令为 `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`。

## Find Ian Entrance

Trigger: 点击设置中的“找回 Ian”或启用后按找回快捷键。

Pass condition: Ian 不可见时回到安全区域，播放短入场动画/特效并显示“我在这儿。”；连续触发时降级为原地短回应。

Fail condition: 记录键盘文本、窗口丢失、连续大幅移动、气泡遮挡主要控制或出现 panic。

## Pointer Curiosity

Trigger: 鼠标靠近 Ian 后离开，或等待 pointer chase fallback。

Pass condition: Ian 低频出现探头/挥手/短追动作，随后回到 idle；设置打开、拖动、输入时不触发。

Fail condition: 持续追踪鼠标、频繁移动、设置打开时仍追逐或读取全局轨迹。

## Drag Carry

Trigger: 按住 Ian 并开始拖动。

Pass condition: Ian 进入 carry 视觉状态，原生拖动保持稳定，自动游走和追鼠标被抑制。

Fail condition: 拖动命中区错位、窗口跳动、拖动期间触发睡眠/漫游/追逐。

## Drop Settle

Trigger: 拖动 Ian 后松手。

Pass condition: 位置保存不回退，Ian 播放短安顿动作或短句，文案不责备用户。

Fail condition: 放下后回弹、位置丢失、长句刷屏或出现责备文案。

## Rare Idle Surprise

Trigger: Ian 空闲一段时间后观察桌面。

Pass condition: 低频出现挥手、整理或轻微 effect，动作短，结束回 idle。

Fail condition: 连续表演、勿扰或减少动画下仍高频触发。

## Memory Echo

Trigger: 用户确认低敏记忆后，与 Ian 进行短对话或等待可用 moment。

Pass condition: 只使用 confirmed low-sensitive tags，气泡短且自然；删除后不再使用。

Fail condition: 使用 candidate、复述原文、暴露路径/代码/私密内容。

## Cooldown And Budget

Trigger: 连续触发多个 moment。

Pass condition: 每类 moment 有 cooldown，全局预算阻止连续惊喜。

Fail condition: 连续触发多个气泡、特效或移动，无法被安静设置抑制。

## Reduced Motion And DND

Trigger: 开启减少动画或勿扰后重复上述 moment。

Pass condition: 点击反馈保留，主动移动、追逐、rare idle 和动态特效降级或关闭。

Fail condition: 减少动画下仍快速移动，勿扰下仍主动气泡或提醒。
