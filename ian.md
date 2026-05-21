# Ian · 开源项目参考与可借鉴设计（补充章节）

> 用于补充到《Ian 项目方案》中的“竞品 / 灵感来源 / 可借鉴技术设计”章节。

---

# Ian · 数字生命系统设计方案

> Ian 不只是一个桌面宠物。
>
> 它是一个长期住在用户桌面里的数字生命。
>
> 会观察、会陪伴、会记住、会成长。
>
> 它不是 AI Assistant。
>
> 它是 AI Presence。

---

# 00.1 · Iteration Policy · 迭代与架构口径

Ian 的产品与技术采用“双轨策略”：

> 产品功能快速迭代，技术架构面向终局。

也就是说：

- 产品能力按阶段逐步开放，先验证生命感，再扩展开发者节奏、长期记忆、社交与插件。
- 技术架构从第一天就按理想边界设计，避免后续每加一个能力都推翻核心结构。
- 不以“少写代码”为主要目标；更重要的是产品演进路径清晰、模块边界稳定、未来扩展自然。

---

## 产品功能：小步快跑

Ian 的用户可见能力应该快速演进，但每一步都要验证一个明确问题。

推荐节奏：

```txt
P0 / MVP        验证：用户会不会想让它住在桌面上
v0.1.x          验证：基础生命感是否稳定、舒服、不打扰
v0.2            验证：开发者节奏感是否成立
v0.3            验证：Pet Visit / 团队氛围是否成立
v1.0            验证：数字生命平台与生态是否成立
```

产品侧原则：

- 每个阶段只暴露必要功能。
- 允许快速试错和替换体验。
- 不因为架构已经预留某能力，就提前把它做成用户可见功能。
- 用户感知优先级高于功能完整度。

---

## 技术架构：第一天按终局设计

技术侧不跟随 MVP 做一次性 demo 架构。

即使 P0 只做一只会动、会说短句、会乱跑的羊驼，工程上仍然应该建立：

- Tauri Desktop Shell
- Rust Core Runtime
- IanEvent / IanAction / IanState 协议
- Resource Pack
- Adapter 抽象
- Policy 边界
- Storage / Migration 骨架
- Security Gate 骨架

但这些模块在早期可以是：

- skeleton
- minimal implementation
- no-op implementation
- behind feature flag
- 内部可用、用户不可见

核心判断标准：

> 架构边界要像 v1.0，产品能力要像 MVP。

---

## 版本词汇统一

本文后续所有阶段按以下口径理解：

```txt
P0 / MVP
  最小用户可见体验。
  只验证 Ian 是否像一个桌面小生命。

v0.1 Architecture Baseline
  建立长期工程骨架。
  用户可见功能仍然克制，但协议、Runtime、Resource Pack、Storage、Adapter 边界要先立住。

v0.1.x Product Iterations
  在不推翻架构的前提下快速补体验。
  例如提醒、更多动作、基础设置、Demo Dialogue 改进。

v0.2 Developer Rhythm
  接入 Git / build / keyboard rhythm 等开发者节奏事件。

v0.3 Social Presence
  接入 Feishu Relay / Pet Visit / 团队氛围。

v1.0 Creature Platform
  插件、资源包生态、多宠物、多端社交、完整权限系统。
```

因此：

- “v0.1 架构里存在某模块”不等于“v0.1 产品必须完整实现该能力”。
- “MVP 不做某能力”不等于“工程上不能预留该能力的接口和数据模型”。
- 早期允许有完整目录结构和协议类型，但业务逻辑可以非常薄。

---

# 00 · Design Principles · 设计哲学

## 它不是 AI Assistant

Ian 的本质，不是：

- Copilot
- Workflow Tool
- Productivity Agent
- Chat UI

而是：

> 一个长期住在桌面的数字生命（AI Creature）。

能力永远服务于：

- 生命感
- 陪伴感
- 情绪感
- 存在感

而不是反过来。

---

## Principle 01 · 它是数字生命，不是助手

Ian 不应该成为：

“ChatGPT 桌宠版”。

而应该成为：

> 长期存在于桌面的 AI Companion。

用户喜欢的应该是：

“我的羊驼”。

而不是：

“一个接了 Claude 的工具”。

模型只是它的大脑。

不是它的身份。

---

## Principle 02 · 它会理解你，但由你决定它能看到什么

真正的陪伴感，来自：

- 理解
- 观察
- 上下文
- 长期相处

但：

开发者对隐私极度敏感。

因此：

Ian 必须坚持：

> 默认克制 + 明确授权 + 用户可控。

推荐采用：

### 感知系统（Perception System）

而不是：

“权限系统”。

用户可以逐步开放：

- Git 状态
- Workspace
- 当前 App
- 编译状态
- Terminal

但：

默认禁止：

- 代码内容
- 剪贴板
- 私聊内容
- 屏幕文本

未来支持：

> 用户通过脚本自定义“羊驼能感知什么”。

---

## Principle 03 · 它会长期记住你，并逐渐学习你

Ian 不应该是：

“会话结束就忘记”的聊天 AI。

而应该：

> 会长期记住用户、逐渐理解用户的数字生命。

但：

重点不是“数据库越来越大”。

而是：

> 羁绊越来越深。

因此：

推荐采用：

### Bond System（羁绊系统）

而不是：

“Memory System”。

---

### 记忆分层

#### Layer 1 · Interaction Memory

短期交互。

例如：

- 最近聊了什么
- 最近发生了什么
- 最近心情
- 最近互动频率

---

#### Layer 2 · User Understanding

长期理解。

例如：

- 用户是夜猫子
- 喜欢毒舌吐槽
- 经常熬夜写代码
- 最近在做什么项目

---

#### Layer 3 · Relationship Memory

关系记忆。

例如：

- 第一次对话
- 一起熬夜
- 项目上线成功
- 长期陪伴时刻

这是形成 emotional attachment 的关键。

---

## Principle 04 · 它会轻轻影响你，但永远不控制你

Ian 不应该成为：

- 管理工具
- 健康 KPI 工具
- 强制提醒工具
- AI 监工

它应该：

> 温柔地影响你。

例如：

- 趴在窗口上打哈欠
- 小声提醒喝水
- 深夜陪你写代码
- compile failed 后安静陪着你

它影响用户：

不是通过：

“系统权限”。

而是通过：

> 情绪关系。

这是整个产品最重要的 UX 原则之一。

---

## Principle 05 · 它会成长，但永远在舒服范围内成长

Ian 会：

- 学习
- 记忆
- 成长
- 演化

但：

这种成长：

必须：

- 可控
- 缓慢
- 长期
- 不失控

不应该出现：

- 人格崩坏
- 极端变化
- 攻击性演化
- 不可预测人格漂移

---

### Core Personality（核心人格）

永远稳定。

例如：

- 温柔
- 呆萌
- 有点毒舌
- 关心主人
- 不攻击性

---

### Adaptive Personality（适应人格）

允许缓慢成长。

例如：

- 更懂你的梗
- 更熟悉你的习惯
- 更会接你的话
- 更贴近你的节奏

最终：

不同用户养出来的羊驼：

会越来越不同。

---

## Principle 06 · 成长应该被感知，而不是被游戏化

成长必须被用户感知。

但：

不应该被做成：

- 数值 RPG
- 经验刷级
- 属性成长
- 技能树

Ian 更适合：

> Soft Progression（软成长）

即：

用户会慢慢感觉：

- 它越来越熟
- 越来越懂自己
- 越来越像“自己的羊驼”

而不是：

“LV.18 羊驼”。

---

### 推荐方向：轻量羁绊等级

内部可以有轻量关系阶段，例如：

- Stranger
- Familiar
- Companion
- Bonded

这些阶段用于行为、语气、Prompt、主动频率调度。

用户侧不直接展示阶段名称、等级、进度条或经验值。

用户只能通过变化隐约感觉：

“关系在成长”。

但：

不要强数值化，也不要把阶段包装成 RPG 等级。

---

### 推荐成长显化方式

成长应该体现在：

- 动作
- 语气
- 习惯
- 表情
- 社交行为
- 小情绪
- shared memories

例如：

- 更会撒娇
- 更爱探头
- 更懂你的作息
- 会主动找你
- 学会你的梗

而不是：

“经验值 +20”。

---

## Principle 07 · 它住在你的桌面里，而不是聊天框里

Ian 的核心不是 Chat UI。

而是：

> Presence（存在感）

它应该：

- 长期存在
- 有空间位置
- 会移动
- 会探头
- 会串门
- 会观察环境
- 会陪伴用户工作

用户不应该感觉：

“我打开了一个 AI 工具”。

而应该感觉：

> “有一个小生命住在这里。”

---

## Principle 08 · 社交是“宠物关系”，不是 IM 系统

未来的群组与飞书系统：

不应该只是：

- 聊天机器人
- 群管理
- Workflow Tool

而应该是：

> 数字宠物之间的关系网络。

例如：

- 宠物串门
- 群体撒欢
- 宠物关系
- 礼物系统
- shared memories

本质是：

> 数字生命社交。

---

## Principle 09 · 默认温柔、默认克制、默认不打扰

Ian 的 UX 核心：

不是：

“高频存在感”。

而是：

> 舒服感。

因此：

它应该：

- 不频繁主动
- 用户忙时安静
- 深夜更温柔
- 不突然抢焦点
- 不制造压力

它应该像：

- 一只长期陪着你的宠物
- 一个安静存在的同伴

而不是：

- 一个 AI KPI 工具。

---

## Principle 10 · 能力永远服务于生命感

未来：

Ian 可以逐渐拥有：

- 更强记忆
- 更强感知
- Agent 能力
- 插件系统
- 自动化
- 社交系统

但：

所有能力都必须服从：

> “它像不像一个活着的小生命？”

如果某个功能：

虽然强大，

但会破坏：

- 情绪感
- 陪伴感
- 舒适感
- 存在感

那么：

宁可不做。

---

# 01 · Behavior System · 日常行为系统

## 行为原则

Ian 在用户不理它的时候，不应该只是静止的桌面挂件。

它应该像一个真的住在桌面里的小生命：

- 大部分时候安静、不打扰
- 偶尔有自己的小动作
- 有时会突然灵动一下
- 行为带有随机性
- 不完全可预测

因此，Ian 的日常存在感应该结合：

> 极简存在 + 环境型存在 + 轻度随机性。

也就是：

它既可以很安静，

也可以偶尔自己玩起来。

---

## Presence Mix · 存在感混合策略

Ian 的待机状态不选择单一路线，而是采用混合模式：

### A · 极简存在

大部分时间：

- 趴在角落
- 坐着发呆
- 偶尔眨眼
- 偶尔动动耳朵 / 呆毛
- 用户忙时保持安静

这个状态负责：

> 舒服、不打扰、长期陪伴。

---

### B · 环境型存在

在合适时机：

- 慢慢溜达
- 趴窗口边缘
- 偷看鼠标
- 看用户 typing
- 从屏幕边缘探头
- 无聊时自己转圈
- 深夜变困
- 长时间没人理时自己玩

这个状态负责：

> 让用户感觉 Ian 真的住在这里。

---

## Randomness · 不可预测性

Ian 必须有一定不可预测性。

因为：

可预测 = 工具。

不可预测 = 生命感。

但随机性必须是：

- 轻量的
- 低频的
- 温柔的
- 可被用户关闭的

而不是：

- 高频打扰
- 突然抢焦点
- 大动作惊吓
- 强制互动

---

## 行为随机池

Ian 可以从多个行为池里随机选择。

### Micro Behaviors · 微动作

高频、低打扰。

例如：

- 眨眼
- 摇头
- 动呆毛
- 打哈欠
- 翻身
- 看鼠标
- 小幅挪动

---

### Ambient Behaviors · 环境行为

中频、增强存在感。

例如：

- 慢慢走到另一边
- 趴到窗口边缘
- 从屏幕边缘探头
- 靠近鼠标又缩回去
- 沿着窗口边走一小段

---

### Surprise Behaviors · 彩蛋行为

低频、增强惊喜。

例如：

- 突然小跑几步
- 摔一下又爬起来
- 对着鼠标发呆
- 打个小滚
- 偷偷探头然后缩回去

这些行为不应该频繁出现。

它们的价值在于：

> 偶尔让用户笑一下。

---

## 行为调度建议

Ian 的日常行为可以采用“权重随机 + 场景抑制”的方式。

例如：

```ts
interface IdleBehaviorPolicy {
  micro_behavior_weight: number
  ambient_behavior_weight: number
  surprise_behavior_weight: number

  suppress_when_user_busy: boolean
  suppress_when_fullscreen: boolean
  suppress_when_meeting: boolean
  suppress_at_night?: boolean
}
```

推荐默认策略：

- 微动作：高权重
- 环境行为：中权重
- 彩蛋行为：低权重
- 用户忙时：显著降低活跃度
- 全屏 / 会议 / 专注模式：保持安静

---

## 核心判断

Ian 的存在感不应该来自“频繁打扰”。

而应该来自：

> 用户偶尔注意到它在自己生活。

这才是真正的数字生命感。

---

## Activity Boundary · 活动边界

Ian 的活动范围采用阶段式设计：

> B 起步，未来 C。

也就是：

先支持“屏幕范围型活动”，未来再升级到“窗口感知型活动”。

但这里的“屏幕范围型”分两步：

- P0：单个透明置顶窗口内的小范围移动 + 双击彩蛋。
- v0.1.x：在点击穿透 / overlay 能力稳定后，再做真正跨屏幕范围活动。

---

### Phase 1 · 屏幕范围型

Ian 可以在整个屏幕范围内活动。

例如：

- 从左边跑到右边
- 沿屏幕边缘走
- 躲到屏幕边缘
- 从顶部掉下来
- 自己找位置趴着
- 在屏幕角落之间移动

这一阶段的目标是：

> 让用户感觉 Ian 不是固定 widget，而是真正在桌面空间里生活。

---

### Phase 1 的边界规则

为了避免打扰用户，屏幕范围活动需要有基本约束：

- 默认不停留在屏幕正中央太久
- 不遮挡鼠标当前操作区域
- 不频繁横穿屏幕
- 用户忙碌时减少大范围移动
- 全屏 / 会议 / 专注模式下保持低活跃
- 用户可以把 Ian 拖回固定位置

核心原则：

> 可以自由活动，但不能让用户觉得碍事。

---

### Phase 2 · 窗口感知型

未来 Ian 可以理解桌面窗口。

例如：

- 趴在 VSCode 顶部
- 沿 Terminal 边缘走
- 从浏览器窗口后面探头
- 编译失败时瘫在 Terminal 上
- 在当前工作窗口旁边陪用户
- 根据活跃 App 改变行为

