# 0113 · Dialogue Provider Fallback V1

## 问题 / 目标

建立 provider 失败时的降级策略，让 Ian 在网络、key 或模型错误时回到本地 Demo Dialogue。

## 当前产品阶段

v0.1.x Reliability。

## 产品范围

- 识别常见 provider 错误。
- 输出短错误气泡或静默降级。
- 保留 Demo fallback。

## 明确不做

- 不做复杂重试队列。
- 不暴露原始错误堆栈给用户。
- 不联网诊断。

## 用户体验

用户看到的是 Ian 更稳定、更可控、更有生命感；如果本 SDD 属于未来能力准备，则默认不主动暴露成打扰性的用户可见功能。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责渲染、设置 UI、动画播放和执行 IanAction。
- 外部输入必须通过 IanEvent、Security Gate 或既有设置 command 进入系统。
- 高敏 adapter 必须默认关闭，并经过权限中心和 Security Gate。
- Resource Pack 不执行脚本、不访问网络、不携带行为逻辑。
- 不把 v0.2 / v0.3 能力提前做成默认用户可见功能。

## 数据 / 协议变化

默认不新增 IanEvent / IanAction / IanState 字段。若实现发现必须变更协议，必须先更新本 spec、plan 和 decisions.md，再进入实现。

## 隐私与安全边界

- 默认不读取代码正文、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出。
- 只允许本 SDD 明确列出的低敏本地数据进入持久化。
- API key、用户正文和敏感 payload 不得写入普通日志或 config.toml。

## 验收标准

- [ ] provider 失败不会导致气泡卡死。
- [ ] Demo fallback 可用。
- [ ] 错误提示短且角色一致。
- [ ] 日志记录错误类别。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行 workspace 类型检查或记录无法运行原因。
- 涉及桌面可见能力时，必须在真实 Tauri 桌面壳中验收。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
