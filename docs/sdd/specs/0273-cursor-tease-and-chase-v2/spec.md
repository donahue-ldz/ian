# 0273 · Cursor Tease And Chase V2

## 问题 / 目标

追鼠标如果只是机械靠近，会显得像功能触发。本 SDD 将追鼠标升级为“逗一下”的微型故事：注意、犹豫、追两步、追不上或得意收尾。

## 当前产品阶段

v0.1.x Life Feel / Cursor Play。

## 产品范围

- 基于 Life Drive 和 Micro Story 改造 pointer chase。
- 支持短追、探头、停顿、得意或放弃收尾。
- 诊断模式可 100% 触发，正式模式遵守冷却和预算。
- 鼠标快速离开时不跳屏、不追出安全区。

## 明确不做

- 不做持续游戏模式。
- 不做全局鼠标轨迹记录。
- 不追踪鼠标下方窗口内容。
- 不绕过 quiet / DND / reduced motion。

## 用户体验

用户把鼠标从 Ian 附近移开时，Ian 可能愣一下、追两步，然后停下或冒一句短气泡。它像在跟用户玩，而不是一直追着不放。

## 架构约束

- pointer 低敏事件进入 Rust Core。
- Core 决定是否生成 tease / chase story。
- React 只执行 story action，不计算追逐策略。

## 数据 / 协议变化

可复用 0271 Story 和 0272 Motion Profile。不得新增完整鼠标轨迹记录。

## 隐私与安全边界

只处理 Ian 附近的低敏 pointer 相对位置或距离等级。

## 验收标准

- [ ] pointer chase 至少包含 3 个 beat，而不是单动作。
- [ ] 追随距离、时长和次数有上限。
- [ ] 鼠标快速离开不会导致 Ian 跳屏或跑丢。
- [ ] quiet / DND / reduced motion 下会抑制或降级。
- [ ] 真实桌面观察中追随有停顿和收尾，不是机械直线跟随。

## 验证方式

- Rust story 选择和边界测试。
- 前端 movement profile 执行测试。
- 真实 Tauri 桌面鼠标靠近 / 离开 / 快速移动验收。