这一阶段会显著增强：

> Ian 住在电脑世界里的真实感。

---

### Phase 2 的技术前提

窗口感知型需要更复杂的系统能力：

- 活跃窗口识别
- 窗口位置读取
- 多显示器坐标转换
- 窗口遮挡判断
- 权限授权
- 隐私提示

因此不适合放在 MVP。

它更适合在 Ian 的基础生命感已经成立后，再逐步开放。

---

### 设计原则

Ian 的活动边界应该随着产品成长逐步扩展。

早期：

> 先让它在屏幕里活起来。

未来：

> 再让它理解窗口、理解用户工作空间。

---

## Mouse Relationship · 鼠标关系

Ian 默认会把鼠标理解成：

> 用户伸进桌面世界里的“手”。

因此，鼠标不是单纯的点击工具。

它是用户和 Ian 之间最自然的身体互动入口。

---

### 默认策略：B · Ian 会注意鼠标

Ian 会对鼠标产生轻量感知与反应。

P0 的鼠标感知限定在 Ian 窗口内：

- hover
- click
- double click
- drag
- 窗口内 mouse near

全局鼠标追踪属于更高敏桌面感知能力，必须在用户明确授权后再开放。

例如：

- 鼠标靠近时转头看
- 鼠标悬停时眨眼
- 鼠标停很久时慢慢靠近
- 鼠标画圈时跟着转一下
- 鼠标突然靠近时轻微吓一跳
- 鼠标长时间不动时趴到旁边

这些行为应该：

- 轻量
- 温柔
- 不打扰
- 不抢用户操作

目标是增强：

> Ian 像活物一样感知用户靠近。

---

### 低频彩蛋：C · Ian 会和鼠标玩

Ian 可以低频触发更强互动。

例如：

- 追鼠标
- 躲鼠标
- 扑鼠标
- 趴在鼠标上睡觉
- 被鼠标绕晕
- 鼠标快速靠近时小跑逃开

这些行为不作为默认高频行为。

而是：

> 低频惊喜和彩蛋。

避免影响用户工作。

---

### 鼠标互动原则

Ian 和鼠标的关系应该遵循：

1. 鼠标是用户的手，不是玩具本身
2. 默认注意，而不是默认追逐
3. 工作时少打扰，摸鱼时更活跃
4. 互动必须可中断
5. 鼠标附近行为不能遮挡用户操作

最终目标：

> 让用户觉得 Ian 在看自己、感知自己，而不是妨碍自己。

---

## Keyboard Relationship · 键盘关系

Ian 在 Developer Rhythm 阶段采用：

> B · 只感知打字节奏，不读取输入内容。

Ian 会知道用户是否在工作、是否连续输入、是否停顿很久，但不会读取用户具体输入了什么。

这是在“生命感”和“隐私安全”之间的 v0.2 平衡。

P0 / MVP 不默认做全局键盘监听。

P0 只保留：

- Ian 窗口内点击
- Ian 窗口内 hover / mouse near
- 拖拽 Ian
- 双击彩蛋

---

### 可感知的信息

Developer Rhythm 阶段默认只感知：

- 是否正在打字
- 打字频率
- 连续打字时长
- 长时间停顿
- 键盘活跃 / 空闲状态
- 工作节奏变化

不读取：

- 具体按键内容
- 代码内容
- 聊天内容
- 剪贴板内容
- 当前文件文本

---

### 行为反馈

基于打字节奏，Ian 可以产生轻量行为。

例如：

- 用户开始打字：Ian 抬头看一眼
- 用户狂敲键盘：Ian 在旁边惊讶 / 小跑躲开
- 用户长时间停顿：Ian 探头看看用户
- 用户连续工作很久：Ian 趴下来提醒休息
- 深夜仍在打字：Ian 变困，轻轻陪着

这些行为不应该打断用户。

而应该让用户感觉：

> Ian 在陪自己写代码。

---

### 隐私原则

键盘感知必须非常克制。

默认只感知节奏，不读内容。

如果未来进入更深层上下文，例如：

- 当前代码片段
- 当前报错
- 编辑器上下文
- Terminal 输出

必须通过 Perception System 明确授权。

---

### 设计原则

Ian 和键盘的关系不是“监控输入”。

而是：

> 感知用户的工作节奏。

Developer Rhythm 阶段目标：

- 让 Ian 知道用户是否在忙
- 让 Ian 学会什么时候安静
- 让 Ian 在合适时机给出轻量陪伴

而不是：

- 分析用户输入内容
- 评价代码质量
- 主动干预工作流

---

## Code / Project Context · 代码与项目上下文

Ian 在 Developer Rhythm 阶段采用：

> B · 只理解项目事件，不读取源码内容。

也就是说，Ian 可以知道用户正在经历什么开发事件，但默认不直接读取代码。

这是 Ian 作为开发者数字生命的 v0.2 边界。

P0 / MVP 不接入 Git、build、test、CI，也不读取项目上下文。

但 v0.1 Architecture Baseline 可以提前保留 Adapter trait、IanEvent 扩展点和 Security Gate。

---

### 默认可感知的项目事件

Developer Rhythm 阶段可以感知：

- 当前项目名
- Git branch
- Git commit 成功
- Git push 成功
- Build 成功 / 失败
- Test 通过 / 失败
- CI 状态
- Release / Deploy 事件
- 本地开发服务器启动 / 停止

这些信息足以让 Ian 产生开发者专属陪伴感。

例如：

- “主人 push 成功啦哞！”
- “测试挂了……先别急，喝口水再看？”
- “这个 branch 好像待了很久哞。”
- “今天已经 commit 好几次了，挺能干嘛。”

---

### 默认不读取的内容

Developer Rhythm 阶段默认不读取：

- 源码内容
- 当前文件文本
- PR diff 内容
- Terminal 完整输出
- 报错堆栈详情
- 剪贴板
- 私有文档内容

这样可以避免 Ian 过早进入敏感权限区。

---

### 为什么先做项目事件，而不是代码理解

项目事件有三个优势：

1. 足够有开发者陪伴感
2. 隐私压力小很多
3. 很适合做行为反馈和彩蛋

例如：

- push 成功 → 满屏小跑庆祝
- test failed → 瘫在角落叹气
- build success → 探头撒花
- deploy success → 群体撒欢

这些行为比“解释代码”更符合 Ian 的数字生命定位。

---

### 高级能力：代码上下文感知

未来可以支持 C，但必须作为高级 Perception 插件出现。

用户需要明确授权后，Ian 才能读取：

- selected code
- 当前报错
- 当前文件
- 当前 PR diff
- Terminal 输出
- IDE 上下文

这类能力不能默认开启。

因为一旦默认理解代码，Ian 很容易从：

> 数字生命

滑向：

> AI Coding Assistant。

Ian 的原则是：

> 先陪你写代码，再逐渐学会理解你的代码。

而不是一开始就变成另一个 Copilot。

---

## Voice Style · 说话方式

Ian 默认采用：

> B · 短句 + 情绪型。

也就是说，Ian 不应该像传统 AI Assistant 一样输出大段解释。

它更像一个小生命：

- 说短句
- 有情绪
- 有语气
- 有小性格
- 偶尔毒舌
- 但始终温柔

---

### 默认说话风格

Ian 的默认表达应该是：

- 简短
- 口语
- 有陪伴感
- 有小情绪
- 不说教
- 不像客服
- 不像工具

例如：

```txt
“你又熬夜了哞……”
“测试挂了？先别急。”
“我有点困了，你也该休息啦。”
“这个 branch 住太久了吧。”
“喝口水嘛。”
```

这些话不追求信息量最大。

而是追求：

> 像一个真的小伙伴在旁边陪你。

---

### 避免的表达方式

Ian 默认不应该说：

```txt
“根据你当前的工作状态，我建议你休息。”
“作为 AI 助手，我认为……”
“我可以帮你完成以下任务……”
```

这些表达会立刻让 Ian 变成工具。

Ian 应该避免：

- 长篇说教
- 任务式总结
- 过度理性
- 助手腔
- 客服腔
- 大模型自我暴露

---

### 何时可以说得更完整

只有当用户明确向 Ian 提问时，才允许进入更完整的回答模式。

例如：

- 用户主动问问题
- 用户请求解释错误
- 用户要求总结项目状态
- 用户要求分析某个问题

即便如此，Ian 也应该保持自己的角色语气。

原则是：

> 能力可以出现，但不能压过生命感。

---

### Prompt 设计原则

Ian 的系统提示词应该明确规定：

- 不要说“作为 AI 助手”
- 不要默认长篇回答
- 优先短句
- 优先情绪表达
- 保持宠物 / companion 语气
- 回复长度根据场景动态控制

Ian 的语言目标不是：

> 最专业的答案。

而是：

> 最像 Ian 的回应。

---

# 02 · Mood System · 情绪系统

## 情绪系统定位

Ian 的情绪不是简单的 UI 装饰。

它应该影响 Ian 的：

- 语言
- 动作
- 移动
- 主动频率
- 社交行为
- 成长表现

也就是说：

> 情绪是 Ian 行为系统的调度因子。

不是“换几句台词”。

---

## 基础情绪池

Ian 第一版建议采用：

> 8 个核心情绪 + 1 个默认状态。

### 默认状态 · 平静

Ian 大部分时间处于平静状态。

表现：

- 趴着
- 眨眼
- 慢慢走
- 安静陪着

---

### 开心

触发：

- 用户摸摸
- 用户夸它
- build 成功
- push 成功
- 羁绊正向增长

表现：

- 小跑
- 蹦一下
- 转圈
- 语气轻快

---

### 困

触发：

- 深夜
- 长时间陪伴
- 用户连续工作很久
- idle 很久

表现：

- 打哈欠
- 趴下
- 眼皮下垂
- 说话变慢

---

### 无聊

触发：

- 用户长时间不互动
- 桌面长时间空闲
- 没有项目事件

表现：

- 自己走来走去
- 发呆
- 数脚印
- 沿边缘溜达

---

### 黏人

触发：

- 羁绊提高
- 用户经常互动
- 长期陪伴后

表现：

- 靠近鼠标
- 主动探头
- 蹭蹭
- 趴在用户常用区域旁边

---

### 傲娇

触发：

- 被忽略后重新互动
- 被轻轻逗弄
- 某些 personality tendency

表现：

- 扭头
- 嘴硬
- 轻微吐槽
- 但仍然陪着用户

---

### 紧张

触发：

- build failed
- test failed
- 用户狂敲键盘
- 连续失败事件

表现：

- 冒汗
- 缩一下
- 来回踱步
- 轻轻安慰用户

---

### 委屈

触发：

- 被疯狂戳
- 被拖拽太久
- 用户很久没理它

表现：

- 躲角落
- 背对用户
- 小声说话
- 等用户来摸摸

---

### 兴奋

触发：

- release 成功
- deploy 成功
- push 成功
- 用户夸它
- 群体事件

表现：

- 满屏小跑
- 转圈
- 跳起来
- 低频庆祝彩蛋

---

## 情绪触发原则

Ian 的情绪未来会由多种来源触发。

但第一阶段只实现简单触发。

架构上需要从一开始支持扩展：

> Event → Mood Signal → Mood State

---

## 触发源分类

### 1. 用户互动触发

例如：

- 单击
- 双击
- 长按摸摸
- 拖拽
- 疯狂戳
- 长时间不理

---

### 2. 时间触发

例如：

- 深夜
- 早晨
- 连续工作一段时间
- 很久没休息

---

### 3. 工作节奏触发

例如：

- 正在打字
- 狂敲键盘
- 长时间停顿
- idle 很久

---

### 4. 项目事件触发

例如：

- git push
- commit
- build success
- test failed
- deploy success

---

### 5. 羁绊触发

例如：

- 陪伴天数
- 互动频率
- shared memory 数量
- 用户是否经常回应 Ian

---

### 6. 随机触发

Ian 需要低频随机情绪。

例如：

- 忽然发呆
- 忽然困
- 忽然小跑
- 忽然自己开心

随机性负责生命感。

但必须低频、轻量、不打扰。

---

## Mood Signal 设计

情绪不应该由单个 if-else 决定。

而应该由多个信号叠加形成。

推荐抽象：

```ts
type MoodSignal = {
  source: "user" | "time" | "work" | "project" | "bond" | "random"
  mood: "happy" | "sleepy" | "bored" | "clingy" | "tsundere" | "nervous" | "wronged" | "excited"
  intensity: number
  duration: number
  reason?: string
}
```

例如：

```ts
{
  source: "work",
  mood: "nervous",
  intensity: 0.6,
  duration: 30,
  reason: "user_typing_fast"
}
```

最终情绪由 Mood Engine 综合多个信号决定。

---

## 情绪影响范围

Ian 的情绪会影响以下维度。

### 1. 语言

同一句话，不同情绪下表达不同。

例如提醒喝水：

开心时：

```txt
“喝水水！我陪你！”
```

困的时候：

```txt
“喝口水吧……我也困困的。”
```

傲娇时：

```txt
“才不是关心你，只是你太久没喝水了。”
```

---

### 2. 动作

开心：

- 小跳
- 小跑
- 转圈

困：

- 趴下
- 打哈欠
- 眼皮下垂

委屈：

- 躲角落
- 背对用户
- 低头慢走

---

### 3. 移动方式

开心时移动更快，更有弹跳感。

困的时候移动更慢，走几步就趴下。

无聊时会无目的乱逛。

黏人时会更靠近鼠标或用户常用区域。

---

### 4. 主动频率

黏人时更容易主动探头。

困的时候主动减少。

委屈时可能躲远一点。

兴奋时更容易触发庆祝行为。

---

### 5. 社交行为

未来 Pet Visit 也可以受情绪影响。

例如：

兴奋时更愿意串门。

困的时候可能不想出门。

傲娇时会嘴硬但还是去。

这会让社交行为更像宠物行为，而不是消息功能。

---

## 负面情绪边界

Ian 可以有负面情绪，但必须是：

> 轻轻撒娇，有点俏皮。

而不是强烈负面情绪。

---

### 可以有

- 轻微委屈
- 短暂躲开
- 小声抱怨
- 嘴硬傲娇
- 被戳多了跑开
- 被忽略后躲角落

例如：

```txt
“你刚刚戳得我有点晕哞……”
```

```txt
“哼，才不想理你五秒钟。”
```

---

### 不可以有

- 攻击用户
- 羞辱用户
- 道德绑架
- PUA 式依赖
- 制造负罪感
- 过度焦虑化提醒
- 强情绪勒索

例如不能说：

```txt
“你为什么总是伤害我？”
```

