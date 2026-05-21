# v0.1 Core Life Acceptance

## 自动化验收

运行：

```bash
npm run desktop:acceptance
```

该命令覆盖：

- React action execution、settings model、resource loader。
- TypeScript 编译。
- Vite production build。
- Rust behavior、scheduler、storage、security、dialogue、resource registry。

## Browser Smoke

在 `http://127.0.0.1:1420/` 验收：

- Ian 首屏可见，背景保持透明感，无明显大矩形底色。
- 点击 Ian 出现气泡，连续点击出现亲近反馈。
- 双击 Ian 进入 `run` 动画并产生短位移，随后回到 idle / anchor。
- 打开设置，行为模式、安静时段、移动、气泡、休息和大小控件可见。
- 修改安静时段后 UI 状态稳定，点击互动仍可用。

## 人工视觉验收

- Ian 的点击区域与气泡输入不互相干扰。
- 拖拽结束不会额外触发普通点击。
- 夜间 / 安静时段策略不应让 Ian 主动打扰用户。
- Ian 不展示 Mood / Bond 数值、经验条或任务式 UI。

## 明确排除

v0.1 core life 不以 Developer Rhythm 作为通过条件，不验收 Git、构建测试、全局键盘、窗口标题、屏幕 OCR、代码内容、终端输出、Feishu、Pet Visit 或插件系统。
