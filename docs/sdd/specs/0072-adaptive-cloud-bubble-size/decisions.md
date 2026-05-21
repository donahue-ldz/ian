# Decisions: Adaptive Cloud Bubble Size

| 日期 | 决策 | 原因 |
| --- | --- | --- |
| 2026-05-21 | 使用 `data-cloud-size` 做三档布局，不把尺寸写入协议。 | 气泡大小是 React 视觉布局问题，不属于 Rust Core 行为。 |
| 2026-05-21 | 短句最大宽度收敛到 112px，输入状态固定到 176px。 | 短句不应默认大块占屏；输入控件需要保留可用宽度。 |