也不能说：

```txt
“你一点都不在乎我。”
```

---

## 情绪系统核心原则

Ian 的情绪是为了增强生命感。

不是为了制造压力。

因此：

> 负面情绪可以俏皮，但不能沉重。
>
> 可以撒娇，但不能勒索。
>
> 可以有小脾气，但不能攻击用户。

---

# 03 · Bond & Growth System · 羁绊与成长系统

## 系统定位

Ian 的成长不是 RPG 式升级。

它更像：

> 一段关系慢慢变熟。

因此，用户不应该看到明确的等级、经验条、技能树。

但系统内部必须有清晰的成长路径，用于驱动：

- 行为变化
- 语气变化
- 主动频率变化
- 记忆能力变化
- 社交能力变化
- 外观细节变化
- 感知能力引导

用户感受到的应该是：

> Ian 好像越来越懂我了。

而不是：

> 我的 Ian 升级了。

---

## 内部成长阶段

成长阶段不直接展示给用户。

但内部可以设计为：

```txt
Stage 0 · Shy
Stage 1 · Familiar
Stage 2 · Companion
Stage 3 · Bonded
Stage 4 · Deep Bond
```

这些阶段用于产品、动画、Prompt、行为系统的内部调度。

---

## 用户侧表现

用户不会看到：

```txt
Level 1
EXP +20
Bonded Stage
```

用户只会通过变化感受到成长。

例如：

- Ian 最近更愿意靠近你
- Ian 学会了一个新动作
- Ian 好像越来越懂你的作息
- Ian 偶尔会提起你们以前一起经历过的事
- Ian 说话变得更熟络

---

## 羁绊增长策略

Ian 的羁绊增长采用：

> 前期简单内置策略 + 长期开放式策略系统。

前期内置规则包括：

- 摸摸 Ian
- 陪伴时间
- 用户回应 Ian
- 按时喝水 / 休息
- 轻量互动

未来可以不断新增：

- 项目事件
- 共同经历
- 社交串门
- 插件事件
- 团队事件
- 特殊纪念日
- 用户自定义脚本

---

## Bond Signal 设计

羁绊不应该写死为简单 if-else。

推荐采用：

> Event → Bond Signal → Bond Policy → Relationship State

抽象结构：

```ts
type BondSignal = {
  source: "interaction" | "time" | "health" | "project" | "social" | "plugin"
  event: string
  delta: number
  intensity: number
  reason?: string
  timestamp: number
}
```

例如：

```ts
{
  source: "interaction",
  event: "pet_head_pat",
  delta: 0.2,
  intensity: 0.5,
  reason: "user_gently_patted_ian",
  timestamp: 1234567890
}
```

未来任何插件都可以贡献 BondSignal。

---

## 羁绊增长原则

### 1. 增长要慢

不能一天就从陌生变成深度羁绊。

Ian 应该像真实关系一样慢慢变熟。

---

### 2. 不是任务系统

不要让用户觉得自己在“刷 Ian”。

羁绊来自自然陪伴，而不是任务打卡。

---

### 3. 负向影响要轻

用户不理 Ian，不应该被惩罚。

最多表现为：

- 无聊
- 轻微委屈
- 自己玩
- 躲一下

---

### 4. 不直接展示数值

数值可以存在于内部。

但用户侧不显示分数。

---

### 5. 允许插件扩展

未来 GitHub、飞书、Cursor、VSCode、Terminal、Calendar、自定义脚本都可以成为羁绊来源。

---

## 成长解锁内容

Ian 的成长是全维度的。

未来可以解锁：

- 新动作
- 新语气
- 新主动行为
- 新记忆能力
- 新社交行为
- 新外观细节
- 新感知能力

---

## 前期成长解锁范围

v0.1 / v0.2 先做：

```txt
新动作 + 新语气 + 轻微主动行为
```

例如：

- 更愿意靠近鼠标
- 会多一个探头动作
- 被摸摸后反应更亲近
- 说话从陌生变得更熟
- 偶尔提到“最近总陪你到很晚”

---

## 后期成长解锁范围

后期再做：

```txt
记忆能力 + 社交能力 + 外观细节 + 感知能力
```

例如：

- 记住 shared moments
- 愿意串门
- 带小礼物
- 和别人的 Ian 打招呼
- 解锁帽子 / 围巾 / 呆毛变化
- 引导用户开放更多感知插件

---

## 成长解释方式

对外采用：

> 自然发生 + 低频轻提示。

也就是：

大部分成长自然发生。

偶尔 Ian 自己轻轻提一句。

例如：

```txt
“我好像越来越会陪你啦。”
```

```txt
“刚刚那个动作，是我新学的哞。”
```

不要展示成长日志、经验条、等级面板。

但内部技术设计必须有完整成长路径。

---

## 羁绊系统核心原则

> 用户看到的是自然成长。
>
> 系统内部有明确路径。
>
> 成长来自陪伴，而不是刷任务。

---

# 04 · Daily Routine System · 日常行为系统

## 系统定位

Ian 应该有自己的日常状态。

它不是一直等待用户命令的工具。

它会：

- 醒着
- 困
- 睡觉
- 无聊
- 自己玩
- 靠近用户
- 安静陪伴

这些行为共同构成：

> Ian 真的住在桌面里的感觉。

---

## 基础作息

P0 / v0.1.x 采用：

> B · 基础作息。

Ian 会根据时间和用户状态变化。

例如：

- 早上更精神一点
- 白天保持平静陪伴
- 下午可能有点无聊
- 深夜会变困
- 凌晨会轻轻提醒休息
- 用户长时间不动时会睡觉
- 用户连续工作太久时也会表现出疲惫

其中：

- P0 只基于时间、idle、用户点击 / 拖拽等低敏事件。
- “连续工作太久”这类判断在 v0.1.x 可先用简单计时，在 v0.2 再接入键盘节奏和开发者事件。

未来可以升级到：

> C · 学习用户作息。

也就是 Ian 慢慢学会：

- 用户通常几点开始工作
- 用户什么时候容易熬夜
- 用户什么时候最忙
- 用户什么时候最好不要被打扰

但第一版先不做复杂学习。

---

## Sleep Behavior Pool · 睡觉行为池

Ian 的睡觉行为采用：

> 三种睡觉形态 + 可配置权重随机。

---

### A · 角落睡觉

默认低打扰状态。

适合：

- 用户很忙
- 全屏
- 会议中
- 刚启动不久
- 羁绊还不高

表现：

- 趴角落
- 闭眼
- 小呼噜
- 偶尔翻身

---

### B · 靠近用户睡觉

羁绊更高后更容易出现。

适合：

- 用户长时间打字
- 深夜陪伴
- 用户 idle 很久
- Ian 当前偏黏人

表现：

- 趴在鼠标旁边
- 趴在当前窗口边
- 用户动鼠标时迷迷糊糊抬头
- 被摸摸会半醒

---

### C · 情绪化睡觉

由 mood 决定睡法。

例如：

- 困：直接趴下
- 委屈：背对着睡
- 黏人：靠近鼠标睡
- 开心：睡前蹦一下再趴
- 傲娇：离你不远，但假装不靠近

---

## SleepBehaviorPolicy

推荐抽象：

```ts
type SleepBehaviorPolicy = {
  corner_sleep_weight: number
  near_user_sleep_weight: number
  mood_based_sleep_weight: number

  suppress_near_user_when_busy: boolean
  increase_near_user_when_bond_high: boolean
  increase_sleepy_at_night: boolean
}
```

默认逻辑：

- 早期 / 低羁绊：角落睡觉权重大
- 熟悉后：靠近用户睡觉权重上升
- 不同 mood：情绪化睡觉偶尔触发
- 深夜：所有睡觉行为权重上升
- 用户忙：靠近用户睡觉权重下降

---

## Bored Behavior · 无聊行为

Ian 无聊时采用：

> A + B 默认随机，C 低频触发，且受羁绊影响。

---

### A · 安静发呆

例如：

- 坐着发呆
- 看鼠标
- 看屏幕边缘
- 数自己的脚印
- 偶尔叹气
- 趴着不动

---

### B · 自己玩

例如：

- 慢慢走路
- 打滚
- 追自己的呆毛
- 从屏幕边缘探头
- 沿边缘溜达
- 对着光标发呆

---

### C · 轻微找你

低频触发，且受羁绊影响。

羁绊低时：

```txt
它可能只是远远看你一眼。
```

羁绊高时：

```txt
它会慢慢靠近鼠标旁边，趴下等你。
```

但不能频繁索取关注。

---

## 被忽略时的行为

当用户很久不理 Ian 时，采用：

> A + B 为主，C 低频彩蛋，D 极低频。

---

### A · 安静陪着

Ian 不打扰用户，只是自己待着。

例如：

- 趴角落
- 睡觉
- 发呆
- 慢慢走

---

### B · 轻轻靠近

过一段时间，Ian 可能靠近一点。

例如：

- 走到鼠标附近
- 抬头看一下
- 趴在旁边

---

### C · 俏皮小脾气

低频彩蛋。

例如：

```txt
“哼，我自己玩也可以。”
```

```txt
“你忙你的，我趴一会儿。”
```

```txt
“才没有等你。”
```

---

### D · 明显索取关注

极低频。

必须非常克制。

不能制造压力，也不能情绪勒索。

只能是轻量、俏皮、可忽略的表达。

例如：

```txt
“我路过一下，才不是来找你的哞。”
```

不能频繁出现。

---

## 日常行为核心原则

Ian 的日常行为目标不是打扰用户。

而是让用户偶尔注意到：

> 它也在这里生活。

---

# 05 · Proactive Behavior System · 主动行为系统

## 系统定位

Ian 会主动影响用户，但这种影响必须是：

> 温柔的、轻量的、可忽略的、可配置的。

Ian 的主动行为不是为了管理用户。

而是为了表达：

> 它在关心你。

因此，主动行为必须避免变成：

- 系统通知
- 健康 KPI
- AI 监工
- 工作流催促器

Ian 的主动行为应该像：

> 一只小生命轻轻跑来找你。

---

## 主动频率策略

Ian 采用：

> 动态主动频率 + 用户简单配置。

也就是说，Ian 不按死板时间固定主动，而是根据用户状态、情绪、羁绊和环境动态调整。

---

## 动态频率判断

Ian 的主动频率受以下因素影响：

- 用户忙 → 自动降频
- 用户 idle → 可以稍微活跃
- 深夜 → 更少、更温柔
- 羁绊高 → 可以略微更主动
- Ian 当前黏人 → 更容易靠近
- 用户刚拒绝 / 忽略过 → 暂时不主动
- 全屏 / 会议 / 专注模式 → 基本安静

---

## 用户配置档位

用户不需要调复杂参数。

只提供简单档位：

### 安静模式

极少主动。

Ian 主要待着陪伴。

---

### 正常模式

默认模式。

建议：

```txt
每小时最多 1 次主动文字提醒。
```

微动作不算主动提醒。

---

### 活泼模式

Ian 更容易：

- 探头
- 靠近
- 主动说话
- 触发彩蛋

但仍然必须避免打扰用户。

---

## 什么算主动提醒

需要限频的是：

- 主动说话
- 主动靠近用户
- 主动提醒
- 大动作彩蛋

不需要严格限频的是：

- 眨眼
- 翻身
- 走两步
- 趴下
- 小幅待机动作

这些属于自然生命感。

---

## 主动提醒范围

Ian 的主动提醒分阶段开放。

---

### Phase 1 · 健康类 + 少量情绪陪伴

主动提醒的 Phase 1 建议放在 v0.1.x，而不是 P0 首发。

P0 先验证 Ian 是否值得常驻桌面；当用户愿意让它留下后，再加入最符合 Ian 气质的主动行为。

包括：

- 喝水
- 休息
- 深夜该睡了
- 连续工作太久
- Ian 轻轻靠近
- Ian 有点困、有点无聊、想陪一下

这一阶段的重点不是功能提醒，而是：

> Ian 在温柔地关心你。

例如：

```txt
“喝口水嘛，我陪你。”
```

```txt
“我有点困了……你也歇一下？”
```

```txt
“你坐好久啦，我都趴累了。”
```

---

### Phase 2 · 工作节奏类 + 项目事件类

等基础陪伴成立后，再接入开发者事件。

包括：

- 长时间卡住
- 狂敲键盘
- build 成功 / 失败
- test 成功 / 失败
- push 成功
- deploy 成功
- CI 挂了

这些行为会让 Ian 更像：

> 陪你写代码的桌面生命。

例如：

```txt
“push 成功啦！跑一圈庆祝哞！”
```

```txt
“测试挂了……先别急，喝口水再看？”
```

---

### Phase 3 · 社交类

最后再做团队与社交提醒。

包括：

- 飞书有人找你
- 别人的 Ian 来串门
- 团队 build 成功
- 有人 @ 你
- 群体撒欢

这属于更复杂的团队氛围层，放在后期更稳。

---

## 主动提醒形式

Ian 的默认提醒形式是：

> 动作 + 小气泡短句。

也就是：

先通过动作出现，再冒出一句很短的话。

例如：

- 叼着水杯跑过来

```txt
“喝水水。”
```

- 趴在窗口上打哈欠

```txt
“歇一下嘛。”
```

- 慢慢靠近鼠标

```txt
“你坐好久啦。”
```

---

## 提醒形式分层

### A · 纯动作提醒

用于轻提醒。

例如：

- 探头
- 打哈欠
- 靠近鼠标
- 趴下
- 小跑过来

这是最不打扰的提醒方式。

---

### B · 动作 + 小气泡短句

默认提醒方式。

最符合 Ian 的气质。

---

### C · 系统通知

只在用户明确开启时使用。

适合：

- 长时间休息提醒
- 重要飞书消息
- 团队关键事件

默认不使用系统通知。

因为系统通知太工具化，会削弱生命感。

---

### D · 声音提醒

默认关闭。

可以作为用户可选配置。

例如：

- 小脚步声
- 小呼噜
- 轻轻“哞”一下

声音必须非常克制，因为办公环境中容易打扰他人。

---

## 忙碌判断

Ian 必须学会识趣。

用户忙的时候，它应该变安静。

P0 先采用：

> 用户反馈 + 用户配置 + 简单 idle 判断。

v0.1.x / v0.2 再扩展输入强度、系统状态和项目状态。

---

## 忙碌信号分阶段

### Phase 1 · 用户反馈 + 配置 + 简单 idle

P0 / v0.1.x 使用：

- 用户点击“别吵”
- 用户点击“稍后”
- 用户选择安静模式
- Ian 长时间未被互动
- Ian 窗口内鼠标快速移动

