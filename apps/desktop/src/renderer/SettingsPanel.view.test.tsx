import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import {
  formatSurfaceScalePercent,
  SettingsPanel,
  stepSurfaceScale,
} from "./SettingsPanel";

describe("SettingsPanel information architecture", () => {
  it("renders settings in user-facing groups without unexplained technical labels", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    for (const label of ["性格", "生活", "打扰", "隐私", "高级"]) {
      expect(html).toContain(`>${label}<`);
    }
    expect(html).toContain("默认不读取代码、窗口标题、终端全文或按键内容。");
    expect(html).toContain("会读取：分支名、是否有未提交改动、短 commit hash。");
    expect(html).toContain("不会读取：不读取代码正文、diff、commit message 全文或远程凭据。");
    expect(html).toContain("可随时关闭");
    expect(html).toContain("高能");
    expect(html).toContain("项目状态");
    expect(html).not.toContain("Git 元数据");
    expect(html).not.toContain("BYOM");
    expect(html).not.toContain("adapter");
    expect(html).not.toContain("protocol");
    expect(html).not.toContain("runtime");
  });

  it("keeps developer rhythm behind a default-closed optional guide", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('<details class="ian-settings-developer">');
    expect(html).not.toContain('<details class="ian-settings-developer" open="">');
    expect(html).toContain("可选开发节奏");
  });

  it("renders a bidirectional pet size controller", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('class="ian-size-control"');
    expect(html).toContain('aria-label="缩小 Ian"');
    expect(html).toContain('aria-label="Ian 大小"');
    expect(html).toContain('aria-label="放大 Ian"');
    expect(html).toContain('class="ian-size-value"');
    expect(html).toContain(">100%</span>");
  });

  it("renders a pet resource pack selector with built-in pets", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('aria-label="选择宠物"');
    expect(html).toContain("小冒险家");
    expect(html).toContain("小狗");
    expect(html).toContain("小猫");
    expect(html).toContain("羊驼");
  });

  it("renders accessible resource pack preview cards that reuse pet switching", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('class="ian-pet-preview-grid"');
    expect(html).toContain('aria-label="切换到小冒险家"');
    expect(html).toContain('aria-pressed="true"');
    expect(html).toContain("像素");
    expect(html).toContain("圆润");
  });

  it("renders visual reset controls without touching privacy or reminder settings", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('aria-label="恢复默认外观"');
    expect(html).toContain('aria-label="重置桌面位置"');
  });

  it("renders Demo and BYOM mode without allowing remote mode before a key exists", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain("对话模式");
    expect(html).toContain("Demo");
    expect(html).toContain("自带模型");
    expect(html).toContain("先保存本地 key 后才能启用");
    expect(html).toMatch(/aria-label="启用自带模型"[^>]*disabled=""/);
  });

  it("renders reminder cadence, do-not-disturb, permission center, and privacy onboarding copy", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain('aria-label="提醒间隔"');
    expect(html).toContain('aria-label="启用勿扰"');
    expect(html).toContain("权限中心");
    expect(html).toContain("高敏能力默认关闭");
    expect(html).toContain("Ian 默认本地优先");
  });

  it("renders memory review and future social plugin boundaries as local default-off surfaces", () => {
    const html = renderToStaticMarkup(<SettingsPanel {...baseProps()} isOpen />);

    expect(html).toContain("记忆候选");
    expect(html).toContain("确认后才会本地保存");
    expect(html).toContain("清空候选");
    expect(html).toContain("未来社交和插件能力");
    expect(html).toContain("Feishu、Pet Visit、插件系统默认关闭");
    expect(html).toContain("默认无遥测");
  });

  it("renders find Ian controls with a default-off low-sensitive shortcut boundary", () => {
    const html = renderToStaticMarkup(
      <SettingsPanel
        {...baseProps()}
        isOpen
        findIanShortcutEnabled={false}
        findIanShortcutStatus="conflict"
      />,
    );

    expect(html).toContain("找回 Ian");
    expect(html).toContain("Cmd+Shift+I");
    expect(html).toContain("只监听一个找回 Ian 快捷键，不记录输入内容");
    expect(html).toContain("快捷键被其他应用占用");
    expect(html).toContain('aria-label="启用找回 Ian 快捷键"');
  });

  it("marks size controls disabled at supported bounds", () => {
    const minHtml = renderToStaticMarkup(
      <SettingsPanel {...baseProps()} isOpen surfaceScale={0.8} />,
    );
    const maxHtml = renderToStaticMarkup(
      <SettingsPanel {...baseProps()} isOpen surfaceScale={1.4} />,
    );

    expect(minHtml).toMatch(/aria-label="缩小 Ian"[^>]*disabled=""/);
    expect(maxHtml).toMatch(/aria-label="放大 Ian"[^>]*disabled=""/);
  });

  it("steps and clamps surface scale without floating point drift", () => {
    expect(stepSurfaceScale(1, 1)).toBe(1.1);
    expect(stepSurfaceScale(1, -1)).toBe(0.9);
    expect(stepSurfaceScale(1.4, 1)).toBe(1.4);
    expect(stepSurfaceScale(0.8, -1)).toBe(0.8);
    expect(stepSurfaceScale(1.2, 1)).toBe(1.3);
    expect(formatSurfaceScalePercent(1)).toBe("100%");
    expect(formatSurfaceScalePercent(1.4)).toBe("140%");
  });
});

function baseProps(): Parameters<typeof SettingsPanel>[0] {
  return {
    activeAppPresenceEnabled: false,
    behaviorMode: "normal",
    bubbleFrequency: "normal",
    buildTestEventsEnabled: false,
    byomEnabled: false,
    byomKeyConfigured: false,
    developerSnooze: { enabled: false, until_ms: null, reason: null },
    developerWorkspace: {
      bound: false,
      enabled: false,
      workspace_id: null,
      display_name: null,
      root_path: null,
    },
    diagnosticsEnabled: true,
    gitMetadataEnabled: false,
    isOpen: false,
    keyboardRhythmEnabled: false,
    movementIntensity: "normal",
    activePetId: "ian-adventurer",
    quietHours: {
      enabled: false,
      start_minute: 22 * 60,
      end_minute: 7 * 60,
    },
    remindersEnabled: true,
    reminderIntervalMinutes: 90,
    doNotDisturb: false,
    privacyOnboardingSeen: false,
    findIanShortcutEnabled: false,
    findIanShortcut: "Cmd+Shift+I",
    findIanShortcutStatus: "idle",
    restBehavior: "normal",
    playfulEnergy: "normal",
    playfulSnoozedUntilMs: null,
    surfaceScale: 1,
    onCapabilityEnabledChange: vi.fn(),
    onClose: vi.fn(),
    onCreatureSettingsChange: vi.fn(),
    onDeveloperSnoozeChange: vi.fn(),
    onDeveloperWorkspaceChange: vi.fn(),
    onModeChange: vi.fn(),
    onPetChange: vi.fn(),
    onQuietHoursChange: vi.fn(),
    onDoNotDisturbChange: vi.fn(),
    onPrivacyOnboardingSeenChange: vi.fn(),
    onFindIan: vi.fn(),
    onFindIanShortcutEnabledChange: vi.fn(),
    onRemindersEnabledChange: vi.fn(),
    onReminderIntervalMinutesChange: vi.fn(),
    onResetAppearance: vi.fn(),
    onResetPosition: vi.fn(),
  };
}
