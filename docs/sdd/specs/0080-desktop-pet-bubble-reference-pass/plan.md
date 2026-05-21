# 0080 Desktop Pet Bubble Reference Pass Plan

## 实现步骤

1. 在 `Bubble.view.test.tsx` 增加 RED 测试，要求默认气泡输出桌面宠物状态语义和低干扰回复标记。
   - 对应验收标准：1、2。
2. 在 `Bubble.view.test.tsx` 增加 CSS 回归断言，要求紧凑头顶锚点、淡边框、自适应尺寸继续存在。
   - 对应验收标准：2、3、4。
3. 修改 `Bubble.tsx`，输出 `data-bubble-role="pet-status"` 和更具体的云朵结构标记。
   - 对应验收标准：1、2。
4. 修改 `ianStage.css`，将气泡收敛为桌面宠物状态泡泡：更靠近头顶、低干扰回复按钮、更自然的小泡尾部。
   - 对应验收标准：2、3、4。
5. 运行验证，保存浏览器截图，并更新 `verification.md`。
   - 对应验收标准：1、2、3、4、5。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/spec.md`
- `docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/plan.md`
- `docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/decisions.md`
- `docs/sdd/specs/0080-desktop-pet-bubble-reference-pass/verification.md`

## 接口 / 模块影响

- 仅增加 DOM data attributes，不改变 TypeScript public props。
- 不改变 Rust action/event/state 协议。
- 不改变持久化格式。

## 兼容性说明

现有 `Bubble` props 和状态模型保持兼容。测试仍使用现有 Vitest + SSR 渲染检查。

## 风险和回滚

- 风险：气泡进一步压缩后，长文本更依赖两行截断。
- 回滚：恢复 `Bubble.tsx` data attribute 与 `ianStage.css` 气泡段落到 0079 状态。

## 执行结果

- 已增加 `data-bubble-role="pet-status"` 和 `data-bubble-shape="thought-cloud"`，把默认气泡标记为桌面宠物状态泡泡。
- 已将气泡锚点调整为 `--ian-pet-bubble-anchor-bottom: 152px`，让气泡更贴近 Ian 头顶。
- 已将回复按钮标记为 `data-control-treatment="low-interruption"`，默认透明度降到 `0.52`。
- 已保留淡描边、自适应尺寸和尾部小泡泡。