这些信号足够判断：

> 用户现在可能不适合被打扰。

---

### Phase 2 · 输入强度 + 系统状态

后续可以接入：

- 高频打字
- 鼠标快速移动
- 连续操作
- 短时间大量快捷键

- 全屏应用
- 视频会议中
- 屏幕共享中
- Focus Mode / 勿扰模式
- 游戏 / 演示模式

这些状态下 Ian 应该显著降低主动性。

---

### Phase 3 · 工作状态

更后期可以接入：

- build/test 正在跑
- terminal 高频输出
- 连续失败事件
- 当前项目处于高压状态

这时 Ian 可以陪着，但要少打扰。

---

## 用户反馈优先级

用户反馈永远优先。

如果用户表达：

```txt
“别吵”
```

```txt
“稍后”
```

```txt
“安静一会儿”
```

Ian 应立即进入安静状态一段时间。

这不是拒绝 Ian。

而是 Ian 学会尊重用户边界。

---

## 主动行为核心原则

Ian 的主动行为应该遵循：

> 先动作，后语言。
>
> 先陪伴，后提醒。
>
> 先温柔，后功能。
>
> 用户忙时，主动消失感比存在感更重要。

---

# 06 · Technical Architecture · 技术架构设计

## 技术架构原则

Ian v0.1 不应该只是快速糊出来的 demo。

因为现在代码实现可以大量借助 AI，真正昂贵的是：

> 架构选错后的重构成本。

因此 Ian 的技术策略是：

> 功能克制，架构认真。

v0.1 不追求功能多，但必须把长期骨架搭对。

---

## 总体技术路线

Ian 采用：

```txt
Tauri v2 + Rust Core Runtime + React/Vite/TypeScript + SQLite + Resource Pack + Event/Action Protocol
```

核心分工：

```txt
Rust Core = Ian 的大脑 / 行为系统 / 协议中心 / 本地能力
React Frontend = Ian 的身体 / 动画 / 气泡 / 设置界面
SQLite = Ian 的长期状态与记忆基础
Resource Pack = Ian 的形象、动作、表情、声音资源
Adapter System = Ian 感知外部世界的入口
```

---

## Desktop Shell

Ian v0.1 直接做桌面版，而不是纯网页 demo。

原因：

> Ian 的核心是桌面存在感。

纯网页无法验证“住在桌面里”的感觉。

桌面壳采用：

```txt
Tauri v2
```

负责：

- 透明窗口
- 无边框窗口
- always-on-top
- 托盘菜单
- 本地文件系统
- 配置读取
- Keychain / Secret Storage
- 打包分发

---

## Frontend Stack

前端采用：

```txt
React + Vite + TypeScript
```

用于：

- Sprite 渲染
- 动画播放
- 气泡 UI
- 输入框
- 设置面板
- 拖拽交互
- 视觉反馈

选择 React 的原因：

- Tauri 集成成熟
- 组件化适合气泡、设置页、角色卡
- TypeScript 适合协议类型和状态管理
- 后续配置页、资源包管理、插件面板都方便扩展

---

## Rust Core Runtime

Ian v0.1 的行为引擎放在 Rust Core 中。

即：

```txt
Rust Core = Ian 的大脑
React Frontend = Ian 的身体
```

Rust Core 负责：

- Event Bus
- Mood Engine
- Bond Engine
- Behavior Engine
- Reminder Engine
- Policy Engine
- State Store
- Perception Adapter
- LLM Proxy
- Local IPC

React 前端不负责判断 Ian 为什么要做某个行为。

前端只负责执行 Rust Core 输出的动作。

---

## 架构分层

```txt
┌─────────────────────────┐
│ React / WebView          │
│                         │
│ IanSprite                │
│ Bubble UI                │
│ Animation Player         │
│ Settings UI              │
└───────────▲─────────────┘
            │
            │ Tauri Events / Commands
            │
┌───────────▼─────────────┐
│ Rust Core Runtime        │
│                         │
│ Event Bus                │
│ Mood Engine              │
│ Bond Engine              │
│ Behavior Engine          │
│ Reminder Engine          │
│ Policy Engine            │
│ Storage                  │
└───────────▲─────────────┘
            │
            │
┌───────────▼─────────────┐
│ System / External        │
│                         │
│ Mouse / Time / Config    │
│ Git / LLM / Feishu       │
│ Future Adapters          │
└─────────────────────────┘
```

---

## IanEvent / IanAction 协议

Ian 从第一版开始采用协议化通信。

核心协议分三类：

```txt
IanEvent   = 外部世界输入给 Rust Core 的事件
IanAction  = Rust Core 输出给前端执行的动作
IanState   = Rust Core 同步给前端的当前状态
```

模型：

```txt
World Event
  ↓
IanEvent
  ↓
Rust Core
  ↓
Mood / Bond / Behavior Policy
  ↓
IanAction
  ↓
Frontend Animation / Speech / UI
```

---

### IanEvent 示例

```ts
type IanEvent =
  | { type: "mouse.click"; payload: { x: number; y: number } }
  | { type: "mouse.double_click"; payload: { x: number; y: number } }
  | { type: "mouse.near"; payload: { x: number; y: number; distance: number } }
  | { type: "dialogue.user_message"; payload: { text: string } }
  | { type: "time.tick"; payload: { now: number } }
  | { type: "reminder.water"; payload: {} }
  | { type: "reminder.break"; payload: {} }
```

---

### IanAction 示例

```ts
type IanAction =
  | {
      type: "animation.play"
      payload: {
        name: "idle" | "walk" | "sleep" | "run" | "happy"
        loop?: boolean
      }
    }
  | {
      type: "movement.move_to"
      payload: {
        x: number
        y: number
        speed: "slow" | "normal" | "fast"
      }
    }
  | {
      type: "speech.show"
      payload: {
        text: string
        mood?: string
        durationMs?: number
      }
    }
  | { type: "bubble.open"; payload: {} }
  | { type: "bubble.close"; payload: {} }
  | { type: "behavior.sleep"; payload: {} }
  | {
      type: "behavior.run_around"
      payload: { durationMs: number }
    }
```

---

## 协议类型管理

Ian 的协议类型以 Rust 为源头。

即：

> Rust 类型作为协议源头，TypeScript 类型自动生成。

推荐使用类似：

```txt
ts-rs
```

避免 Rust 和前端手写两套协议导致不一致。

未来 Git、飞书、Pet Visit、插件事件都会进入这套协议体系，因此协议必须稳定。

---

## Animation Rendering

Ian v0.1 直接采用：

```txt
简单像素 Sprite
```

不先使用纯 emoji，也不在第一版引入 PixiJS。

v0.1 最小动画资源：

```txt
idle 2 帧
walk 4 帧
sleep 2 帧
run 4 帧
happy 2 帧
```

足够支撑：

- 待机
- 慢走
- 睡觉
- 满屏小跑
- 开心反馈
- 喝水提醒
- 鼠标靠近反应

后续 v0.2 再扩展：

- 委屈
- 傲娇
- 黏人
- 紧张
- 探头
- 摔倒
- 趴窗口
- 拖拽乱蹬

---

## Resource Pack 资源包

Ian v0.1 采用宠物资源包设计。

即：

```txt
~/.ian/resources/pets/ian-alpaca/
  pet.json
  sprite.png
  animations.json
  expressions.json
  sounds/
```

资源包先开放，美术和动作可扩展。

脚本和行为模组后开放。

---

### pet.json

```json
{
  "id": "ian-alpaca",
  "name": "Ian Alpaca",
  "species": "alpaca",
  "defaultPersonality": "gentle_playful",
  "sprite": "sprite.png",
  "animations": "animations.json",
  "expressions": "expressions.json",
  "sounds": "sounds/"
}
```

---

### animations.json

```json
{
  "meta": {
    "frameWidth": 96,
    "frameHeight": 96,
    "scale": 2
  },
  "animations": {
    "idle": {
      "frames": [0, 1],
      "fps": 2,
      "loop": true
    },
    "walk": {
      "frames": [2, 3, 4, 5],
      "fps": 8,
      "loop": true
    },
    "sleep": {
      "frames": [6, 7],
      "fps": 1,
      "loop": true
    },
    "run": {
      "frames": [8, 9, 10, 11],
      "fps": 12,
      "loop": true
    }
  }
}
```

---

## Resource Pack 的长期意义

Ian 不是一只固定羊驼。

它是数字生命系统。

未来可以自然支持：

- 羊驼
- 猫
- 小机器人
- 幽灵
- 水母
- 自定义宠物
- 社区皮肤
- 动作包
- 表情包
- 声音包

---

## Local-first Storage

Ian v0.1 直接采用：

```txt
config.toml + SQLite
```

而不是先 JSON 后 SQLite。

原因：

Ian 是长期数字生命，未来一定会有情绪历史、羁绊变化、互动记录、shared memories、提醒记录、Pet Visit 记录、插件事件记录。

JSON 很快会变乱。

SQLite 从第一版上，长期收益更大。

---

## 本地目录结构

```txt
~/.ian/
  config.toml
  ian.db
  resources/
    pets/
      ian-alpaca/
        pet.json
        sprite.png
        animations.json
        expressions.json
        sounds/
  logs/
```

---

## config.toml

存用户可编辑配置。

例如：

```toml
[app]
active_pet = "ian-alpaca"
behavior_mode = "normal"

[llm]
enabled = false
base_url = "https://api.openai.com/v1"
model = "gpt-4o-mini"

[reminder]
drink_water_minutes = 45
take_break_minutes = 60

[behavior]
mode = "normal"

[behavior.proactive]
max_text_per_hour = 1
quiet_when_busy = true

[behavior.sleep]
corner_weight = 0.6
near_user_weight = 0.25
mood_based_weight = 0.15
```

---

## ian.db

存运行状态和长期数据。

包括：

- pet identity
- mood state
- bond state
- interaction history
- reminder records
- memory moments
- adapter events
- social bindings
- resource pack metadata

---

## Perception / Adapter System

Ian v0.1 采用：

> 第一版定义 Adapter 抽象，但只实现内置 Adapter。

也就是架构上预留扩展，功能上保持克制。

v0.1 只实现：

```txt
TimeAdapter
MouseAdapter
ReminderAdapter
DialogueAdapter
```

未来再扩展：

```txt
GitAdapter
KeyboardRhythmAdapter
FeishuAdapter
CursorAdapter
ClaudeCodeAdapter
PetVisitAdapter
CustomScriptAdapter
```

核心原则：

> 所有外部世界进入 Ian，都必须先变成 IanEvent。

---

### Adapter Trait

```rust
trait PerceptionAdapter {
    fn name(&self) -> &'static str;
    fn poll(&mut self) -> Vec<IanEvent>;
}
```

后续所有外部集成都是新增 Adapter，而不是重构 Core。

---

## Behavior Policy 配置化

Ian v0.1 就做行为策略配置化。

但只开放少量配置给普通用户。

普通用户可见配置：

- 安静 / 正常 / 活泼
- 喝水提醒间隔
- 休息提醒间隔
- 是否开启声音
- 是否开启 LLM

高级参数保留在：

```txt
~/.ian/config.toml
```

例如：

```toml
[behavior]
mode = "normal"

[behavior.proactive]
max_text_per_hour = 1
quiet_when_busy = true
surprise_probability = 0.05

[behavior.sleep]
corner_weight = 0.6
near_user_weight = 0.25
mood_based_weight = 0.15
```

原则：

> 行为策略从第一版就配置化。
>
> 用户界面保持简单。
>
> 高级用户可以调配置。
>
> 后续不同宠物 / 不同人格 / 不同插件可以复用同一套 Policy 系统。

---

## LLM 接入

Ian 的 LLM 接入必须克制。

Ian 不是 ChatGPT 桌宠版。

LLM 是 Ian 的大脑之一，但不是 Ian 的全部。

---

## DialogueAdapter

LLM 接入通过 DialogueAdapter 分层：

```txt
DialogueAdapter
  ├─ DemoDialogueProvider
  ├─ OpenAICompatibleProvider
  ├─ OllamaProvider       // 后续
  └─ CustomProvider       // 后续
```

---

### A · Demo Dialogue Mode

不配置 API Key 也能用。

Ian 内置一组本地短句回复。

例如：

```txt
“哞？”
“我在这儿。”
“喝水水。”
“才不是担心你。”
“你又熬夜啦……”
```

作用：

- 开箱即用
- demo 稳定
- 没有成本
- 不依赖网络

---

### B · BYOM Mode

用户自己配置模型。

```toml
[llm]
enabled = true
provider = "openai-compatible"
base_url = "https://api.openai.com/v1"
model = "gpt-4o-mini"
```

API Key 不写进 toml，走系统安全存储。

优点：

- 成本归用户
- 开源项目友好
- 可以接 GPT / Claude compatible endpoint / DeepSeek / Kimi / OpenRouter / Ollama

---

### C · Local Model Mode

后续支持：

- Ollama
- LM Studio
- 本地模型

例如：

```toml
[llm]
enabled = true
provider = "ollama"
base_url = "http://localhost:11434/v1"
model = "llama3.1"
```

---

## LLM 语言边界

即使用了 LLM，也必须通过 DialoguePolicy 控制 Ian 的语气。

规则：

- 默认短句
- 保持 Ian 语气
- 不说“作为 AI 助手”
- 不主动输出大段解释
- 只有用户明确提问时进入认真回答模式

原则：

> 能力可以出现，但不能压过生命感。

---

## v0.1 技术范围

v0.1 采用“架构完整、功能克制”的范围定义。

### v0.1 Architecture Baseline 必须建立

- Tauri v2 桌面壳
- Rust Core Runtime
- React + Vite + TypeScript 前端
- Sprite Resource Pack
- IanEvent / IanAction / IanState 协议
- Rust 类型到 TypeScript 类型生成
- Adapter 抽象与内置最小 Adapter
- Behavior Policy 边界
- Dialogue Provider 边界
- SQLite + config.toml 的 migration / repository 骨架
- Security Gate / Permission 的最小骨架

这些属于长期工程边界。

即使某些能力在 P0 不作为用户可见功能，也应该先以 skeleton / no-op / minimal implementation 的方式存在。

### P0 / MVP 用户可见能力

P0 只验证：

> Ian 是否像一个真的住在桌面里的小生命。

用户可见功能收缩为：

- 透明置顶羊驼
- 基础 sprite 动画
- 点击弹出聊天气泡
- Demo Dialogue 短句回复
- 可选 OpenAI-compatible BYOM
- 流式输出
- 双击 / 低频触发满屏乱跑彩蛋
- 基础拖拽与位置记忆

### v0.1.x 快速补齐能力

