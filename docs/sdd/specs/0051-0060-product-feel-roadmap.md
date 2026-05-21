# 0051-0060 SDD 拆分总览: Product Feel 体验优化

## 状态

草稿，待确认。

## 拆分原则

0050 之前已经覆盖核心生命感、开发者节奏和阶段性验收。接下来不继续堆能力，而是补一轮 Product Feel：让 Ian 在桌面上更像一个有生命的陪伴对象，而不是带设置面板的工具。

本批 SDD 只追加在 0051 之后，不修改 0050 以前已经验收的范围。实现时可以复用已有能力，但验收只看本批定义的体验目标。

## 顺序

| 编号 | Packet | 阶段 | 目标 |
| --- | --- | --- | --- |
| 0051 | `0051-desktop-presence-chrome-reduction` | Product Feel | 降低工具感，让默认桌面存在只突出 Ian 本体和轻量气泡。 |
| 0052 | `0052-bubble-interaction-polish` | Product Feel | 让气泡更短、更克制、更像陪伴反馈，不像聊天窗口。 |
| 0053 | `0053-touch-and-affection-language` | Product Feel | 统一单击、连点、拖拽、双击的触摸语言和冷却规则。 |
| 0054 | `0054-movement-personality-tuning` | Product Feel | 调整移动频率、曲线和个性差异，减少随机游走的突兀感。 |
| 0055 | `0055-settings-information-architecture` | Product Feel | 按用户心智重组设置，不把技术开关直接暴露成主体验。 |
| 0056 | `0056-privacy-permission-copywriting` | Product Feel | 把权限和隐私说明写得清楚、中文优先、可验证。 |
| 0057 | `0057-developer-rhythm-progressive-disclosure` | Product Feel | Developer Rhythm 渐进暴露，避免首次体验被开发工具感覆盖。 |
| 0058 | `0058-idle-rest-visual-comfort` | Product Feel | 优化待机、休息、睡眠状态，让少动也有生命感。 |
| 0059 | `0059-resource-pack-visual-quality-pass` | Product Feel | 对默认资源包做视觉质量验收，保证动作和表情一致。 |
| 0060 | `0060-product-feel-acceptance-suite` | Product Feel Acceptance | 建立产品体验验收套件，覆盖第一眼、短时陪伴和权限设置。 |

## 依赖关系

```txt
0051 -> 0052
0051 -> 0055
0052 -> 0053
0053 -> 0054
0054 -> 0058
0055 -> 0056
0056 -> 0057
0058 -> 0059
0051-0059 -> 0060
```

## 防漂移规则

- 体验优化不是新增大功能，不借机做完整聊天、Agent、IDE 助手或通知中心。
- 默认桌面第一眼必须是 Ian 的生命存在，而不是设置、权限、项目或命令入口。
- 中文优先，文案短句优先，避免把技术名词放到主路径。
- Developer Rhythm 是后置增强，不抢占核心生命感的首屏。
- 任何主动反应都必须低频、有冷却、可暂停。
- 隐私说明必须说清楚“会读取什么”和“不会读取什么”，不能只写抽象承诺。
