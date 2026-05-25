# 0273 · 决策记录

## 2026-05-22

- 追鼠标要像互动玩耍，不做持续追踪功能。
- 正式模式必须保守，诊断模式负责可复现验收。

## 2026-05-25

- 0273 不重做已有 pointer chase 基础能力，只补 V2 缺口：注意 beat、短追两步、收尾气泡、idle settle。
- Life Drive 只影响 chase 步幅强度，不增加鼠标轨迹记录或长期行为画像。
- reduced motion / quiet / DND / active interaction 继续通过 Rust Core gate 抑制，不在 React 里自行判断追逐策略。
