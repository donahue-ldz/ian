# 验证记录: Core Life Telemetry Local

## 状态

已实现，待用户验收。

## 自动验证

- [x] Life event / diagnostics repository path covered by Rust storage tests.
- [x] Runtime records low-sensitive `diagnostic.behavior_decision` summaries when diagnostics are enabled.
- [x] Source scan - app source 未新增窗口标题、屏幕 OCR、终端输出、代码内容或全局输入读取。
- [x] `npm run desktop:acceptance` - passed。

## 备注

诊断摘要复用 `life_events`，只记录事件类型、动作类型和策略原因，不上传。
