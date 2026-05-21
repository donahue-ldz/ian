# Spec: Playful Visual Effects

## 状态

已实现，待用户验收。

## 问题 / 目标

高能跑动和撒娇只靠位置变化不够可爱。0066 目标是在不喧宾夺主的前提下增强视觉表现，例如速度感、表情变化、小心心或动作残影等。

## 产品范围

- 为 zoomies 增加 run / excited / dizzy 等可区分视觉状态。
- 为撒娇反应增加贴贴、害羞、开心、被摸晕的小动作表现。
- 视觉效果必须可关闭或受 reduced motion 影响。
- 缺失资源时可 fallback 到已有 run / happy / idle。

## 明确不做什么

- 不做大面积粒子轰炸。
- 不加入音频作为必需项。
- 不用装饰遮挡用户内容。
- 不为了效果让 React 接管行为决策。

## 用户体验

Ian 高能跑动时更有速度和兴奋感；撒娇时更有表情和姿态，不只是弹一句话。

## 架构约束

- Rust Core 输出动作和状态，React 执行视觉效果。
- 视觉资源遵循 Resource Pack contract。
- reduced motion / quiet mode 必须影响效果强度。

## 数据 / 协议变化

可新增 animation name 或 visual effect action，例如 `effect.play`。新增类型必须从 Rust 生成 TypeScript。

## 隐私与安全边界

视觉效果只使用本地资源，不访问网络，不加载远程脚本。

## 验收标准

- [x] zoomies 和普通 run 视觉上可区分。
- [x] 至少 3 类撒娇视觉反馈可用。
- [x] reduced motion 下效果明显降低。
- [x] 缺失资源 fallback 不白屏、不崩溃。
- [x] 视觉效果不遮挡气泡和设置入口。

## 验证方式

- 前端资源和动画测试。
- Browser / Tauri 截图或录屏。
- reduced motion smoke。