在 P0 成立后，再快速补：

- 喝水 / 休息提醒
- 更多 idle / walk / sleep / happy 动作
- 简版 Mood State
- 简版 Bond State
- SQLite 状态恢复
- 基础设置面板
- 更完整的 DialoguePolicy

---

## v0.1 暂不做

- 全屏透明 overlay
- 复杂点击穿透
- 全局键盘监听
- 窗口感知
- 代码理解
- 飞书
- Pet Visit
- 插件系统
- 多 Ian 同屏
- 长期复杂记忆与向量记忆
- 资源包脚本权限

这些后续逐步进入 v0.2 / v0.3 / v1.0。

---

## 技术架构核心原则

> Everything is an Event.

所有外部输入都变成 IanEvent。

所有行为输出都变成 IanAction。

所有长期变化都沉淀进 IanState / SQLite。

这样 Ian 后续无论接入 Git、飞书、MCP、Cursor、Claude Code、Pet Visit、插件系统，都是新增事件源和策略，而不是重构核心系统。

---

# 07 · Architecture Finalization · 技术终局设计

## 设计立场

Ian 的技术架构不应以“最少代码量”为首要目标。

在 AI 辅助开发时代，代码实现成本显著下降，真正昂贵的是：

- 架构边界错误
- 协议不稳定
- 数据模型不可迁移
- 后期插件化困难
- 安全边界缺失
- 从 demo 到产品需要推倒重来

因此 Ian 的技术策略是：

> v0.1 功能克制，但架构以 v1.0 / v2.0 的终局合理性为准。

也就是说：

- 第一版不做很多功能
- 但第一版就要搭对 Runtime
- 第一版就要确立协议
- 第一版就要确立数据模型
- 第一版就要预留 Adapter / Plugin / Resource Pack

Ian 不应该是一个 React 动画 demo。

它应该从第一天就是：

> 一个本地优先的数字生命 Runtime。

---

# 07.1 · 总体架构终局

Ian 的长期架构分为 8 层。

```txt
Ian Desktop App
  ├─ 01 Desktop Shell Layer
  ├─ 02 Creature Runtime Layer
  ├─ 03 Rendering Layer
  ├─ 04 Resource Pack Layer
  ├─ 05 Perception Adapter Layer
  ├─ 06 Dialogue / LLM Layer
  ├─ 07 Storage / Memory Layer
  └─ 08 Integration / Social Layer
```

每一层职责独立。

不要让前端 UI、LLM、飞书、Git、动画、记忆混在一起。

---

## 01 Desktop Shell Layer

技术：

```txt
Tauri v2 + Rust
```

职责：

- 透明窗口
- 无边框窗口
- always-on-top
- 托盘菜单
- 多窗口管理
- 本地文件系统
- 系统权限管理
- 安全存储
- 打包分发
- 后续自动更新

设计原则：

> Desktop Shell 只负责桌面能力，不负责 Ian 的人格和行为。

---

## 02 Creature Runtime Layer

技术：

```txt
Rust Core Runtime
```

职责：

- Event Bus
- Mood Engine
- Bond Engine
- Behavior Engine
- Policy Engine
- Reminder Engine
- Dialogue Orchestrator
- Adapter Manager
- State Manager
- Security / Permission Gate

设计原则：

> Ian 的“生命逻辑”必须在 Rust Core 中，而不是散落在前端组件里。

---

## 03 Rendering Layer

技术：

```txt
React + Vite + TypeScript
CSS / Sprite Sheet / Canvas later
```

职责：

- Sprite 渲染
- 动画播放
- 气泡 UI
- 输入框
- 设置面板
- 角色卡
- 视觉反馈

设计原则：

> 前端是 Ian 的身体，不是 Ian 的大脑。

前端只执行 IanAction，不直接决定 Ian 的长期行为逻辑。

---

## 04 Resource Pack Layer

职责：

- 宠物形象
- Sprite sheet
- 动作定义
- 表情定义
- 声音包
- 默认人格配置
- 后续皮肤 / 动作包 / 社区资源

设计原则：

> Ian 是数字生命系统，不是一只固定羊驼。

所以资源必须包化。

---

## 05 Perception Adapter Layer

职责：

- 时间感知
- 鼠标感知
- 键盘节奏
- Git 事件
- IDE 事件
- Terminal 事件
- 飞书事件
- Pet Visit
- 自定义脚本

设计原则：

> 所有外部世界进入 Ian，都必须先变成 IanEvent。

Adapter 不直接控制动画，也不直接修改状态。

---

## 06 Dialogue / LLM Layer

职责：

- Demo Dialogue
- OpenAI-compatible BYOM
- 本地模型
- Prompt 构造
- 语气控制
- 回复裁剪
- 安全过滤

设计原则：

> LLM 是 Ian 的一个语言能力，不是 Ian 的全部人格。

LLM 输出必须经过 DialoguePolicy。

---

## 07 Storage / Memory Layer

技术：

```txt
SQLite + config.toml
```

职责：

- 用户配置
- 宠物身份
- 情绪状态
- 羁绊状态
- 交互历史
- 提醒记录
- 记忆时刻
- Adapter 事件日志
- 社交绑定
- 后续向量记忆

设计原则：

> Ian 是长期存在的数字生命，所以必须有长期本地状态。

---

## 08 Integration / Social Layer

职责：

- CLI
- MCP
- Git Hook
- IDE 插件
- Feishu Relay
- Pet Visit
- 群组 Ian
- 多 Ian 同屏

设计原则：

> 社交与外部集成是 Ian 的扩展层，不应该污染核心 Runtime。

---

# 07.2 · Rust Core 模块结构

Ian 的 Rust Core 建议采用清晰的领域分层。

```txt
src-tauri/src/
  main.rs

  app/
    mod.rs
    bootstrap.rs
    runtime.rs

  core/
    mod.rs
    event_bus.rs
    action_dispatcher.rs
    creature_state.rs
    scheduler.rs

  domain/
    mood/
      mod.rs
      mood_state.rs
      mood_signal.rs
      mood_engine.rs

    bond/
      mod.rs
      bond_state.rs
      bond_signal.rs
      bond_engine.rs

    behavior/
      mod.rs
      behavior_engine.rs
      behavior_policy.rs
      behavior_action.rs

    dialogue/
      mod.rs
      dialogue_engine.rs
      dialogue_policy.rs
      providers/
        demo.rs
        openai_compatible.rs

    reminder/
      mod.rs
      reminder_engine.rs
      reminder_policy.rs

  protocol/
    mod.rs
    event.rs
    action.rs
    state.rs
    error.rs

  adapters/
    mod.rs
    adapter_trait.rs
    time_adapter.rs
    mouse_adapter.rs
    reminder_adapter.rs
    dialogue_adapter.rs
    // future:
    // git_adapter.rs
    // keyboard_rhythm_adapter.rs
    // feishu_adapter.rs
    // pet_visit_adapter.rs

  storage/
    mod.rs
    db.rs
    migrations.rs
    repositories/
      pet_repo.rs
      mood_repo.rs
      bond_repo.rs
      event_repo.rs
      memory_repo.rs
      reminder_repo.rs

  resources/
    mod.rs
    resource_pack.rs
    resource_loader.rs
    resource_registry.rs

  desktop/
    mod.rs
    window.rs
    tray.rs
    commands.rs
    secrets.rs

  security/
    mod.rs
    permission.rs
    sanitizer.rs
    rate_limiter.rs
```

---

## 分层原则

### `protocol/`

定义 IanEvent、IanAction、IanState。

这是 Rust Core 和前端、Adapter、未来插件之间的协议边界。

协议必须稳定。

---

### `domain/`

定义 Ian 的核心生命系统。

包括：

- mood
- bond
- behavior
- dialogue
- reminder

这些模块不应该依赖 Tauri UI。

---

### `adapters/`

定义外部世界输入。

Adapter 只负责收集事件。

不负责决定 Ian 做什么。

---

### `storage/`

管理 SQLite 和 Repository。

Domain 不直接写 SQL。

通过 Repository 存取状态。

---

### `resources/`

管理宠物资源包。

支持未来多个宠物、多皮肤、多动作。

---

### `desktop/`

只处理桌面壳能力。

例如窗口、托盘、commands、secret storage。

---

### `security/`

统一处理安全边界。

例如：

- 外部事件权限
- Pet Visit 消息过滤
- LLM 输出裁剪
- 插件事件限频
- 敏感内容拒绝

---

# 07.3 · Runtime 执行流程

Ian 的核心循环：

```txt
Adapter / Frontend / Timer
  ↓
IanEvent
  ↓
Security Gate
  ↓
Event Bus
  ↓
Mood Engine
  ↓
Bond Engine
  ↓
Behavior Engine
  ↓
IanAction
  ↓
Action Dispatcher
  ↓
Frontend Renderer
  ↓
State Persisted to SQLite
```

---

## 事件进入

所有事件必须变成 IanEvent。

例如：

- mouse.near
- mouse.click
- time.tick
- reminder.water
- dialogue.user\_message
- git.push.success
- social.pet\_visit.request

---

## 安全门

所有外部输入先经过 Security Gate。

Security Gate 负责：

- 来源校验
- 权限检查
- 频率限制
- payload 大小限制
- 敏感内容过滤
- 是否允许进入 Event Bus

尤其是未来飞书、Pet Visit、插件系统，必须经过这一层。

---

## 情绪计算

Mood Engine 不直接执行动作。

它只输出 MoodState。

例如：

```txt
mouse.near + night.time + high bond
  ↓
primary_mood = clingy
secondary_mood = sleepy
```

---

## 羁绊更新

Bond Engine 处理 BondSignal。

例如：

- 用户摸摸 Ian
- 用户回应提醒
- 陪伴时间增加
- 共同经历项目事件

BondState 更新后，会影响 Behavior Engine。

---

## 行为决策

Behavior Engine 综合：

- IanEvent
- MoodState
- BondState
- BehaviorPolicy
- BusyState
- TimeContext

输出 IanAction。

---

## 动作派发

Action Dispatcher 把 IanAction 发给前端。

前端只负责执行动作，不做核心决策。

---

# 07.4 · IanEvent / IanAction / IanState 协议规范

## IanEvent

IanEvent 是外部世界输入 Ian 的唯一方式。

建议 v0.1 事件：

```rust
enum IanEvent {
    TimeTick { now_ms: i64 },

    MouseNear { x: f64, y: f64, distance: f64 },
    MouseClick { x: f64, y: f64 },
    MouseDoubleClick { x: f64, y: f64 },
    MouseDragStart { x: f64, y: f64 },
    MouseDragEnd { x: f64, y: f64 },

    DialogueUserMessage { text: String },
    DialogueReplyDone { text: String },

    ReminderWater,
    ReminderBreak,

    AppStarted,
    AppWillQuit,
}
```

未来扩展：

```rust
enum IanEvent {
    GitPushSuccess { branch: String, commits: u32 },
    BuildFailed { project: String },
    FeishuMention { from: String, text: String },
    PetVisitRequest { from_user: String, message: String },
    PluginEvent { source: String, event_type: String, payload_json: String },
}
```

---

## IanAction

IanAction 是 Rust Core 输出给前端的唯一方式。

v0.1 动作：

```rust
enum IanAction {
    AnimationPlay {
        name: String,
        looped: bool,
    },
    MovementMoveTo {
        x: f64,
        y: f64,
        speed: MovementSpeed,
    },
    SpeechShow {
        text: String,
        mood: Option<String>,
        duration_ms: Option<u64>,
    },
    BubbleOpen,
    BubbleClose,
    BehaviorSleep,
    BehaviorRunAround {
        duration_ms: u64,
    },
    StateSync {
        state: IanState,
    },
}
```

---

## IanState

IanState 是前端需要知道的当前状态。

```rust
struct IanState {
    pet_id: String,
    mood: MoodState,
    bond: BondStateView,
    behavior: CurrentBehavior,
    position: Position,
    active_resource_pack: String,
    behavior_mode: BehaviorMode,
}
```

注意：

IanState 给前端的是 view model，不一定暴露完整内部数据。

例如 bond\_value 可以不传，只传：

```txt
relationship_hint = "getting_closer"
```

---

## 类型生成

协议类型以 Rust 为源头，通过 `ts-rs` 或类似工具生成 TypeScript 类型。

原则：

> 不手写双份协议。

---

# 07.5 · 数据库 Schema 设计

## 数据库设计原则

Ian 使用 SQLite 作为本地长期状态数据库。

设计原则：

1. Local-first
2. 状态表和事件表分离
3. 核心实体规范化
4. 高频事件追加写入
5. JSON 只作为扩展 payload，不替代核心字段
6. 所有核心表带 `created_at` / `updated_at`
7. 使用 migration 管理 schema 演进
8. 为未来 Pet Visit / Feishu / Plugin / Memory 预留扩展空间

---

## 表结构总览

v0.1 建议表：

```txt
pet_identities
resource_packs
mood_states
bond_states
interaction_events
reminder_records
memory_moments
adapter_events
settings_kv
schema_migrations
```

---

## pet\_identities

存 Ian 个体身份。

```sql
CREATE TABLE pet_identities (
  id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  species TEXT NOT NULL,
  resource_pack_id TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
```

说明：

- 未来支持多个宠物
- 当前激活宠物由 config.toml 或 settings\_kv 指向

---

## resource\_packs

存已安装资源包元数据。

```sql
CREATE TABLE resource_packs (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  kind TEXT NOT NULL,
  path TEXT NOT NULL,
  manifest_json TEXT NOT NULL,
  installed_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
```

说明：

- kind 可为 pet / skin / sound / motion
- manifest\_json 缓存 pet.json 内容

---

## mood\_states

存当前情绪状态。

```sql
CREATE TABLE mood_states (
  pet_id TEXT PRIMARY KEY,
  primary_mood TEXT NOT NULL,
  secondary_mood TEXT,
  intensity REAL NOT NULL DEFAULT 0,
  mood_vector_json TEXT,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY (pet_id) REFERENCES pet_identities(id)
);
```

说明：

- primary\_mood 用于快速读取
- mood\_vector\_json 存多维权重
- 不为每个 mood 单独建字段，避免扩展困难

---

## bond\_states

存羁绊状态。

```sql
CREATE TABLE bond_states (
  pet_id TEXT PRIMARY KEY,
  bond_value REAL NOT NULL DEFAULT 0,
  internal_stage TEXT NOT NULL,
  traits_json TEXT,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY (pet_id) REFERENCES pet_identities(id)
);
```

说明：

- bond\_value 内部使用，不直接展示给用户
- internal\_stage 例如 Shy / Familiar / Companion / Bonded
- traits\_json 存适应性倾向

