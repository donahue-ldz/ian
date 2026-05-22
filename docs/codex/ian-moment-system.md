# Ian Moment System

本文档定义 Ian Moments 的技术方案和产品边界。

Moment System 要解决的问题是：Ian 已经有很多单点能力，但“惊喜感”和“生命感”不应该来自更多随机动画，而应该来自短小、低频、时机合适的小场景编排。

## 产品定位

Ian Moments 是低频、短时、可打断的数字生命小瞬间。

它们用于让 Ian 更像活在桌面上：

- 被叫回时有一个短入场
- 注意到鼠标时表现出好奇
- 被拖动时像被抱起
- 被放下后轻轻安顿
- idle 时偶尔出现低频小惊喜
- 偶尔回响用户确认过的本地小记忆

它们不是：

- 通知系统
- 任务自动化系统
- 聊天系统
- 社交在线状态
- 插件行为
- 全局键盘监听

Moment System 属于 v0.1.x Life Feel，不扩大 P0 范围。

## 核心原则

惊喜感来自编排，不来自数量。

Moment 行为必须是：

- 低频
- 短时
- 温柔
- 可打断
- 用户可控
- 本地优先
- 隐私安全

如果某个 moment 变得吵、操控感强、像工具通知，应该降低频率、延后或移除。

## 架构位置

Moment Orchestrator 属于 Rust Core，靠近 `BehaviorEngine` / `BehaviorPolicy`。

React 不能成为 moment 行为大脑。

推荐边界：

```txt
IanEvent + IanState + local policy context
  -> Rust Core Moment Orchestrator
  -> IanAction[]
  -> React / Tauri executors
```

React 可以：

- 渲染 sprite
- 播放动画
- 显示气泡
- 执行移动动作
- 执行视觉效果
- 展示设置

React 不得：

- 决定核心 moment 时机
- 维护长期 moment 状态
- 绕过 Rust Core 冷却
- 为 moment 决策直接读取 storage
- 在 `IanAction` 之外创建随机生命行为

## 输入

允许输入仅限现有低敏 Ian 信号：

- `IanEvent`
- `IanState`
- time tick
- Ian 窗口内鼠标事件
- drag start / drag end
- find-Ian 触发
- 本地配置
- 用户确认过的低敏 memory tags

默认禁止输入：

- 代码正文
- 剪贴板
- 私聊内容
- 屏幕 OCR
- 全局键盘文本
- 完整终端输出
- 任意文件内容

未来更高敏感度来源必须经过 Adapter、Security Gate、显式授权和独立 SDD。

## 输出

Moment 输出应优先复用已有 `IanAction`：

- `animation.play`
- `movement.move_to`
- `speech.show`
- `bubble.open`
- `effect.play`
- 已存在的 appearance / playful state action

只有在现有动作无法表达时，才新增协议。诊断 action 必须保持低敏，不应向用户暴露内部分数。

## 冷却与预算

每个 moment 都必须同时通过本地冷却和全局预算。

必须抑制的场景：

- 用户正在拖动 Ian
- 用户正在气泡里输入
- 设置面板打开
- 勿扰开启
- reduced motion 要求降低动效
- 另一个 active moment 还在收尾

Moment System 应优先选择少而有意义的瞬间，而不是频繁特效。

## 记忆边界

Memory Echo moment 只能使用 confirmed 低敏 memory tags。

不得使用：

- 未确认的 memory candidates
- 原始聊天文本
- 用户画像总结
- 代码内容
- 路径
- 私密消息

用户删除或清空记忆后，后续 Moment 决策必须停止使用对应内容。

## SDD 映射

第一批 Moment System 切片：

- `0252-moment-orchestrator-v1`：核心编排边界
- `0253-find-ian-entrance-moment`：找回入场瞬间
- `0254-pointer-curiosity-moment`：鼠标好奇瞬间
- `0255-drag-carry-moment`：拖动抱起瞬间
- `0256-drop-settle-moment`：放下安顿瞬间
- `0257-rare-idle-surprise-moments`：低频 idle 惊喜
- `0258-memory-echo-moment`：已确认记忆回响
- `0259-moment-cooldown-and-budget`：冷却与预算策略
- `0260-moment-desktop-acceptance-suite`：真实桌面验收

技术方案文档切片：

- `0261-moment-system-architecture-doc`

## 验收规则

Moment 必须能在真实 Tauri 桌面壳中验收，才算可接受。

浏览器预览、单元测试和类型检查都很有用，但不能替代以下能力的桌面验收：

- 窗口移动
- 鼠标交互
- 拖动行为
- 找回 Ian
- 动画时机
- 气泡位置
- 视觉效果强度
