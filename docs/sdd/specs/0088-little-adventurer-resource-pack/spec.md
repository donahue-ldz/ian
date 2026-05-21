# 0088 小冒险家 Ian 资源包

## 问题 / 目标

当前 Ian 已经具备桌面移动、追鼠标、拖动和基础表情能力，但角色视觉还偏“单一宠物形象”。本 SDD 目标是设计并实现一个原创“小冒险家 Ian”资源包，让 Ian 在桌面上更像会探索、会观察、会跑来跑去的小伙伴。

本轮重点不是复刻参考图，而是提取“轻快行走、发光探路道具、冒险姿态”的气质，做成可长期扩展的原创 resource pack。

## 当前产品阶段

P0 / MVP + Product Feel 体验增强。

小冒险家属于视觉资源包和动画表现增强，不改变 Ian 的产品身份：Ian 仍是本地优先的桌面数字生命，不是游戏角色、战斗角色或助手皮肤。

## 产品范围

- 新增 `ian-adventurer` 资源包。
- 角色设定：圆润小动物体型、短腿、小背包、软围巾、发光探路棒。
- 视觉气质：好奇、轻快、会探索、会撒娇，不做攻击性武器姿态。
- 第一批实现必须覆盖现有语义动画：
  - `idle`
  - `walk`
  - `run`
  - `zoomies`
  - `happy`
  - `rest`
  - `sleep`
- 资源包内设计大量动画目录，分为“立即可用”和“后续可接入”两类。
- 第一批实现的动画必须能被现有 `AnimationPlayer` 播放，不要求本轮新增行为协议。
- 资源包必须保留稳定锚点、稳定透明边界和稳定缩放，避免桌面移动时抖动。

## 动画目录设计

### A. 当前语义动画，第一批必须实现

1. `idle_breathe`：站立呼吸，围巾轻摆。
2. `idle_blink`：眨眼，探路棒微微发光。
3. `idle_look_left`：向左探头。
4. `idle_look_right`：向右探头。
5. `idle_map_check`：低头看小地图。
6. `walk_light`：普通走路，背包轻晃。
7. `walk_scout`：探路式走路，探路棒向前。
8. `run_dash`：短腿快跑，围巾后飘。
9. `run_chase`：追鼠标时前倾奔跑。
10. `zoomies_loop`：兴奋乱跑，身体压低。
11. `happy_wave`：开心挥探路棒。
12. `happy_hop`：小跳一下。
13. `rest_sit`：坐下休息。
14. `rest_bag_adjust`：整理背包。
15. `sleep_curl`：蜷起来睡。
16. `sleep_dream`：睡梦中小光点闪烁。

这些内部动作会映射到现有语义动画：

- `idle` 可以由 `idle_breathe` / `idle_blink` / `idle_look_left` / `idle_look_right` 的帧组成。
- `walk` 可以由 `walk_light` / `walk_scout` 的帧组成。
- `run` 可以由 `run_dash` / `run_chase` 的帧组成。
- `zoomies` 使用 `zoomies_loop`。
- `happy` 使用 `happy_wave` / `happy_hop`。
- `rest` 使用 `rest_sit` / `rest_bag_adjust`。
- `sleep` 使用 `sleep_curl` / `sleep_dream`。

### B. 生命感增强动画，后续优先接入

17. `curious_tilt`：歪头观察。
18. `sniff_ground`：低头闻一闻。
19. `listen_alert`：耳朵竖起听声音。
20. `surprise_pop`：被点到后轻微惊讶。
21. `proud_pose`：完成小动作后得意站姿。
22. `shy_hide`：半躲到围巾后面。
23. `attention_peek`：从屏幕边缘探头。
24. `attention_call`：轻轻招手。
25. `mouse_chase_start`：发现鼠标，身体压低。
26. `mouse_chase_turn`：追逐中转向。
27. `mouse_chase_stop`：追到附近后刹住。
28. `settle_puff`：跑完轻轻喘一下。
29. `drag_pickup`：被拖起时缩手缩脚。
30. `drag_carry`：被拎着时晃动。
31. `drag_release`：落地恢复平衡。
32. `tiny_patrol_start`：开始桌面巡逻。
33. `tiny_patrol_corner`：到角落看一眼。
34. `tiny_patrol_return`：巡逻回来。

### C. 撒娇 / 淘气动画，后续可选接入