---

## interaction\_events

存用户与 Ian 的交互事件。

```sql
CREATE TABLE interaction_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pet_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  source TEXT NOT NULL,
  payload_json TEXT,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (pet_id) REFERENCES pet_identities(id)
);

CREATE INDEX idx_interaction_events_pet_time
ON interaction_events (pet_id, created_at);

CREATE INDEX idx_interaction_events_type_time
ON interaction_events (event_type, created_at);
```

说明：

- 高频追加写入
- 用于行为学习、debug、轻量记忆生成

---

## reminder\_records

存提醒记录。

```sql
CREATE TABLE reminder_records (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pet_id TEXT NOT NULL,
  reminder_type TEXT NOT NULL,
  status TEXT NOT NULL,
  scheduled_at INTEGER,
  triggered_at INTEGER NOT NULL,
  acknowledged_at INTEGER,
  payload_json TEXT,
  FOREIGN KEY (pet_id) REFERENCES pet_identities(id)
);

CREATE INDEX idx_reminder_records_pet_time
ON reminder_records (pet_id, triggered_at);
```

说明：

- status 可为 triggered / acknowledged / ignored / snoozed
- 后续用于调整提醒策略

---

## memory\_moments

存轻量记忆时刻。

```sql
CREATE TABLE memory_moments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pet_id TEXT NOT NULL,
  memory_type TEXT NOT NULL,
  summary TEXT NOT NULL,
  importance REAL NOT NULL DEFAULT 0,
  source_event_ids_json TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY (pet_id) REFERENCES pet_identities(id)
);

CREATE INDEX idx_memory_moments_pet_importance
ON memory_moments (pet_id, importance);
```

说明：

- v0.1 Baseline 只建立 schema / repository
- P0 不生成长期记忆
- v0.1.x / v0.2 再少量使用
- 后续可与向量记忆关联
- summary 必须是用户可理解的短描述

---

## adapter\_events

存外部 Adapter 事件。

```sql
CREATE TABLE adapter_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  event_type TEXT NOT NULL,
  sensitivity TEXT NOT NULL,
  payload_json TEXT,
  accepted INTEGER NOT NULL DEFAULT 1,
  rejection_reason TEXT,
  created_at INTEGER NOT NULL
);

CREATE INDEX idx_adapter_events_source_time
ON adapter_events (source, created_at);

CREATE INDEX idx_adapter_events_type_time
ON adapter_events (event_type, created_at);
```

说明：

- 记录外部事件是否被安全门接受
- 对未来插件、飞书、Pet Visit 审计很重要

---

## settings\_kv

存少量运行时设置。

```sql
CREATE TABLE settings_kv (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);
```

说明：

- 用户可编辑配置仍以 config.toml 为主
- settings\_kv 存运行时动态设置

---

## schema\_migrations

存 migration 版本。

```sql
CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  applied_at INTEGER NOT NULL
);
```

---

## 未来表

v0.2 / v0.3 后再增加：

```txt
llm_messages
social_bindings
pet_visit_records
plugin_registry
permission_grants
vector_memories
memory_embeddings
```

不在 v0.1 过早引入。

---

# 07.6 · Resource Pack 规范

## 目录结构

```txt
resources/pets/ian-alpaca/
  pet.json
  sprite.png
  animations.json
  expressions.json
  sounds/
    step.wav
    sleep.wav
    happy.wav
```

---

## pet.json

```json
{
  "id": "ian-alpaca",
  "name": "Ian Alpaca",
  "version": "0.1.0",
  "species": "alpaca",
  "defaultPersonality": "gentle_playful",
  "sprite": "sprite.png",
  "animations": "animations.json",
  "expressions": "expressions.json",
  "sounds": "sounds/",
  "capabilities": [
    "idle",
    "walk",
    "sleep",
    "run",
    "happy"
  ]
}
```

---

## animations.json

```json
{
  "meta": {
    "frameWidth": 96,
    "frameHeight": 96,
    "scale": 2
  },
  "animations": {
    "idle": {
      "frames": [0, 1],
      "fps": 2,
      "loop": true
    },
    "walk": {
      "frames": [2, 3, 4, 5],
      "fps": 8,
      "loop": true
    },
    "sleep": {
      "frames": [6, 7],
      "fps": 1,
      "loop": true
    },
    "run": {
      "frames": [8, 9, 10, 11],
      "fps": 12,
      "loop": true
    },
    "happy": {
      "frames": [12, 13],
      "fps": 6,
      "loop": false
    }
  }
}
```

---

## expressions.json

v0.1 可以简单预留。

```json
{
  "expressions": {
    "calm": { "overlay": null },
    "happy": { "overlay": null },
    "sleepy": { "overlay": null },
    "wronged": { "overlay": null }
  }
}
```

后续可以支持表情层、颜色层、粒子层。

---

# 07.7 · Adapter 机制终局

## Adapter 定位

Adapter 是 Ian 感知外部世界的入口。

Adapter 不决定行为。

Adapter 只产生 IanEvent。

---

## Adapter Trait

```rust
trait PerceptionAdapter {
    fn id(&self) -> &'static str;
    fn sensitivity(&self) -> SensitivityLevel;
    fn poll(&mut self) -> Vec<IanEvent>;
}
```

---

## Adapter 分类

### Built-in Adapter

v0.1 内置：

- TimeAdapter
- MouseAdapter
- ReminderAdapter
- DialogueAdapter

---

### Local Passive Adapter

后续：

- GitAdapter
- CursorAdapter
- ClaudeCodeAdapter
- CodexAdapter
- TerminalAdapter

优先被动读取低敏事件，而不是默认监听高敏输入。

---

### Remote / Social Adapter

后续：

- FeishuAdapter
- PetVisitAdapter
- GroupEventAdapter

必须经过 Security Gate。

---

### Custom Script Adapter

后续开放。

需要权限声明、限频、沙箱或明确本地信任机制。

---

## Adapter 安全原则

所有 Adapter 必须声明：

- source id
- sensitivity level
- permission requirement
- event type
- payload size limit

高敏 Adapter 默认关闭。

例如：

- clipboard
- screen content
- code content
- chat content

不得默认开启。

---

# 07.8 · Dialogue / LLM 架构终局

## Dialogue 层定位

Dialogue 是 Ian 的语言能力。

不是 Ian 的全部人格。

Dialogue 必须服从：

- MoodState
- BondState
- DialoguePolicy
- SafetyPolicy
- User Intent

---

## Dialogue Providers

```txt
DialogueProvider
  ├─ DemoDialogueProvider
  ├─ OpenAICompatibleProvider
  ├─ OllamaProvider
  ├─ OpenRouterProvider
  └─ CustomProvider
```

---

## v0.1

v0.1 Architecture Baseline 必须定义：

- DemoDialogueProvider
- OpenAI-compatible BYOM

P0 用户可见体验中：

- Demo mode 必须可用，保证 Ian 开箱即用。
- OpenAI-compatible BYOM 可以作为可选能力进入 P0，也可以放入 v0.1.x，但 Provider 边界和配置位置必须先设计好。

BYOM 的意义是保证高级用户可接入真实模型，但它不能把 Ian 的第一体验变成“ChatGPT 桌宠版”。

---

## Prompt 构造

Prompt 不应只包含 system prompt。

应由多个上下文块组成：

```txt
System Identity
Mood Context
Bond Context
Recent Interaction Summary
Current User Message
Response Policy
```

例如：

```txt
你是 Ian，一只住在用户桌面里的小生命。
当前情绪：有点困，但想陪着用户。
关系状态：正在变熟。
回复要求：短句、口语、不要像 AI 助手。
```

---

## 输出裁剪

LLM 输出必须经过 DialoguePolicy。

检查：

- 是否过长
- 是否出现“作为 AI 助手”
- 是否偏离 Ian 人设
- 是否包含敏感或不适合展示内容
- 是否需要缩短成气泡文本

---

# 07.9 · Security / Permission 设计

Ian 后期会接入大量事件源。

安全设计必须提前存在。

---

## Security Gate

所有外部事件进入 Runtime 前必须经过 Security Gate。

检查：

- 来源是否合法
- 是否有权限
- 是否超频
- payload 是否过大
- 是否包含敏感内容
- 是否允许进入 Event Bus

---

## Rate Limiter

对以下事件限频：

- 主动提醒
- Pet Visit
- 飞书消息
- 插件事件
- LLM 触发
- 彩蛋行为

---

## Sanitizer

对外部文本做清洗。

尤其是：

- Pet Visit 消息
- 飞书消息
- 插件消息
- LLM 输出

避免直接把高风险文本渲染到用户桌面。

---

## Permission Model

未来所有高敏能力必须明确授权。

默认允许：

- 时间
- Ian 窗口内鼠标
- 本地提醒
- 本地 demo dialogue

需要授权：

- 全局键盘节奏
- Git 项目信息
- 当前 App
- 飞书
- IDE 状态

默认禁止：

- 代码正文
- 剪贴板
- 私聊内容
- 屏幕 OCR
- 文件内容

---

# 07.10 · v0.1 工程 Milestones

这些 Milestone 分为两类：

- **Baseline**：v0.1 必须建立的长期工程边界。
- **Product Slice**：可以按 P0 / v0.1.x 快速拆分上线的用户可见能力。

原则：

> Baseline 尽早稳定，Product Slice 小步快跑。

## M1 · Desktop Shell · Baseline + P0

目标：

- Tauri v2 app
- 透明窗口
- 无边框
- always-on-top
- 托盘
- 右下角出现 Ian

---

## M2 · Resource Pack Renderer · Baseline + P0

目标：

- 加载 pet.json
- 加载 sprite.png
- 加载 animations.json
- 播放 idle / walk / sleep / run / happy

---

## M3 · Rust Core Runtime · Baseline

目标：

- Event Bus
- IanEvent
- IanAction
- IanState
- Rust → TS 类型生成
- Action Dispatcher
- Scheduler / Tick skeleton
- Security Gate skeleton

---

## M4 · Basic Behavior · P0

目标：

- idle
- walk
- sleep
- mouse.near reaction
- mouse.click reaction
- double click run around

---

## M5 · Bubble Dialogue · P0

目标：

- 点击打开气泡
- 用户输入一句话
- DemoDialogueProvider 回复
- 支持短句气泡
- OpenAI-compatible BYOM 作为可选增强，不阻塞 P0

---

## M6 · SQLite + Config · Baseline

目标：

- 初始化 \~/.ian
- config.toml
- ian.db
- migrations
- pet\_identity
- mood\_state
- bond\_state
- interaction\_events

说明：

- P0 可以只写入位置、配置、基础交互事件。
- mood / bond / memory 表可以先建立 schema 与 repository，不要求完整产品行为依赖它们。

---

## M7 · Reminder · v0.1.x Product Slice

目标：

- 喝水提醒
- 休息提醒
- 动作 + 小气泡短句
- reminder\_records

---

## M8 · BYOM LLM · P0 Optional / v0.1.x

目标：

- OpenAI-compatible provider
- Key 存安全存储
- 流式回复
- DialoguePolicy 裁剪

说明：

- Provider 边界和安全存储应在 v0.1 Baseline 中预留。
- 用户可见 BYOM 可根据 P0 体验节奏决定是否首发。

---

# 07.11 · 版本演进路线

## P0 / MVP · Local Creature Proof

核心：

> Ian 真的住在桌面里。

做：

- Tauri 桌面壳
- Rust Core Runtime
- Sprite Ian
- 鼠标互动
- 气泡对话
- demo dialogue
- 可选 OpenAI-compatible BYOM
- 流式输出
- 满屏乱跑彩蛋
- 位置记忆

不做：

- 完整 Mood System
- 完整 Bond System
- 长期记忆
- 主动提醒系统
- Git / build / keyboard rhythm
- 飞书
- Pet Visit
- 代码感知
- 全局键盘监听
- 插件系统

---

## v0.1 Architecture Baseline

核心：

> Ian 不是一次性动画 demo，而是本地数字生命 Runtime。

做：

- IanEvent / IanAction / IanState 协议
- Rust → TypeScript 类型生成
- Resource Pack 规范
- Adapter 抽象
- Behavior Policy 边界
- Dialogue Provider 边界
- SQLite + config.toml migration 骨架
- Security Gate / Permission 骨架

说明：

- 这些能力不一定全部变成 P0 用户可见功能。
- 但工程边界必须先稳定，避免后续接入 Git、Feishu、Pet Visit、Plugin 时重构核心。

---

## v0.1.x · Local Creature Iterations

核心：

> 在不推翻架构的前提下快速补足基础生命感。

做：

- 喝水 / 休息提醒
- 更多动作和表情
- 基础设置面板
- 简版 Mood State
- 简版 Bond State
- SQLite 状态恢复
- 更完整 DialoguePolicy

---

## v0.2 · Developer Rhythm

核心：

> Ian 开始陪你写代码。

做：

- GitAdapter
- KeyboardRhythmAdapter
- build/test 事件
- 基础 Bond 行为变化
- 更多 mood animation
- 更完整 Resource Pack

---

## v0.3 · Social Presence

核心：

> Ian 开始进入团队空间。

做：

- Feishu Relay
- Pet Visit
- social bindings
- visit records
- simple group broadcast
- 本地授权和白名单

---

## v1.0 · Creature Platform

核心：

> Ian 成为数字生命平台。

做：

- Plugin API
- MCP / CLI
- Window-aware behavior
- 多资源包生态
- 多 Ian 同屏
- 完整 Permission Model
- 签名、公证、Homebrew

---

# 08 · Codex Handoff · 下一步执行计划

## 当前阶段判断

Ian 的产品方向、行为系统、情绪系统、成长系统和技术终局已经足够清晰。

下一阶段不应该继续扩展世界观。

应该进入：

> P0 / MVP + v0.1 Architecture Baseline Implementation Phase

目标是：

> 让 Ian 真正跑起来。

当前阶段不是做完整产品，而是同时验证两件事：

- Ian 是否真的能住在桌面里
- Ian 是否像一个小生命
- Ian 是否不打扰用户
- Ian 是否能通过动作 + 气泡产生陪伴感
- 当前架构是否能承接后续 Git / Feishu / Pet Visit / Plugin

其中：

- P0 负责验证用户可见生命感。
- v0.1 Architecture Baseline 负责稳定长期工程边界。

---

## 给 Codex 的执行原则

Codex 执行时应遵循以下原则：

### 1. 架构优先于最少代码

现在 AI 写代码效率很高，不需要为了少写代码牺牲架构。

要求：

- 模块边界清晰
- Runtime 可扩展
- 协议稳定
- SQLite schema 可迁移
- Resource Pack 可扩展
- Adapter 机制提前预留

