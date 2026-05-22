# 0126 · 决策记录

## 2026-05-21

- 本轮按用户授权连续实现 0110-0159，范围保持为 v0.1.x / future skeleton：默认关闭、摘要事件、本地优先，不把 P0 变成完整开发工具或 AI assistant。
- Adapter / Developer Rhythm skeleton 只做最小可运行骨架和可验证入口；任何联网、代码正文读取、窗口标题读取、全局键盘文本、剪贴板、远端消息和插件执行都不在本轮启用。
- 新增状态只持久化低敏配置：BYOM 是否已有本地 key 标记、提醒间隔、勿扰、隐私 onboarding 是否看过；API key 本体进入本地 secret store，不写入 config。
- 高敏 adapter 在权限注册表中默认关闭；未知 adapter 默认拒绝。
- 0112/0113/0131-0139/0140-0149 中尚未具备完整产品面的部分，沿用已有 provider / policy / test harness / desktop shell 骨架，不新增默认可见能力。

## Scope Changes

- 将 0159 隐私优先 onboarding 并入权限中心设置面板，以中文文案呈现“本地优先 / 高敏默认关闭 / 逐项授权”。
- 为支撑 0111、0115-0117、0121、0150-0158，增加少量 Rust 状态、config 默认值、secret store、repository、migration 和 sanitizer 测试。

## Deferred Work

- BYOM 真实 provider key 管理 UI、流式气泡完整渲染、诊断导出文件、发布签名、crash boundary、性能预算自动化和完整 onboarding 向导留到后续 SDD。
- Developer Rhythm 仍是 opt-in skeleton，不默认绑定工作区，不读取代码正文或路径。
