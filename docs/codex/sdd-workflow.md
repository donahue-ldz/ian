# SDD 开发流程

SDD 指 Spec-Driven Development，即规格驱动开发。

在本仓库中，SDD 的目的不是增加文档负担，而是防止 AI 实现时范围漂移、架构漂移和产品阶段漂移。

用户只需要用自然语言描述目标。AI 负责在开发前结构化思考，并生成、维护对应的 SDD 文档。

## 核心规则

用户不需要手写 SDD 文档。

对于非平凡任务，AI 必须在实现前生成 SDD packet，并在开发过程中持续维护。

SDD packet 包括：

- `spec.md`
- `plan.md`
- `decisions.md`
- `verification.md`

## 硬性约束

- 在 `spec.md` 和 `plan.md` 创建之前，不得开始实现。
- 未获得用户确认前，不得开始实现；除非用户在同一轮对话中明确授权继续执行。
- 实现内容必须严格限制在已确认的 `spec.md` 范围内。
- 不得把未来阶段功能提前做成 P0 用户可见功能；如需调整，必须先更新 spec 并获得确认。
- SDD 文档内容中文优先；除代码标识、命令、文件路径、协议类型、API 名称和必要英文术语外，正文应使用中文。
- 每一条验收标准都必须客观可验证。
- 每一个实现步骤都必须对应一个或多个验收标准。
- 任何范围变化、设计变化或临时取舍，都必须记录到 `decisions.md`。
- 声称完成前，必须更新 `verification.md`，记录实际验证方式和结果。

## 什么时候必须使用 SDD

以下任务必须先生成 SDD 文档，再进入实现：

- 新功能
- 架构调整
- 新 app、package、module 的脚手架
- 多文件改动
- 行为逻辑变化
- 协议变化
- 数据模型或持久化变化
- 安全、权限、隐私相关改动
- 影响 Ian 产品阶段边界的改动

以下任务可以跳过完整 SDD：

- 拼写修正
- 格式调整
- 只读分析
- 无行为影响的简单文档修改
- 无行为影响的简单单文件修改

不确定时，默认使用 SDD。

## SDD Packet 结构

每个有明确范围的开发任务使用一个目录：

```txt
docs/sdd/specs/<id>-<slug>/
  spec.md
  plan.md
  decisions.md
  verification.md
```

编号使用四位递增数字：

```txt
0001-p0-local-creature-proof
0002-resource-pack-renderer
0003-dialogue-provider-boundary
```

## 执行流程

### 1. 理解需求

开发前先读取：

- `AGENTS.md`
- `docs/codex/ian-current-stage.md`
- `docs/codex/ian-architecture-rules.md`
- `ian.md` 中相关章节
- 与任务相关的已有代码和文档

然后明确：

- 用户目标
- 当前产品阶段
- 受影响的架构边界
- 明确不做什么
- 可能的验证方式

### 2. 生成 `spec.md`

AI 负责写 spec。

spec 必须包含：

- 问题 / 目标
- 当前产品阶段
- 产品范围
- 明确不做什么
- 用户体验
- 架构约束
- 数据 / 协议变化
- 隐私与安全边界
- 验收标准
- 验证方式

spec 不得悄悄扩大用户请求或当前阶段允许的范围。

验收标准必须使用 checkbox，并且必须可客观验证。

不合格示例：

```md
- [ ] Ian 感觉不错
```

合格示例：

```md
- [ ] 双击 Ian 后，前端发送 `MouseDoubleClick`，Rust Core 返回 `BehaviorRunAround`，React 播放 run 动画。
```

### 3. 生成 `plan.md`

AI 负责写实现计划。

plan 必须包含：

- 有序实现步骤
- 预计改动文件
- 受影响的接口或模块
- migration / 兼容性说明
- 验证命令或手动检查方式
- 风险和回滚说明

每个实现步骤必须对应一个或多个 spec 验收标准。

### 4. 获得确认

实现前，AI 必须向用户展示 spec 和 plan 摘要。

只有满足以下任一条件，才能开始实现：

- 用户确认 spec 和 plan。
- 用户在同一轮对话中已经明确授权“生成 SDD 后继续执行”。

如果用户改变方向，先更新 spec 和 plan，再继续。

### 5. 严格按 spec 实现

实现过程中：

- 不得添加 spec 之外的用户可见功能。
- 不得把 v0.2 / v0.3 功能提前做成 P0 用户可见行为。
- 未来阶段能力只能以 skeleton、trait、interface、no-op、schema placeholder 或明确 non-goal 的形式出现。
- 必须保持 React、Rust Core、Adapter、Storage、Security 的职责边界。
- 重要范围变化、设计变化、临时取舍必须记录到 `decisions.md`。

如果实际代码情况使原计划失效，停止实现，先更新 SDD packet，再继续。

### 6. 验证并记录

实现后，运行与改动相关的最小有效验证。

在 `verification.md` 中记录：

- 执行了什么命令或手动检查
- 结果是什么
- 哪些检查失败
- 哪些检查跳过
- 剩余风险
- 后续建议

没有 `verification.md` 中的验证记录，不得声称任务完成。

## 防漂移检查

如果实现过程中开始偏离，使用以下检查：

- 这是否在已确认的 spec 中？
- 这是否符合当前产品阶段？
- 这是否仍然让 Ian 像数字生命，而不是 AI Assistant？
- 这是否保持 Rust Core 作为行为大脑？
- 输入和输出是否仍然通过 `IanEvent` / `IanAction`？
- 这是否把本应只是 skeleton 的能力做成了用户可见功能？

如果任一答案是否定的，必须更新 `decisions.md`，必要时更新 spec / plan，并获得确认后再继续。

## 当前第一份 SDD Packet

第一份 SDD packet 建议是：

```txt
docs/sdd/specs/0001-p0-local-creature-proof/
```

它应该覆盖：

- P0 产品验证
- v0.1 Architecture Baseline

它不应该覆盖完整 Ian 产品。
