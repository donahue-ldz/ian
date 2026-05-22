# 0098 · 验证记录

## 2026-05-21

### 自动化验证

- RED：新增 `sendArmedChaseCandidate` 在设置打开时不得读取桌面 cursor 的测试后失败。
- GREEN：定向前端测试通过；全量 `desktop:test` 84/84 通过。
- Rust：`pointer_chase_*` 相关测试继续通过。

### 桌面 / 预览验收

- 浏览器预览确认设置面板可打开；真实 Tauri shell smoke 启动成功。

### 剩余风险

- 未用真实鼠标在桌面壳逐项验证靠近 / 离开追随动作，仅完成策略测试和启动 smoke。
