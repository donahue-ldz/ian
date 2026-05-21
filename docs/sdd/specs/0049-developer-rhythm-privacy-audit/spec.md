# Spec: Developer Rhythm Privacy Audit

## 状态

已实现，待验收。

## 问题 / 目标

v0.2 接入更多开发者上下文后，必须有可重复的隐私审计。0049 目标是建立本地 privacy audit：检查事件 payload、adapter 源码路径、日志字段和诊断输出，证明 Ian 没有读取或保存敏感内容。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 建立 privacy audit checklist。
- 添加 source scan 或测试，覆盖禁用字段和禁用 API。
- 审计 Git、build/test、keyboard、active app、diagnostics。
- 记录审计结果到 verification。

## 明确不做什么

- 不做合规法律文档。
- 不上传审计结果。
- 不扫描用户项目内容。
- 不引入第三方遥测。

## 用户体验

普通用户不需要看到审计细节，但开发团队可以证明 Developer Rhythm 的隐私边界是可验证的。

## 架构约束

- Security / Sanitizer / Permission 是审计核心。
- Adapter 必须声明 source 和 sensitivity。
- 诊断日志必须使用 allowlist。

## 数据 / 协议变化

可新增审计文档和测试 helper。不新增产品协议。

## 隐私与安全边界

审计本身不能读取用户项目内容，只检查代码路径、测试 fixture 和本地配置。

## 验收标准

- [ ] 有 Developer Rhythm privacy audit checklist。
- [ ] Source scan 覆盖 diff、code body、stdout/stderr、key/text、window title、URL、OCR、clipboard。
- [ ] Sanitizer 测试覆盖敏感字段拒绝。
- [ ] Diagnostics 不记录敏感 payload。
- [ ] verification 记录审计命令和结果。

## 验证方式

- `rg` source scan。
- Rust security / sanitizer tests。
- 文档 checklist 自检。
