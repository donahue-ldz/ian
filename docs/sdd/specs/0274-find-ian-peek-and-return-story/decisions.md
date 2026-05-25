# 0274 · 决策记录

## 2026-05-22

- 找回优先保证可见和可控，探头 story 只做短增强。
- 不新增新的系统监听能力。

## 2026-05-25

- 保留现有找回入口，不新增快捷键监听能力；只改 Rust Core 返回的 find story。
- 非 reduced motion 下使用 peek target -> final target 两段移动；reduced motion 下使用单段 gentle return，降低动效但仍保证找回。
- 找回 story 只使用屏幕几何和 Ian 自身状态，不读取屏幕内容。
