# Spec: Developer Rhythm Onboarding

## 状态

已实现，待验收。

## 问题 / 目标

Ian 已经具备 Developer Rhythm 的早期 adapter 和权限边界，但用户需要明确知道这些能力会读取什么、不会读取什么。0041 目标是建立开发者节奏授权引导，默认关闭，用户显式开启后才进入 v0.2 能力。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 设置面新增 Developer Rhythm 入口或分组。
- 显示每类能力的数据范围：Git 元数据、build/test 摘要、键盘节奏、应用类别。
- 用户可一次性开启基础低敏能力，也可逐项开关。
- 默认所有开发者感知能力关闭。

## 明确不做什么

- 不默认读取项目、键盘或活动应用。
- 不做长篇隐私政策页面。
- 不做账号登录、云同步或远程授权。
- 不把 Ian 变成 IDE 助手。

## 用户体验

用户看到的是“让 Ian 理解一点点你的开发节奏”，并能清楚知道 Ian 不读取代码正文、不读取终端全文、不记录按键内容。

## 架构约束

- 授权状态存本地配置。
- Security Gate 必须消费同一授权状态。
- React 设置面只修改授权，不绕过 Rust 权限检查。

## 数据 / 协议变化

可扩展 capability config 和 settings command。已有字段优先复用：`git_metadata_enabled`、`build_test_events_enabled`、`keyboard_rhythm_enabled`、`active_app_presence_enabled`。

## 隐私与安全边界

默认关闭。授权文案必须明确敏感非目标：代码正文、diff、终端全文、按键内容、窗口标题、URL、屏幕 OCR。

## 验收标准

- [ ] Developer Rhythm 入口默认显示为关闭状态。
- [ ] 用户可分别开关 Git、build/test、keyboard rhythm、active app presence。
- [ ] 每项开关旁有简短数据范围说明。
- [ ] 未授权时 Security Gate 继续拒绝对应事件。
- [ ] 测试覆盖默认关闭、开关持久化、未授权拒绝。

## 验证方式

- Rust config / security tests。
- Frontend settings tests。
- Browser smoke：开关状态、说明文案和持久化。
