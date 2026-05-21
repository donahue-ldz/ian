# 决策记录: Local State Persistence Hardening

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 0006 先做本地状态收口，再进入更多体验功能。 | Mood/Bond/Reminder 都依赖稳定本地状态。 | 后续功能复用 storage 边界，不重复改架构。 |
| 2026-05-21 | 位置恢复必须连接到真实窗口定位，而不只停留在 config round-trip。 | 0001 验收发现 Rust config 测试通过但启动窗口仍总是回到右下角。 | `configure_main_window` 优先使用已保存 position，前端加载 state 后也会调用 Tauri window position API。 |

## 范围变化

暂无。

## 延后工作

- 长期记忆
- 云同步
- 用户可见历史记录
