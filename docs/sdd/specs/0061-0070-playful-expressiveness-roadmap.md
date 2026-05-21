# 0061-0070 SDD 拆分总览: Playful Expressiveness 高能卖萌

## 状态

0061-0070 已实现，待用户验收。

## 拆分原则

0021-0040 已经完成克制版核心生命感：会动、会停、会回应、可控。用户现在明确希望 Ian 具备更强的随机感、满屏乱跑和撒娇卖萌能力。本批 SDD 追加为 Playful Expressiveness 阶段，不修改 0040 以前的验收结论，也不把 0023 的 idle micro roaming 改成失控随机移动。

本阶段把“满屏乱跑”定义为可控高能彩蛋：由用户互动、低频自发惊喜或设置开启触发；必须有边界、冷却、退出、安静模式保护和用户关闭入口。

## 顺序

| 编号 | Packet | 阶段 | 目标 |
| --- | --- | --- | --- |
| 0061 | `0061-playful-energy-mode` | Playful Expressiveness | 建立高能卖萌模式的行为边界和开关。 |
| 0062 | `0062-zoomies-path-behavior` | Playful Expressiveness | 实现可控“满屏乱跑”路径，不影响普通 idle roam。 |
| 0063 | `0063-controlled-randomness-policy` | Playful Expressiveness | 引入可测试的受控随机策略，让行为不机械。 |
| 0064 | `0064-cute-affection-reaction-pack` | Playful Expressiveness | 扩展撒娇、卖萌、贴贴短句和动作反应包。 |
| 0065 | `0065-playful-trigger-rules` | Playful Expressiveness | 定义高能彩蛋触发条件、概率、冷却和取消条件。 |
| 0066 | `0066-playful-visual-effects` | Playful Expressiveness | 增强高能跑动和撒娇的视觉表现。 |
| 0067 | `0067-playful-safety-and-user-control` | Playful Expressiveness | 增加用户控制、安静模式和防打扰安全网。 |
| 0068 | `0068-playful-state-and-cooldown-model` | Playful Expressiveness | 建立 playful state、冷却、疲劳和恢复模型。 |
| 0069 | `0069-playful-tuning-diagnostics` | Playful Expressiveness | 提供本地调参和诊断，解释为什么触发或没触发。 |
| 0070 | `0070-playful-expressiveness-acceptance-suite` | Playful Acceptance | 建立高能卖萌体验验收套件。 |

## 依赖关系

```txt
0061 -> 0062
0061 -> 0064
0061 -> 0067
0062 -> 0063
0063 -> 0065
0064 -> 0065
0065 -> 0068
0062 + 0064 -> 0066
0067 + 0068 -> 0069
0061-0069 -> 0070
```

## 防漂移规则

- 高能卖萌不是默认失控，不替代 idle micro roaming。
- “满屏乱跑”只能在安全边界内执行，必须可取消、可冷却、可关闭。
- 随机必须可测试、可复现，不允许不可控 flaky 行为。
- 撒娇卖萌优先用短句、动作、表情和节奏表达，不做数值化养成面板。
- 安静模式、拖拽、输入、睡眠、权限设置和用户控制优先级高于高能彩蛋。
- React 只执行 action，随机、触发和行为决策仍由 Rust Core 负责。
