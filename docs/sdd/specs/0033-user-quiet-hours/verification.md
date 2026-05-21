# 验证记录: User Quiet Hours

## 状态

已实现，待用户验收。

## 自动验证

- [x] Config tests - quiet hours 持久化、跨午夜、禁用状态通过。
- [x] Behavior tests - quiet hours 覆盖 lively 自主移动通过。
- [x] Frontend settings model tests - 时间格式与选项通过。
- [x] Browser smoke - 设置面安静时段开关、开始/结束时间控件可见且可操作。
- [x] `npm run desktop:acceptance` - passed。
