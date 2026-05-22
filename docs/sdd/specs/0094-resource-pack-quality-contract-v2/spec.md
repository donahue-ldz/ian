# 0094 · Resource Pack Quality Contract V2

## 问题 / 目标

把当前多资源包经验沉淀成更严格的本地质量合同，避免后续新增资源包出现缺帧、尺寸不一致、关键语义缺失或黑屏。

## 当前产品阶段

v0.1 Architecture Baseline hardening。

## 产品范围

- 扩展 resource pack 测试覆盖所有 bundled packs。
- 检查关键动画、尺寸、fallback、风格标记和安全属性。
- 输出资源包作者最低要求文档。

## 明确不做

- 不做在线资源包校验服务。
- 不做第三方资源包安装。
- 不改变现有运行时协议。

## 用户体验

用户感知到的是 Ian 的基础生命感和可控性继续变稳：行为更清楚、更可恢复、更少打扰；任何新增能力都保持短句、桌面宠物、低敏和本地优先。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责渲染、动画播放、设置 UI 和执行 IanAction。
- 外部或桌面输入必须先转换为 IanEvent 或通过既有设置 command 进入 Rust / Storage 边界。
- Resource Pack 只描述本地资源，不执行脚本、不访问网络。
- 不把 v0.2 / v0.3 能力提前做成 P0 用户可见功能。

## 数据 / 协议变化

除本 SDD 实现时明确记录到 decisions.md 的最小必要变更外，默认不新增 IanEvent / IanAction / IanState 字段。若实现发现必须改协议，先更新本 spec 和 plan，再实现。

## 隐私与安全边界

- 默认不读取代码内容、剪贴板、屏幕文字、私聊内容或全局键盘。
- 只使用 Ian 自身窗口事件、本地配置、bundled resource pack 和用户明确设置。
- 所有持久化内容必须是低敏本地数据，并避免记录用户正文或敏感值。

## 验收标准

- [ ] 所有 bundled resource pack 均通过统一质量测试。
- [ ] 缺少关键动画、frame 越界或 SVG 缺少风格标记时测试失败。
- [ ] 资源包作者文档说明必须字段、推荐动画和 fallback 规则。
- [ ] 质量合同不执行资源包脚本、不访问网络。

## 验证方式

- 运行与改动相关的 Rust / frontend 单元测试。
- 运行 npm run desktop:typecheck 或等价的 workspace typecheck。
- 涉及桌面可见行为时，必须启动真实 Tauri 桌面壳验收，并在 verification.md 记录窗口位置、动画、气泡、点击 / 拖动或设置操作结果。
- 如果桌面端无法验收，必须记录原因、替代验证和剩余风险。
