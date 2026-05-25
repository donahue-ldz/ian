# Life Feel Acceptance V2

本清单用于验收 0270-0278 生命感二阶段能力。验收必须在真实 Tauri 桌面壳中执行，不要用浏览器替代桌面验收。记录结果时不记录屏幕内容、代码正文、剪贴板或窗口标题，只记录 Ian 自身行为、动作序列、设置状态和是否通过。

## 使用方式

1. 启动桌面端：`tauri dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`。
2. 打开设置，确认本地诊断可用；如无法自动化点击，记录 Accessibility 状态和人工观察结果。
3. 每个条目按 Trigger / Expected / Fail condition / Degrade check 执行。
4. DND、quiet、reduced motion 至少各覆盖一次。
5. 验收结论必须写入对应 SDD 或 0279 的 `verification.md`。

## SDD 0270 Life Drive Model

Trigger: 在桌面端依次做点击 Ian、靠近鼠标、拖起、放下、等待 idle tick；同时查看 Rust Core 测试或诊断输出是否仍只暴露低敏 reason。

Expected: Ian 的后续动作倾向发生轻微变化，例如追鼠标强度、idle 私生活候选顺序或放下安顿反馈有差异，但界面不显示 curiosity、comfort、boredom、affection、energy 数值。

Fail condition: 前端维护或展示 Life Drive 分数；DND / quiet / reduced motion 被 Life Drive 绕过；行为需要读取外部窗口、代码或文本内容。

Degrade check: 开启 DND、quiet、reduced motion 后，Life Drive 仍可内部更新，但不能强行触发高打扰动作。

## SDD 0271 Micro Story Moment Framework

Trigger: 用 Moment 诊断触发 pointer curiosity 或 private life Moment，观察是否出现发现、停顿、行动、收尾的有序短故事。

Expected: Story 由 Rust Core 输出为 `IanAction` 序列，React 只播放动作；故事短于 5 秒或可被用户交互打断。

Fail condition: 只播放单个僵硬动作；React 自己随机决定 story 顺序；用户拖动或设置打开时 story 仍持续抢控制。

Degrade check: reduced motion 下故事缩短，去掉大幅移动或强动效，但保留温和反馈。

## SDD 0272 Motion Physics Feel Layer

Trigger: 触发 playful movement、find return、drop settle 或 tiny patrol，观察 gentle / playful / settle 三类 motion profile 的差异。

Expected: Ian 起步、移动和停下有身体感；playful 更活泼，settle 更像缓停安顿，gentle 更低打扰。

Fail condition: 所有移动都像直线 UI 平移；快速连续移动造成队列堆积、跳屏或位置丢失。

Degrade check: reduced motion 下倾斜、回弹、移动幅度和强动效明显降低。

## SDD 0273 Cursor Tease And Chase V2

Trigger: 鼠标从 Ian 附近移开，或用诊断触发 pointer curiosity / pointer chase 对应路径。

Expected: Ian 有注意、犹豫、追两步、收尾；追随距离和次数有上限，不追出安全区，也不会一直黏着鼠标。

Fail condition: Ian 机械直线追踪鼠标、跳屏、跑丢、无收尾，或持续追踪全局鼠标轨迹。

Degrade check: DND、quiet、reduced motion、用户拖动中，追鼠标被抑制或变成低动效短反馈。

## SDD 0274 Find Ian Peek And Return Story

Trigger: 把 Ian 移到不易发现的位置，然后触发找回 Ian 快捷键、托盘或设置中的找回入口。

Expected: Ian 回到当前可见 safe zone，并呈现 peek / enter / settle 的短故事；最终停在可拖动、可交互位置。

Fail condition: Ian 瞬移得突兀、被菜单栏或 Dock 遮挡、连续找回闪烁或堆叠、落点不可拖动。

Degrade check: reduced motion 下使用单段温和返回和短提示，不做大幅入场。

## SDD 0275 Drag Body Language V2

Trigger: 真实桌面连续拖动 Ian 至少 5 次，覆盖短拖、长拖、快速拖和放到屏幕边缘附近。

Expected: 拖起时有 pickup / carry 身体语言，拖动中命中稳定，最终落点准确并可持久化。

Fail condition: 无法拖动、命中偏移明显、拖动中跳屏、放手后位置和视觉状态不一致，或 idle / chase Moment 打断拖动。

Degrade check: reduced motion 下仍可拖动，视觉姿态更克制，但不影响最终位置。

## SDD 0276 Drop Comfort Settle V2

Trigger: 拖动 Ian 后放下，观察放下后的短 settle feedback；重复快速拖放至少 3 次。

Expected: Ian 有轻微安顿、短气泡或低幅效果，随后回到 idle；连续放下不叠加多个 settle story。

Fail condition: 放下后立即僵住、settle 太长、连续叠加、影响再次拖动，或改变最终持久化位置。

Degrade check: reduced motion 下 settle 降级为短气泡或低幅动画，不做明显回弹。

## SDD 0277 Idle Private Life Moments

Trigger: 用诊断分别触发 `idle_peek_around`、`idle_tiny_patrol`、`idle_pretend_innocent`；再进行 10-20 分钟低干扰观察。

Expected: Ian 偶尔像自己在生活：偷偷张望、小巡逻、装作无事；正式模式低频，动作短、安静、可打断。

Fail condition: 私生活动作过频、过长、抢占屏幕、用户输入中仍触发，或由 React 随机播放而不是 Core 决策。

Degrade check: DND、quiet、reduced motion、设置打开、拖动中会抑制或降级私生活 Moment。

## SDD 0278 Novelty And Repeat Avoidance

Trigger: 连续触发多个 private life Moment 和常见气泡反馈，观察相邻 kind / variant / phrase 是否避免机械重复。

Expected: Core 记录短期低敏 ID；连续重复项被排除或降权；候选不足时仍有 fallback；诊断指定触发不受去重影响。

Fail condition: 连续多次完全相同动作和文案；Novelty 记录原始文本、鼠标轨迹、窗口信息或用户工作内容；去重导致 Ian 完全无动作。

Degrade check: DND、quiet、reduced motion 仍优先于 Novelty，Novelty 不能绕过冷却、预算和用户控制。

## 体验观察记录模板

- 环境：桌面端版本、分辨率、是否开启 Accessibility。
- 已验收条目：列出 SDD 编号和通过 / 失败 / 未能自动化。
- 主观观察：只写 Ian 行为感受，例如“追鼠标有停顿但收尾偏短”，不要记录外部屏幕内容。
- 剩余问题：按影响生命感排序。
- 下一轮优先级：最多 3 项，必须能拆成 SDD。

## 当前重点风险

- 无 Accessibility 权限时，拖动、点击诊断和真实鼠标离开只能人工验收。
- idle private life 的 long-run 频率需要真实使用时间观察，单元测试只能证明冷却和预算。
- Motion feel 的“好不好看”仍需结合桌面视觉观察，不能只靠 action 序列通过。
