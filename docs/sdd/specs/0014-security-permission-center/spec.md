# Spec: Security Permission Center

## 状态

已实现，待验收。

## 问题 / 目标

Ian 后续会逐步接入 Reminder、BYOM、Developer Rhythm 和更多 adapter。0014 目标是把权限、敏感源、默认关闭策略和用户控制面收紧，避免后续功能扩展时隐私边界漂移。

## 当前产品阶段

v0.1.x Architecture Hardening。

## 产品范围

- Rust Security Gate 明确 source sensitivity。
- 设置面展示已启用/未启用的本地能力。
- 高敏感 adapter 默认关闭。
- 对外部输入做统一大小限制和 sanitizer。

## 明确不做什么

- 不实现 Feishu、Git、键盘监听。
- 不读取剪贴板、屏幕 OCR、代码内容。
- 不做插件系统。
- 不做复杂权限审计日志 UI。

## 用户体验

用户能看到 Ian 当前只启用了哪些本地能力，并能关闭提醒/对话等可控项。

## 架构约束

- 所有 adapter 经过 Security Gate。
- Permission 属于 Rust Core / Security 层。
- React 只展示权限状态和发送用户授权选择。

## 数据 / 协议变化

可新增 permission state view 和配置项；高敏感能力必须默认 false。

## 隐私与安全边界

默认只允许时间、窗口内鼠标、用户主动输入和本地资源。其他能力必须显式授权。

## 验收标准

- [ ] Security Gate 对 source/sensitivity/permission 有单元测试。
- [ ] 默认配置不启用高敏感 adapter。
- [ ] 设置面可展示当前启用能力。
- [ ] sanitizer 对过长文本和基本危险字符有测试。
- [ ] 现有 P0/v0.1.x 行为不需要额外系统权限。

## 验证方式

- Rust security tests。
- Config tests。
- 前端设置面测试。
