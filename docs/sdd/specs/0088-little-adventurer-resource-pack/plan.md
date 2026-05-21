# 0088 实施计划

## 实现步骤

1. 建立小冒险家资源包目录和 manifest。
   - 对应验收：资源包文件存在、字段校验通过、capabilities 完整。
2. 设计 `animations.json`。
   - 第一批映射现有语义动画：`idle`、`walk`、`run`、`zoomies`、`happy`、`rest`、`sleep`。
   - 额外保留扩展动画 key，作为后续行为映射储备。
   - 对应验收：动画定义非空、fps / loop 合法、fallback 稳定。
3. 绘制 `sprite.svg`。
   - 使用原创角色：像素风小冒险家、小背包、软围巾、发光探路棒。
   - 至少 32 帧，优先覆盖第一批语义动画。
   - 对应验收：动作可辨、锚点稳定、像素网格清晰、无大面积黑色剪影、无攻击性武器姿态。
4. 添加或扩展资源包加载测试。
   - 确认 `ian-adventurer` 可以加载。
   - 确认未知动画 fallback 到 `idle`。
5. 将当前前端默认资源包切到 `ian-adventurer`，用于真实桌面验收；配置层默认迁移另行决策。
6. 运行前端验证、类型检查和桌面端验收。
7. 更新 `verification.md`，记录测试、截图和剩余风险。

## 预计改动文件

- `apps/desktop/public/resources/pets/ian-adventurer/pet.json`
- `apps/desktop/public/resources/pets/ian-adventurer/animations.json`
- `apps/desktop/public/resources/pets/ian-adventurer/expressions.json`
- `apps/desktop/public/resources/pets/ian-adventurer/sprite.svg`
- `apps/desktop/src/resources/resourceLoader.test.ts`
- `docs/sdd/specs/0088-little-adventurer-resource-pack/spec.md`
- `docs/sdd/specs/0088-little-adventurer-resource-pack/plan.md`
- `docs/sdd/specs/0088-little-adventurer-resource-pack/decisions.md`
- `docs/sdd/specs/0088-little-adventurer-resource-pack/verification.md`

## 接口和模块影响

- 不新增 Rust 协议。
- 不新增持久化字段。
- 不新增 Tauri API。
- React resource loader 继续使用现有 contract。
- 资源包可以包含扩展动画 key，但行为脑不会在本轮主动请求这些 key。

## 兼容性说明

- 现有 `ian-puppy`、`ian-kitten`、`ian-alpaca` 不受影响。
- 当前前端默认加载 `ian-adventurer`，用于让用户直接看到小冒险家版本。
- 配置层默认值和旧配置迁移仍保持现状；如果后续要彻底把小冒险家设为产品默认，需要单独 SDD 验证迁移。

## 验证命令和桌面验收

- `npm run desktop:test -- resourceLoader.test.ts`
- `npm run desktop:test -- AnimationPlayer.test.ts`
- `npm run desktop:typecheck`
- `npm run desktop:test`
- `git diff --check`
- 启动或复用真实 Tauri 桌面壳：`npm run desktop:tauri -- dev --host 127.0.0.1`
- 桌面验收：
  - 加载 `ian-adventurer`。
  - 截图记录 idle。
  - 点击 Ian，记录 happy。
  - 触发追鼠标，记录 run / chase。
  - 等待或触发 rest，记录 rest。
  - 检查窗口位置、透明边界、点击区域和桌面日志。

## 风险和回滚

- 风险：一次性帧数过多导致 SVG 体积变大。
  - 应对：保持单帧结构复用，避免复杂滤镜和过多路径。
- 风险：扩展动画 key 很多，但行为暂时用不上。
  - 应对：第一批只把 7 个语义动画接入现有行为，扩展动作作为后续接入储备。
- 风险：视觉太像战斗角色、参考图复刻或黑色糊成一团。
  - 应对：主体做原创轻量像素风小冒险家，探路棒表现为发光工具，不画刀刃、攻击轨迹或伤害反馈。
- 回滚：保留资源包但不切换默认；如资源加载有问题，删除 `ian-adventurer` 引用即可恢复现有默认资源包。
