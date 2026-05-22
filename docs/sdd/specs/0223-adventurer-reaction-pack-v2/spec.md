# 0223 · 小冒险家反应包 V2

## 问题 / 目标

为小冒险家资源包补充更丰富的反应动作，使找回、睡醒、撒娇、挥手更贴合角色。

## 当前产品阶段

Visual Expressiveness。

## 产品范围

- 新增或映射反应动画。
- 保持像素风格一致。
- 更新 manifest 和测试。

## 明确不做

- 不再次更换主角色方向。
- 不引入大体积视频。

## 用户体验

Ian 的小冒险家形态更灵动，动作语义更清楚。

## 架构约束

- Rust Core 继续作为 Ian 的行为大脑；React 只负责渲染、设置 UI、动画播放和执行 IanAction。
- 所有外部输入必须进入 IanEvent，并经过既有 Security / Permission / Sanitizer 边界。
- 不得把未来阶段能力提前做成默认开启的 P0 用户可见功能。
- 涉及桌面窗口、动画、气泡、拖动、快捷键或 Tauri API 的能力，必须真实 Tauri 桌面端验收。

## 数据 / 协议变化

实现前必须盘点现有 IanEvent / IanAction / IanState、config、SQLite schema 和资源包 manifest。若需要新增协议、配置或 migration，必须在 decisions.md 中记录兼容和回滚策略；若不需要变更，也必须在 verification.md 中确认。

## 隐私与安全边界

- 默认本地优先，不上传用户数据。
- 不读取代码正文、diff、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出，除非本 SDD 明确授权且默认关闭。
- API key、路径、用户正文和敏感 payload 不得进入普通日志或导出。
- 新增能力必须可关闭，并明确记录默认状态。

## 验收标准

- [ ] 新增 find/wake/wave/affection 语义动作或 fallback。
- [ ] 资源包合同测试通过。
- [ ] 桌面目视验收记录。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行类型检查；如无法运行，记录原因。
- 涉及桌面可见行为时，启动真实 Tauri 桌面壳验收，记录步骤和结果。
- 涉及安全 / 隐私时，覆盖拒绝路径、默认关闭和脱敏检查。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