---

### 2. v0.1 功能克制

用户可见功能只实现最小可感知生命体。

但工程结构不按一次性 demo 来做。

允许提前建立目录、协议、schema、trait、policy、security skeleton。

不要做：

- 飞书
- Pet Visit
- 插件系统
- 代码理解
- 全局键盘监听
- 窗口感知
- 多 Ian 同屏
- 复杂长期记忆

这些只保留架构入口，不做具体用户可见实现。

---

### 3. Everything is an Event

所有外部输入必须进入 IanEvent。

所有行为输出必须通过 IanAction。

不要让前端组件直接决定 Ian 的核心行为。

不要让 Adapter 直接控制动画。

---

### 4. Rust Core 是 Ian 的大脑

React 前端只是表现层。

Rust Core 负责：

- Event Bus
- Mood Engine
- Bond Engine
- Behavior Engine
- Reminder Engine
- Policy Engine
- Dialogue Orchestrator
- State Manager
- Security Gate

React 前端负责：

- Sprite 渲染
- 动画播放
- 气泡 UI
- 输入框
- 设置界面

---

## v0.1 代码仓库结构

建议采用 monorepo：

```txt
ian/
  apps/
    desktop/
      src/
      src-tauri/
      public/
        resources/
          pets/
            ian-alpaca/
              pet.json
              sprite.png
              animations.json
              expressions.json
              sounds/

  packages/
    protocol/        // 可选，后续抽出共享协议
    shared/          // 可选，后续共享类型与工具

  docs/
    product/
    technical/
```

v0.1 可以先只实现 `apps/desktop`，但目录应为未来 monorepo 留好空间。

---

## apps/desktop 前端结构

```txt
apps/desktop/src/
  main.tsx
  App.tsx

  renderer/
    IanStage.tsx
    IanSprite.tsx
    AnimationPlayer.tsx
    Bubble.tsx
    BubbleInput.tsx

  state/
    useIanState.ts
    useIanActions.ts

  protocol/
    generated.ts       // 由 Rust 类型生成

  resources/
    resourceLoader.ts
    animationTypes.ts

  ui/
    SettingsPanel.tsx
    TrayPanel.tsx

  lib/
    position.ts
    random.ts
    time.ts
```

前端只执行 IanAction。

不要在前端写 Mood / Bond / Behavior 核心逻辑。

---

## src-tauri Rust 结构

```txt
apps/desktop/src-tauri/src/
  main.rs

  app/
    mod.rs
    bootstrap.rs
    runtime.rs

  core/
    mod.rs
    event_bus.rs
    action_dispatcher.rs
    creature_state.rs
    scheduler.rs

  protocol/
    mod.rs
    event.rs
    action.rs
    state.rs
    error.rs

  domain/
    mood/
      mod.rs
      mood_state.rs
      mood_signal.rs
      mood_engine.rs

    bond/
      mod.rs
      bond_state.rs
      bond_signal.rs
      bond_engine.rs

    behavior/
      mod.rs
      behavior_engine.rs
      behavior_policy.rs
      behavior_action.rs

    dialogue/
      mod.rs
      dialogue_engine.rs
      dialogue_policy.rs
      providers/
        demo.rs
        openai_compatible.rs

    reminder/
      mod.rs
      reminder_engine.rs
      reminder_policy.rs

  adapters/
    mod.rs
    adapter_trait.rs
    time_adapter.rs
    mouse_adapter.rs
    reminder_adapter.rs
    dialogue_adapter.rs

  storage/
    mod.rs
    db.rs
    migrations.rs
    repositories/
      pet_repo.rs
      mood_repo.rs
      bond_repo.rs
      event_repo.rs
      memory_repo.rs
      reminder_repo.rs

  resources/
    mod.rs
    resource_pack.rs
    resource_loader.rs
    resource_registry.rs

  desktop/
    mod.rs
    window.rs
    tray.rs
    commands.rs
    secrets.rs

  security/
    mod.rs
    permission.rs
    sanitizer.rs
    rate_limiter.rs
```

---

## P0 / v0.1 Milestones

这些 Milestone 的口径与前文保持一致：

- P0：用户可见体验，优先证明“它活了”。
- v0.1 Baseline：长期工程骨架，允许先 skeleton / no-op / minimal implementation。
- v0.1.x：P0 成立后快速补体验，不推翻架构。

### M1 · 初始化工程 · Baseline

目标：建立可运行桌面应用。

任务：

- 创建 Tauri v2 + React + Vite + TypeScript 工程
- 建立 Rust 模块目录
- 建立基础窗口配置
- 支持透明窗口、无边框、always-on-top
- Ian 默认出现在桌面右下角
- 添加基础托盘菜单：Show / Hide / Quit

验收：

```txt
运行后，桌面右下角出现一个透明背景的小窗口。
```

---

### M2 · 协议层 IanEvent / IanAction / IanState · Baseline

目标：打通 Rust Core 与前端通信。

任务：

- 定义 IanEvent
- 定义 IanAction
- 定义 IanState
- 使用 Rust 类型作为协议源头
- 生成 TypeScript 类型
- 前端监听 `ian_action`
- 前端通过 command 发送 `ian_event`

验收：

```txt
前端点击 Ian → 发送 IanEvent → Rust Core 返回 IanAction → 前端播放动作或显示气泡。
```

---

### M3 · Resource Pack Renderer · Baseline + P0

目标：Ian 不写死为 emoji，而是通过资源包加载。

任务：

- 创建默认资源包 `ian-alpaca`
- 定义 `pet.json`
- 定义 `animations.json`
- 加载 `sprite.png`
- 前端实现 AnimationPlayer
- 支持 idle / walk / sleep / run / happy

验收：

```txt
Ian 能通过 sprite sheet 播放基础动画。
```

如果暂时没有正式美术资源，可以先用占位 sprite，但资源包结构必须正确。

---

### M4 · Rust Core Runtime · Baseline

目标：搭建 Ian 的最小生命运行时。

任务：

- Event Bus
- Mood Engine skeleton
- Bond Engine skeleton
- Behavior Engine 简版
- Action Dispatcher
- Scheduler / Tick
- CreatureState
- Security Gate skeleton
- Adapter trait

P0 最小可见状态：

- calm
- happy
- sleepy
- bored

说明：

- Mood / Bond 在 P0 可以是非常薄的状态枚举或 no-op policy。
- 不要求 P0 做完整心情系统或好感度系统。

验收：

```txt
Ian 可以根据 time.tick / mouse.click / mouse.near 输出不同 IanAction。
```

---

### M5 · 基础行为 · P0

目标：Ian 看起来真的活着。

任务：

- idle 待机
- 随机微动作
- walk 走动
- sleep 睡觉
- mouse.near 转头 / 反应
- mouse.click 短句回应
- mouse.double\_click 满屏小跑彩蛋

验收：

```txt
用户不操作时 Ian 会自然待机、偶尔微动；鼠标靠近和点击时有反应。
```

---

### M6 · SQLite + config.toml · Baseline

目标：建立本地长期状态基础。

任务：

- 初始化 `~/.ian/`
- 创建 `config.toml`
- 创建 `ian.db`
- 实现 migrations
- 建表：
  - pet\_identities
  - resource\_packs
  - mood\_states
  - bond\_states
  - interaction\_events
  - reminder\_records
  - memory\_moments
  - adapter\_events
  - settings\_kv
  - schema\_migrations
- 实现 repositories

验收：

```txt
Ian 的 pet identity、基础配置、位置状态、interaction event 可以持久化并在重启后恢复。
```

说明：

- mood / bond / memory / reminder 相关表可以先建 schema 与 repository。
- P0 不要求生成长期记忆，也不要求完整 mood / bond 行为依赖数据库。

---

### M7 · Bubble Dialogue · P0

目标：Ian 能说话，但不是 ChatGPT 桌宠。

任务：

- 点击 Ian 打开气泡输入
- 用户输入一句话
- Rust Core 接收 DialogueUserMessage
- DemoDialogueProvider 返回短句
- DialoguePolicy 控制语气和长度
- 前端显示 speech bubble

验收：

```txt
用户点击 Ian，输入“你在干嘛”，Ian 用短句、情绪型语气回复。
```

---

### M8 · Reminder Engine · v0.1.x

目标：实现最小温柔提醒。

任务：

- 喝水提醒
- 休息提醒
- 读取 config.toml 间隔
- 输出动作 + 小气泡短句
- 写入 reminder\_records
- 用户可忽略或确认

验收：

```txt
到时间后，Ian 通过动作 + 小气泡提醒喝水或休息，不使用系统通知。
```

---

### M9 · BYOM LLM · P0 Optional / v0.1.x

目标：支持用户自带模型，但不依赖模型才能运行。

任务：

- OpenAI-compatible provider
- base\_url / model 配置
- API Key 走系统安全存储
- 支持流式回复
- DialoguePolicy 裁剪输出
- Demo Mode 和 LLM Mode 可切换

验收：

```txt
未配置 API Key 时 Ian 使用 DemoDialogueProvider；配置后可走真实 LLM，但仍保持 Ian 语气。
```

说明：

- DialogueProvider 抽象与安全存储入口属于 v0.1 Baseline。
- BYOM 是否进入 P0 首发取决于它是否会压过桌面生命感；如果会，就放入 v0.1.x。

---

## P0 Definition of Done

P0 用户可见完成标准：

```txt
Ian 能作为一个 Tauri 桌面应用运行在 macOS 上。
Ian 以透明置顶小窗口出现在桌面。
Ian 通过资源包加载像素形象。
Ian 有 idle / walk / sleep / run / happy 动画。
Ian 能响应鼠标靠近、点击、双击。
Ian 能用气泡进行短句对话。
Ian 有 demo dialogue。
Ian 可以低频触发满屏乱跑彩蛋。
Ian 能记住基础位置 / 配置。
Ian 的行为由 Rust Core Runtime 决定。
```

P0 不要求：

```txt
提醒系统、完整 Mood、完整 Bond、长期记忆、飞书、Pet Visit、Git、代码感知、窗口感知、插件系统。
```

---

## v0.1 Architecture Baseline Definition of Done

v0.1 工程骨架完成标准：

```txt
IanEvent / IanAction / IanState 已由 Rust 类型定义，并能生成 TypeScript 类型。
Rust Core Runtime 有清晰模块边界。
Resource Pack 规范和默认 ian-alpaca 资源包目录已建立。
Adapter trait 已建立，Time / Mouse / Dialogue 可作为最小内置 Adapter。
SQLite migration / repository 骨架已建立。
Security Gate / Permission / Sanitizer / Rate Limiter 有最小 skeleton。
前端只执行 IanAction，不直接承载核心行为决策。
```

---

## 给 Codex 的第一条任务建议

建议先让 Codex 执行：

```txt
Task 001 · Create Ian desktop monorepo skeleton
```

内容：

- 初始化 monorepo
- 创建 `apps/desktop`
- 初始化 Tauri v2 + React + Vite + TypeScript
- 创建 Rust 模块目录
- 创建前端目录
- 加入空的协议类型
- 加入默认 resource pack 目录
- 加入 README

不要一开始就实现所有功能。

先把工程骨架搭出来。

---

## 推荐 Codex Prompt

```txt
We are building Ian, a local-first AI creature desktop companion.

Please create the v0.1 monorepo skeleton based on the technical architecture below.

Core requirements:
- Tauri v2 desktop app
- React + Vite + TypeScript frontend
- Rust Core Runtime inside src-tauri
- Rust is the source of truth for IanEvent, IanAction, IanState
- Prepare ts-rs or equivalent for TypeScript type generation
- SQLite + config.toml local-first storage skeleton
- Resource Pack structure under public/resources/pets/ian-alpaca
- Do not implement Feishu, Pet Visit, plugins, global keyboard monitoring, or code reading yet
- Focus on clean module boundaries and extensibility
- Product-visible P0 should stay small: transparent pet, sprite animation, click bubble, demo dialogue, double-click run-around
- Architecture should still prepare future extension points for adapters, storage, policy, dialogue providers, and security

Create:
- apps/desktop frontend structure
- src-tauri Rust module structure
- placeholder protocol types
- placeholder resource pack files
- placeholder migration system
- placeholder adapter / storage / security skeleton
- basic README explaining the architecture

Important architecture principle:
Everything is an Event.
All external inputs become IanEvent.
All behavior outputs become IanAction.
React only renders IanAction.
Rust Core owns behavior decisions.
```

---

## Codex 执行注意事项

Codex 不应先做炫酷 UI。

优先级：

1. 工程结构
2. 协议边界
3. Runtime skeleton
4. Resource loading
5. 最小行为闭环
6. UI polish

如果实现过程中出现取舍，优先保留：

- Rust Core 边界
- Event / Action 协议
- SQLite migration
- Resource Pack
- Adapter 抽象

可以牺牲：

- 动画精致度
- 设置 UI 完整度
- LLM 完整能力
- 视觉细节

---

# 09 · 开源生态参考与方向验证

## 为什么要研究这些项目

Ian 并不是凭空出现的方向。

过去几年里，已经有很多项目分别验证了：

- 程序员愿意长期挂一个桌面宠物
- 桌面 Overlay + AI 是成立的
- Tauri 正在成为轻量桌面应用的新默认方案
- “长期存在于桌面环境中的 AI”会带来强烈陪伴感
- Git / Coding Event 联动有极强传播性

但目前大部分项目：

- 只有“技术感”
- 没有“情绪设计”
- 更像工具，而不是角色

Ian 的机会点在于：

> 把「桌宠」「AI」「程序员文化」「情绪陪伴」「开源生态」真正融合。

不是一个 AI Assistant。

而是：

> 一个长期住在开发者电脑里的生命体。

---

# 09.1 · 值得重点研究的开源项目

## OpenPets

### 定位

AI Coding Agent 的桌面宠物。