35. `pout_stomp`：小跺脚。
36. `tantrum_flop`：轻轻趴倒撒赖。
37. `roll_peek`：翻一下又偷看用户。
38. `begging_look`：抬头看用户。
39. `hug_staff`：抱住探路棒。
40. `scarf_wrap`：把围巾裹紧。
41. `bag_rummage`：翻小背包。
42. `treasure_found`：找到小亮点。
43. `sparkle_discover`：发现什么，眼睛亮一下。
44. `confused_spin`：迷路转一圈。
45. `stumble_recover`：小绊一下又站稳。
46. `jump_over`：小跳跨过想象障碍。
47. `climb_edge`：爬桌面边缘的错觉动作。
48. `slide_stop`：滑步刹车。
49. `wave_goodnight`：睡前挥手。
50. `morning_stretch`：醒来伸懒腰。
51. `rainy_idle`：低能量时抱着小背包。
52. `focus_quiet`：安静模式下坐好。

## 明确不做什么

- 不复刻参考图中的具体角色、轮廓、服装和武器设定。
- 不做攻击动作、砍击动作、战斗待机或伤害反馈。
- 不新增全局鼠标监听。
- 不读取屏幕内容、窗口内容或用户文件。
- 不把所有 52 个动作一次性接入 Rust 行为策略。
- 不引入远程素材下载、皮肤市场或运行时脚本。

## 用户体验

用户看到的 Ian 会从普通小宠物变成“小冒险家”：

- 平时会呼吸、眨眼、探头、看地图。
- 走路和巡逻时背包、围巾和探路棒会一起动。
- 追鼠标时会像发现目标后追上去，而不是简单平移。
- 被点击或拖动时会有轻微惊讶、开心、被抱起的反馈。
- 安静模式下仍然收敛，不会过度跑动或频繁打扰。

## 架构约束

- 资源包仍遵循现有 Resource Pack contract。
- Rust Core 继续输出语义动作；React 只负责播放资源包动画。
- 第一批不新增 `IanEvent` / `IanAction`。
- 额外动画先作为 resource pack 内部可用素材，不强制立即暴露为协议。
- 若后续需要更细动画选择，必须另开 SDD，评估是否新增动画 alias、情绪到动画映射或行为策略。

## 数据 / 协议变化

第一批不新增协议类型。

资源包新增：

```txt
apps/desktop/public/resources/pets/ian-adventurer/
  pet.json
  animations.json
  expressions.json
  sprite.svg
```

`animations.json` 可以包含现有语义动画之外的扩展动画 key，但当前前端只会在行为输出请求对应 key 时播放；未知 key 继续 fallback 到 `idle`。

## 隐私与安全边界

- 所有素材为本地静态资源。
- 不联网下载资源。
- 不执行资源包脚本。
- 不读取用户屏幕、文件、剪贴板或应用内容。

## 验收标准

- [ ] 新增 `ian-adventurer` resource pack，包含 `pet.json`、`animations.json`、`expressions.json`、`sprite.svg`。
- [ ] `pet.json` 通过现有资源包字段校验，且 `capabilities` 至少包含 `idle`、`walk`、`run`、`zoomies`、`happy`、`rest`、`sleep`。
- [ ] `animations.json` 至少定义 7 个现有语义动画，且每个动画都有非空 `frames`、正数 `fps` 和明确 `loop`。
- [ ] `sprite.svg` 至少包含 32 个可播放帧，覆盖站立、走路、跑步、开心、休息、睡觉和追逐姿态。
- [ ] 角色视觉包含小背包、软围巾和发光探路棒，但不呈现攻击性武器姿态。
- [ ] 主要动画切换时角色底部锚点稳定，没有明显跳高、裁切或缩放突变。
- [ ] 新资源包可以被前端资源加载测试读取，并能 fallback 到 `idle`。
- [ ] 真实 Tauri 桌面端可切换或临时加载小冒险家资源包，并完成 idle、run / chase、happy、rest 的桌面截图验收。

## 验证方式

- 资源包 manifest 单元测试。
- `npm run desktop:test -- resourceLoader.test.ts`
- `npm run desktop:test -- AnimationPlayer.test.ts`
- `npm run desktop:typecheck`
- `npm run desktop:test`
- 启动真实 Tauri 桌面壳，加载 `ian-adventurer`，在桌面端观察：
  - 默认 idle 是否显示。
  - 点击 / 追鼠标时是否能看到跑动姿态。
  - 点击反馈是否能看到 happy 姿态。
  - idle / run / happy / rest 切换是否无明显裁切或锚点跳变。
  - 桌面端运行日志是否无明显资源加载错误。
