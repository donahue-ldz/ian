# 0193 · 0190 拖动稳定性验收收口

## 问题 / 目标

对 0190 多显示器拖动稳定性做真实验收和缺口补齐，确认 Ian 在多屏混合缩放环境下拖动不跳动、不回弹。

## 当前产品阶段

Desktop Creature Usability。

## 产品范围

- 核对桌面端拖动是否只由 Tauri 原生 startDragging 驱动。
- 确认 pointer move 不再同时 setPosition。
- 拖动结束读取真实 outerPosition 并保存。
- 在真实三屏桌面端验收并更新 verification。

## 明确不做

- 不做自动跨屏漫游。
- 不新增窗口感知或屏幕内容读取。
- 不重做拖动交互设计。

## 用户体验

用户按住 Ian 拖动时，窗口应跟随鼠标；松手后留在释放位置，重启后恢复。

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

- [ ] 桌面端拖动期间不再同时执行手动 setPosition 位移。
- [ ] 拖动结束保存 Tauri outerPosition。
- [ ] 浏览器预览拖动仍可用。
- [ ] 真实 Tauri 桌面端三屏拖动验收记录完整。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行类型检查；如无法运行，记录原因。
- 涉及桌面可见行为时，启动真实 Tauri 桌面壳验收，记录步骤和结果。
- 涉及安全 / 隐私时，覆盖拒绝路径、默认关闭和脱敏检查。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
