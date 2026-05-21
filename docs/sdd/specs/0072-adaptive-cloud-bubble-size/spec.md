# Spec: Adaptive Cloud Bubble Size

## 状态

已实现，待用户验收。

## 问题 / 目标

0071 的云朵气泡外观成立，但默认宽度偏大。用户希望气泡按文字大小自适应，短句不要占用过大的云朵面积。0072 目标是让 Ian 的云朵气泡根据文本长度使用紧凑、常规、长句三档尺寸，并在输入框打开时保持可用宽度。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 为 bubble 添加本地尺寸分档：`short`、`medium`、`long`。
- 短句使用更小的云朵宽度和内边距。
- 长句仍保留最大宽度、两行截断和可读性。
- 输入框打开时使用足够宽的气泡，避免输入控件过窄。

## 明确不做什么

- 不改变 bubble 文案、触发、持续时间和输入提交逻辑。
- 不改变 Rust Core、协议和持久化。
- 不做可配置主题系统。

## 用户体验

短句气泡更贴近文字，不再默认大块占屏；较长句子自动变宽，仍然保持云朵想法气泡的可爱外观。

## 架构约束

- 尺寸判断在 React 视图层基于已格式化短文本完成。
- React 仍不决定 Ian 行为，只决定可视布局。
- CSS 继续使用本地样式，不新增图片资源。

## 数据 / 协议变化

无协议和持久化变化。

## 隐私与安全边界

只基于当前 bubble 文本长度做布局，不读取任何新数据。

## 验收标准

- [x] 短句 bubble 渲染 `data-cloud-size="short"`。
- [x] 较长文本 bubble 渲染 `data-cloud-size="long"`。
- [x] CSS 中短句气泡宽度小于 0071 的固定 178px 默认宽度。
- [x] 输入框打开时可以使用较宽尺寸，不挤压输入控件。
- [x] Browser smoke 截图记录短句紧凑气泡。

## 验证方式

- `npm run desktop:test -- Bubble.view.test.tsx`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- Browser smoke 截图。
