# 0021-0030 SDD 拆分总览: 核心生命感追加

## 状态

0021-0030 已实现，待用户整体验收。

## 调整原则

0020 以前的 SDD 已执行并验收通过，编号和内容保持不变。0021-0030 作为追加批次，优先补 Ian 的核心生命感：真实移动、短路径跑动、自主小范围游走、移动边界、动画表情、作息、好奇反应、亲近互动、回窝锚点和统一生命节奏。

Developer Rhythm 相关能力仍保留在 0016-0020 的既有结果中；后续是否继续推进由用户单独决定。本批不新增 Git、build/test、键盘节奏或窗口感知能力。

## 顺序

| 编号 | Packet | 阶段 | 目标 |
| --- | --- | --- | --- |
| 0021 | `0021-desktop-movement-execution` | v0.1.x Core Life | 让 `movement.move_to` 真正驱动桌面窗口或渲染层位移。 |
| 0022 | `0022-run-around-path-behavior` | v0.1.x Core Life | 双击后按短路径真实跑动，而不是只切 run 动画。 |
| 0023 | `0023-idle-micro-roaming` | v0.1.x Core Life | Ian 待机时低频小范围自主走动。 |
| 0024 | `0024-movement-boundary-policy` | v0.1.x Core Life | 建立屏幕安全边界、移动幅度和行为模式约束。 |
| 0025 | `0025-animation-expression-upgrade` | v0.1.x Core Life | 增强 idle/walk/run/happy/sleep 表情与动作过渡。 |
| 0026 | `0026-rest-and-sleep-cycle` | v0.1.x Core Life | 建立不打扰的趴下、休息、醒来节奏。 |
| 0027 | `0027-curiosity-attention-reactions` | v0.1.x Core Life | 鼠标靠近、停留、离开时有好奇/注意力反应。 |
| 0028 | `0028-affection-touch-loop` | v0.1.x Core Life | 点击、轻抚、连续互动形成亲近反馈，不数值化展示。 |
| 0029 | `0029-home-and-anchor-behavior` | v0.1.x Core Life | 支持 home/anchor，Ian 能回到用户认可的位置。 |
| 0030 | `0030-life-rhythm-composition` | v0.1.x Core Life | 统一 Movement、Mood、Bond、Rest 的生命节奏策略。 |

## 依赖关系

```txt
0021 -> 0022 -> 0023
0021 -> 0024
0024 -> 0029
0025 可与 0023 并行，但应在 0026 / 0028 前完成
0026 依赖 0023 / 0024
0027 依赖 0021 / 0025
0028 依赖 0011 / 0025
0030 依赖 0023 / 0026 / 0028 / 0029
```

## 防漂移规则

- 不改动 0020 以前的既有 SDD 编号和验收结论。
- 本批不做 Developer Rhythm 新能力。
- 自主移动必须可被拖拽、设置和边界策略覆盖。
- 好奇反应只使用 Ian 窗口内 pointer 事件，不做全局鼠标监听。
- 亲近反馈不展示经验条、等级、积分或任务化 UI。
- 生命节奏策略不应变成提醒中心、效率助手或通知器。