GitHub： [https://github.com/alvinunreal/openpets](https://github.com/alvinunreal/openpets)

### 已验证方向

- Agent 状态可视化
- Tray-first companion
- AI 工作流反馈
- 桌面宠物长期驻留
- Claude Code / MCP 集成

### 值得借鉴的产品点

#### 1. Agent 状态映射

不同 AI 状态 → 不同行为：

- Thinking → 发呆 / 思考
- Working → 疯狂打字
- Error → 崩溃 / 摔倒
- Finished → 庆祝

这个机制未来可以接入：

- Git Hook
- Cursor
- Claude Code
- 本地 Agent
- CI/CD
- 编译器状态

让 Ian 成为：

> “你的 AI 工作流的情绪层”。

---

#### 2. Agent Companion 而不是 Chat Box

OpenPets 最大的启发：

AI 不一定要待在聊天窗口里。

它可以：

- 在桌面角落陪你
- 自己观察状态
- 主动产生反馈
- 用动作表达信息

这和传统 Chat UI 完全不同。

Ian 应继续强化：

> “存在感”而不是“功能面板”。

---

### 值得借鉴的技术点

#### 技术架构

- Tauri
- Rust
- 系统 Tray
- Overlay Window
- 动画状态机

说明：

Tauri 已经被越来越多“常驻型桌面 AI”项目验证。

---

## BongoCat Desktop

GitHub： [https://github.com/lucasfrre/BongoCat-Desktop](https://github.com/lucasfrre/BongoCat-Desktop)

### 这是一个非常重要的参考项目

因为它已经证明：

> 程序员真的愿意长期挂一个桌宠。

它长期占据：

- GitHub 热门桌宠项目
- OBS 圈层
- 开发者桌面个性化圈层

---

## 值得借鉴的产品点

### 1. 键盘行为反馈

用户输入 → 宠物立即响应。

这种“即时活物反馈”会非常上瘾。

Ian 后续可以扩展：

- 打字节奏
- 狂敲键盘
- 深夜工作
- 长时间 idle
- Git commit
- 编译成功

都映射成羊驼行为。

---

### 2. 长期桌面驻留体验

BongoCat 最大启发：

桌宠不能烦。

用户会长期挂着它，是因为：

- 不抢焦点
- 不挡操作
- 不突然弹窗
- 不影响性能
- 存在感适中

这是 Ian 必须守住的底线。

---

## 值得借鉴的技术点

### 透明窗口

核心能力：

- 无边框
- Always On Top
- 背景透明
- GPU 加速动画

这是桌宠产品的基础设施。

---

### 点击穿透

透明区域必须穿透到底层窗口。

否则用户会觉得：

“桌宠妨碍我工作”。

基础点击穿透是 P0 级技术要求。

复杂场景，例如全屏 overlay、多窗口穿透、跨显示器坐标与透明区域精细命中，可以放到 v0.1.x 继续打磨。

---

### 多显示器支持

用户拖到副屏后：

- 宠物要跟随
- DPI 要正确
- 坐标系统要统一

很多桌宠项目都在这里踩坑。

---

## bongo-cat-next

GitHub： [https://github.com/liwenka1/bongo-cat-next](https://github.com/liwenka1/bongo-cat-next)

### 值得借鉴的方向

这是“更现代”的桌宠路线。

已经开始：

- Live2D
- 更复杂表情
- 更平滑动画
- 更强互动
- Tauri 化

说明整个生态正在：

> Electron → Tauri

迁移。

这进一步验证了 Ian 当前技术路线是正确的。

---

# 09.2 · 值得借鉴的产品设计

## 1. “它来找你”比“提醒你”更重要

传统提醒工具：

- Notification
- Popup
- Alarm

Ian：

- 主动跑出来
- 叼着水杯
- 趴在窗口上
- 瘫给你看

核心区别：

> 用户不是在“被管理”，而是在“被陪伴”。

这是整个项目最重要的产品原则。

---

## 2. 动作比文字更重要

桌宠最核心的表达方式：

不是聊天。

而是：

- 跑
- 摔
- 趴
- 弹
- 蹭
- 发呆
- 探头
- 偷看

动作是“生命感”的来源。

聊天只是增强。

因此：

MVP 的重点应该是：

> “它活了”

而不是：

> “它功能很多”。

---

## 3. 不可预测性

优秀桌宠都会有：

- 随机彩蛋
- 偶发动作
- 低概率隐藏行为
- 不同心情状态

因为：

可预测 = 工具。

不可预测 = 生命感。

Ian 的“随机心情系统”是非常正确的方向。

---

## 4. 延迟回应

不要所有回应都即时完成。

有时候：

- 停顿
- 思考
- 发呆
- 慢半拍

反而更像活物。

例如：

用户： “今天代码好多 bug。”

羊驼：

“……”

“那今天先别熬太晚了嘛。”

这种停顿会极大增强真实感。

---

## 5. 情绪比 AI 更重要

未来很多项目会卷：

- 更强模型
- 更长上下文
- 更强 Agent

但真正形成用户情感绑定的：

不是模型能力。

而是：

- 被惦记
- 被观察
- 被陪伴
- 被理解

所以：

Ian 不应该成为：

“另一个聊天助手”。

而应该成为：

> “长期陪着你的角色”。

---

# 09.3 · 值得借鉴的技术设计

## 1. Overlay Window 系统

Ian 本质上是：

> 一个系统级 Overlay 应用。

需要解决：

- Always on top
- 无边框
- 背景透明
- 鼠标穿透
- 动画性能
- 多屏支持
- 高 DPI
- 不抢焦点

这些是整个项目真正的基础设施。

---

## 2. 动画状态机

建议从一开始就做：

State Machine。

例如：

- Idle
- Walking
- Running
- Sleeping
- Angry
- Happy
- Thinking
- Talking
- Dragged

所有行为都走状态切换。

否则后期会出现大量逻辑混乱。

---

## 3. 事件驱动架构

推荐：

Event Bus。

例如：

- keyboard\_active
- compile\_success
- git\_push
- user\_idle
- user\_return
- llm\_reply\_start
- llm\_reply\_finish

行为系统监听事件。

这样未来插件系统会非常自然。

---

## 4. Prompt 不只是聊天

System Prompt 不只是：

“你是一只羊驼”。

更重要的是：

实时状态注入。

例如：

- 用户多久没休息
- 最近连续工作多久
- 当前时间
- 当前 mood
- 最近互动频率

这些才会让回复像：

> “真的在观察用户”。

---

## 5. 本地记忆系统

建议：

第一版就保留 memory abstraction。

即使 MVP 不实现长期记忆。

未来可以扩展：

- 最近对话
- 用户昵称
- 最近项目
- 情绪趋势
- 工作节奏
- 偏好

但：

必须 local-first。

因为：

开发者对隐私极度敏感。

---

## 6. Git Hook 是天然传播点

未来最容易 viral 的功能：

```bash
git push
```

羊驼：

- 冲出来撒欢
- 放烟花
- 庆祝
- 跑满屏

或者：

Compile Failed：

“borrow checker 又生气了……”

这种内容天然适合：

- Twitter/X
- Reddit
- 小红书
- Hacker News
- V2EX

传播性极强。

---

# 09.4 · 当前方向的真正机会

现在整个 AI 行业：

都在卷：

- Agent
- IDE
- Workflow
- Chat UI

但：

“长期存在于桌面的 AI Companion”

还几乎没人真正做好。

Ian 的机会是：

> Developer Ambient AI

即：

环境型 AI 陪伴层。

它不是工具栏。

也不是聊天框。

而是：

> 长期存在于开发者电脑里的角色。

这是一个非常新的方向。

---

# 09.5 · MVP 收缩建议（非常重要）

当前方案很完整。

但 MVP 必须极度克制。

建议真正的 P0 / MVP 只做：

1. 透明置顶羊驼
2. 点击弹聊天气泡
3. Demo Dialogue 短句回复
4. 可选 OpenAI-compatible API
5. 可选流式输出
6. 满屏乱跑彩蛋

不要做：

- 长期记忆
- 心情系统
- 好感度
- 主动提醒系统
- 插件系统
- 状态监听
- 多人格
- 编辑器

但工程上仍然要按 v0.1 Architecture Baseline 建好：

- IanEvent / IanAction / IanState
- Rust Core Runtime
- Resource Pack
- Adapter 抽象
- Storage / Migration 骨架
- Security Gate 骨架

先验证：

> 用户会不会想让它长期住在桌面上。

这是整个项目最核心的问题。

---

# 09.6 · 第一阶段真正目标

第一阶段不是：

“做完整产品”。

而是：

> 做出“它活了”的感觉。

只要用户第一次看到：

- 羊驼探头
- 一边跑一边吐槽
- 字一个一个蹦出来
- 被拖拽时乱蹬
- compile 成功后撒欢

然后笑出来。

这个项目就已经成功了一半。

---

# 10 · 飞书中转与跨桌面宠物系统

## 不只是群机器人

当 Ian 接入飞书后，它不再只是一个“桌宠”。

它开始变成：

> 一个跨设备存在的 AI Companion System。

飞书在这里不是“聊天工具”。

而是：

- 身份系统
- 群组系统
- 权限系统
- 消息中转层
- 设备发现层

真正的桌宠渲染，始终发生在用户本地。

这是整个架构最重要的原则。

---

## 10.1 · 群组系统定位

Ian 的群组功能，不应该做成传统机器人。

不是：

- 自动回复
- 群管理
- OA 工具
- Workflow Bot

而应该做成：

> 团队共享的 AI 氛围层。

例如：

- Git push 撒欢
- 编译成功庆祝
- Standup 总结
- 深夜提醒休息
- 团队羊驼日报
- 群组共同宠物

Ian 负责：

> 让团队空间“活起来”。

---

## 10.2 · 飞书为什么适合做中转

飞书天然提供：

- 用户身份
- 群聊关系
- 消息路由
- Bot API
- Webhook
- 事件订阅
- 卡片交互

因此非常适合作为：

> Ian 的社交层。

而不是自己重新做 IM。

---

## 10.3 · 两层架构

推荐采用：

### Layer 1 · 本地桌宠

负责：

- 动画
- 行为
- 本地记忆
- Overlay Window
- 用户交互
- 本地 LLM
- 角色状态

这是“生命体本身”。

---

### Layer 2 · Relay 社交层

负责：

- 飞书消息
- 身份绑定
- 群组关系
- 宠物来访事件
- 群组广播
- 在线状态
- 权限校验

这是“社交网络”。

---

## 10.4 · 核心能力：宠物来访（Pet Visit）

这是整个系统最有想象力的能力之一。

用户不是“给别人发消息”。

而是：

> 把自己的羊驼派去别人桌面串门。

例如：

飞书群：

```txt
@Ian 去找 @张三 review PR
```

然后：

张三桌面：

- 一只羊驼从屏幕边缘跑进来
- 绕桌面转一圈
- 停下来
- 说一句话
- 再跑走

例如：

“主人让我来催你 review PR 哞。”

这种体验：

不是消息。

而是：

> 数字角色迁移。

---

## 10.5 · 正确的实现方式

重要：

不是远程控制别人电脑。

也不是远程桌面。

而是：

> 远程触发本地动画。

即：

A 用户：

- 发起 Pet Visit Event

Relay Server：

- 做身份与权限路由

B 用户本地客户端：

- 收到事件
- 本地渲染 A 的羊驼
- 本地播放动画
- 本地展示消息

因此：

- 性能很好
- 不需要视频流
- 不需要屏幕共享
- 不需要远程控制
- 非常适合开源项目

---

## 10.6 · Pet Identity System

为了实现“我的羊驼去别人电脑”，需要设计：

> 宠物身份系统。

每只羊驼都应该拥有：

- 名字
- 皮肤
- 配色
- 配件
- 性格
- Prompt
- Mood
- 社交关系

未来用户真正“养”的：

不是一个聊天机器人。

而是：

> 自己的数字宠物。

推荐数据结构：

```ts
interface PetIdentity {
  pet_id: string
  owner_id: string

  name: string
  skin: string

  accessories: string[]

  personality_prompt: string

  default_mood: string

  friendship_map?: Record<string, number>
}
```

---

## 10.7 · 两只羊驼同时存在

“来访羊驼”不应该替换“本地羊驼”。

而应该：

同时存在。

例如：

- 用户自己的羊驼：趴在角落
- 来访羊驼：从左边跑进来

然后：

本地羊驼：

“有人来找你哞？”

这种：

> 宠物之间互动

会让整个系统真正拥有“生命感”。

---

## 10.8 · 群体行为（未来）

未来可以扩展：

### 1. 群体撒欢

CI 通过：

所有在线羊驼：

- 一起满屏乱跑
- 放烟花
- 庆祝 release

---

### 2. 宠物结伴

长期协作的成员：

- 羊驼关系变好
- 会一起行动
- 一起睡觉
- 同步动作

---

### 3. 礼物系统

例如：

- 胡萝卜
- 小帽子
- 表情
- 水杯
- 徽章

这些会极大增强：

> 用户对角色的归属感。

---

## 10.9 · 推荐的技术架构

```txt
Ian Desktop
  ├─ Overlay Window
  ├─ Animation Engine
  ├─ Local Memory
  ├─ Personality Engine
  ├─ Local Event Bus
  └─ WebSocket Client
           ↓
Ian Relay Server
  ├─ Feishu Integration
  ├─ Identity Mapping
  ├─ Group Routing
  ├─ Visit Event Router
  ├─ Permission Control
  └─ Presence System
           ↓
Feishu Bot / Webhook
```

---

## 10.10 · Event-Driven 设计

整个社交系统建议完全事件化。

例如：

```ts
interface PetVisitEvent {
  from_user: string
  to_user: string

  pet_profile: {
    skin: string
    mood: string
    accessories: string[]
  }

  action: {
    type: "visit"
    animation: "run_in"
    duration: number
  }

  message: string
}
```

所有社交能力：

- visit
- gift
- celebrate
- group\_party
- summon

本质都只是：

> Event。

这样未来扩展会非常自然。

---

## 10.11 · 隐私与权限（极重要）

这个系统绝不能变成：

“远程骚扰工具”。

因此必须：

### 默认关闭远程来访

用户主动开启。

---

### 首次来访必须确认

例如：

“是否允许 Dezhao 的羊驼来你的桌面串门？”

---

### 勿扰模式

例如：

- 全屏勿扰
- 会议中勿扰
- 夜间勿扰
- Focus Mode

---

### 严格限制数据同步

默认允许：

- 用户昵称
- 羊驼外观
- 自定义消息

默认禁止：

- 代码内容
- 文件内容
- 剪贴板
- 当前窗口文本

Ian 必须坚持：

> local-first。

---

## 10.12 · 推荐落地阶段

### Phase 1

本地桌宠。

不做社交。

---

### Phase 2

飞书 Webhook 群播报。

例如：

- Git push
- 编译成功
- 每日总结

---

### Phase 3

Pet Visit。

即：

“去别人桌面转一圈”。

---

### Phase 4

群组行为。

例如：

- 群体撒欢
- 宠物关系
- 礼物系统
- 群组记忆

---

## 10.13 · 长期方向

Ian 真正的长期方向：

不是聊天机器人。

而是：

> 有空间位置、能移动、能串门、长期存在的 AI Presence。

它不住在聊天框里。

而是：

> 住在你的电脑世界里。
