# 决策记录: Curiosity Attention Reactions

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 好奇反应只基于 Ian 窗口内 pointer。 | 这是生命感核心，但不能引入高敏全局监听。 | 保持 P0/v0.1.x 隐私边界。 |
| 2026-05-21 | `MouseNear` 增加 Rust Core cooldown，不由 React 防抖。 | 验收发现频繁 hover 会重复触发 happy。 | `MouseNear` 携带窗口内事件时间；cooldown 内不重复输出 attention action，`MouseLeave` 只回 idle、不重置 cooldown。 |
