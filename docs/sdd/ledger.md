# SDD 台账

更新时间：2026-05-22

本台账用于继续 SDD 开发前快速判断状态。它不删除历史目录，不伪造验收结果；状态以各 SDD 的 `verification.md`、实际代码和本轮扫描为准。

## 状态口径

| 状态 | 含义 | 下一步 |
| --- | --- | --- |
| 已实现待验收 | verification 记录已有通过命令，但仍标注待用户验收或缺少人工桌面验收 | 需要桌面端复验或用户验收后再归档 |
| 待执行 | `verification.md` 仍为“待实现后更新” | 可按编号继续开发 |
| 重复 / 被覆盖 | 早期草稿、重复编号或后续 SDD 已覆盖同一目标 | 不直接执行，先核对后续 SDD |
| Skeleton | 架构骨架存在，但不是完整产品化能力 | 只能默认关闭继续演进 |

## 编号冲突

| 编号 | 目录 | 状态 | 风险 | 下一步 |
| --- | --- | --- | --- | --- |
| 0002 | `0002-p0-acceptance-hardening`、`0002-resource-pack-renderer` | 重复 / 被覆盖 | 前者是草稿收口，后者已有实现验证 | 后续开发以资源包实现和更高编号验收为准 |
| 0003 | `0003-dialogue-provider-boundary`、`0003-resource-pack-renderer` | 重复 / 被覆盖 | `0003-resource-pack-renderer` verification 未执行，内容被 `0002-resource-pack-renderer` 覆盖 | 不直接执行重复草稿 |
| 0004 | `0004-basic-behavior`、`0004-dialogue-provider-boundary` | 重复 / 被覆盖 | dialogue boundary 已由 `0003-dialogue-provider-boundary` 验证 | 不直接执行重复草稿 |
| 0005 | `0005-bubble-dialogue`、`0005-local-state-persistence` | 重复 / 部分覆盖 | persistence 草稿未执行，后续 hardening 已覆盖 | 以 `0006-local-state-persistence-hardening` 为准 |
| 0012 | `0012-kitten-resource-pack`、`0012-reminder-engine` | 重复 | 两个主题不同；kitten resource pack 有桌面拖拽验收阻塞记录 | 后续用目录 slug 区分 |
| 0080 | `0080-desktop-pet-bubble-reference-pass`、`0080-pointer-chase-playful-behavior` | 重复 | 两条体验线并行 | 后续验收按 slug 分开 |
| 0082 | `0082-centered-bubble-text`、`0082-puppy-micro-life-upgrade` | 重复 | 文本布局与资源包体验并行 | 后续验收按 slug 分开 |
| 0083 | `0083-bubble-text-full-center`、`0083-click-triggered-pointer-chase` | 重复 | 气泡和交互能力并行 | 后续验收按 slug 分开 |
| 0086 | `0086-desktop-liveliness-tuning`、`0086-smooth-desktop-window-movement` | 重复 | 桌面窗口能力需要真实 Tauri 验收 | 后续验收按 slug 分开 |

## 范围状态

| SDD 范围 | 状态 | 剩余风险 | 下一步 |
| --- | --- | --- | --- |
| 0001 | 已实现待验收 | P0 端到端仍依赖真实桌面复验 | 保留为 P0 根验收 |
| 0002-0011 | 已实现 / 重复草稿已标注 | 部分早期草稿 verification 为空，易被误执行 | 以后按已实现后续 SDD 为准 |
| 0012-0014 | 已实现待验收 | 0012 kitten 的自动桌面拖拽受 macOS Accessibility 阻塞 | 需要人工桌面验收 |
| 0015-0020 | Skeleton / 已验证 | Developer Rhythm 属 v0.2，不应默认变成 P0 用户可见能力 | 保持默认关闭 |
| 0021-0060 | 已实现待验收 | 多数 verification 已有自动测试，但人工桌面验收深度不一 | 桌面能力变更时用统一 smoke 清单补验 |
| 0061-0090 | 已实现待验收 | playful / resource / movement 功能多，需防止用户控制弱化 | 回归时重点看安静模式、拖拽、气泡遮挡 |
| 0091-0120 | 已实现待验收 | v0.1.x 能力增多，BYOM / reminder / permission 必须保持默认克制 | 继续补隐私和桌面验收 |
| 0121-0159 | 已实现或 Skeleton | adapter / security 相关能力不能读取敏感源 | 继续以 source scan 和拒绝路径测试验证 |
| 0160-0189 | Skeleton | 这是 memory/social/plugin/platform 的最小骨架，不是完整产品化能力 | 只能作为默认关闭的后续基础 |
| 0190-0191 | 已实现，验收待补 | 桌面拖动和找回需要真实 Tauri 验收记录 | 由 0193/0194 收口 |
| 0192-0201 | 已执行待验收 | 记忆能力涉及隐私；verification 已记录自动测试和 Tauri 壳启动，人工设置面点击仍可补验 | 如用户要求提交，先整理同一工作树内连续改动 |
| 0202-0231 | 已执行待验收 | verification 已记录自动测试和 Tauri 壳启动；仍保留未提交状态 | 如用户要求提交，先整理同一工作树内连续改动 |
| 0232-0251 | post-0231 readiness 已执行 | 范围限定为小型 hardening、审计和默认关闭边界；仍保留未提交状态 | 后续只做回归或提交整理，不再作为当前待执行队列 |
| 0252-0261 | Moment System 已执行 | verification 已记录自动测试和真实 Tauri 启动 smoke；逐项人工视觉操作仍可按验收清单补验 | 后续视觉调参或人工验收应引用 `docs/sdd/moment-desktop-acceptance.md` |
| 0262-0269 | Moment 体验调参与控制已执行 | 新增诊断触发、用户控制 guardrail、气泡时长上限和桌面启动 smoke；长时间人工观察仍可补验 | 如继续推进，应先提交或整理当前连续未提交改动，再生成下一批 SDD |

## 当前待执行队列

| 编号 | 名称 | 状态 | 剩余风险 | 下一步 |
| --- | --- | --- | --- | --- |
| 无 | 当前没有待执行 SDD | 已扫描 `verification.md` 待实现标记和最高编号目录 | 仍有大量未提交改动，不能在未整理前继续叠加新范围 | 如用户要求继续新方向，先生成下一批 SDD spec / plan 并确认范围 |
