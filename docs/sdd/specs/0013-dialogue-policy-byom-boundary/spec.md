# Spec: Dialogue Policy and BYOM Boundary

## 状态

已实现，待验收。

## 问题 / 目标

0003/0005 已建立 Demo Dialogue 和气泡输入。0013 目标是增强 DialoguePolicy，并建立 BYOM 的安全边界和配置占位；是否开放真实模型调用由验收时再定，默认不让 Ian 变成 ChatGPT 桌宠。

## 当前产品阶段

v0.1.x Product Iteration。

## 产品范围

- DialoguePolicy 增加身份约束、长度约束、安全裁剪。
- BYOM provider config 只做本地结构和安全存储边界。
- 默认仍使用 Demo Provider。
- 可提供“未启用真实模型”的设置占位或内部配置，不作为主体验。

## 明确不做什么

- 不默认请求网络。
- 不强制用户配置 API Key。
- 不做聊天历史。
- 不做 Markdown 长回复。
- 不把 Ian 定位成 AI assistant。

## 用户体验

Ian 的回复更稳定、短、像桌面生命；没有配置真实模型时仍完全可用。

## 架构约束

- DialogueProvider 边界保持在 Rust Core。
- 所有 provider 输出必须经过 DialoguePolicy。
- Secret 或 API Key 不能明文写入日志。

## 数据 / 协议变化

可新增 provider config 类型和 secure secret placeholder；如未实现真实调用，必须明确 no-op。

## 隐私与安全边界

默认不访问网络。任何真实 provider 必须显式启用，并经过权限与密钥存储策略。

## 验收标准

- [ ] DialoguePolicy 对长度、身份措辞和空输出有测试。
- [ ] Demo Provider 输出始终经过 DialoguePolicy。
- [ ] BYOM config 边界存在，但默认不发起网络请求。
- [ ] 不记录 API Key 或完整敏感输入日志。
- [ ] 未配置 BYOM 时 Demo Dialogue 正常工作。

## 验证方式

- Rust dialogue policy 测试。
- Security/secret 测试。
- 前端输入 smoke。
