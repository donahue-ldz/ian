import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { createInitialIanViewState } from "../state/ianActions";
import { IanStage } from "./IanStage";

describe("IanStage desktop chrome", () => {
  it("keeps the default settings entry low-interruption but accessible", () => {
    const html = renderToStaticMarkup(
      <IanStage
        {...baseProps()}
        isSettingsOpen={false}
      />,
    );

    expect(html).toContain('aria-label="打开 Ian 设置"');
    expect(html).toContain('title="打开 Ian 设置"');
    expect(html).not.toContain(">设置</button>");
    expect(html).not.toContain('class="ian-settings"');
  });

  it("keeps settings reachable when the entry is activated", () => {
    const html = renderToStaticMarkup(
      <IanStage
        {...baseProps()}
        isSettingsOpen={true}
      />,
    );

    expect(html).toContain('aria-label="Ian 设置"');
    expect(html).toContain("行为");
    expect(html).toContain("开发者节奏");
  });

  it("marks quiet mode so visual motion can be reduced", () => {
    const html = renderToStaticMarkup(
      <IanStage
        {...baseProps()}
        behaviorMode="quiet"
      />,
    );

    expect(html).toContain('data-behavior-mode="quiet"');
  });

  it("marks the resting drag state for carry-specific styling", () => {
    const html = renderToStaticMarkup(<IanStage {...baseProps()} />);

    expect(html).toContain('data-dragging="false"');
    expect(html).toContain('data-drag-phase="resting"');
  });

  it("marks movement profiles so CSS can express body feel", () => {
    const html = renderToStaticMarkup(
      <IanStage
        {...baseProps()}
        viewState={{
          ...createInitialIanViewState(),
          movementTarget: {
            x: 24,
            y: 36,
            speed: "normal",
            profile: "playful",
          },
        }}
      />,
    );

    expect(html).toContain('data-motion-profile="playful"');
  });

  it("marks desktop and browser preview window contexts separately", () => {
    const desktopHtml = renderToStaticMarkup(
      <IanStage {...baseProps()} isDesktopWindow={true} />,
    );
    const browserHtml = renderToStaticMarkup(
      <IanStage {...baseProps()} isDesktopWindow={false} />,
    );

    expect(desktopHtml).toContain('data-window-context="desktop"');
    expect(browserHtml).toContain('data-window-context="preview"');
  });
});

describe("IanStage idle visual comfort", () => {
  it("defines rest, sleep, quiet, and reduced-motion visual rules", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain('.ian-sprite[data-animation="rest"]');
    expect(ianStageCss).toContain(
      '.ian-sprite[data-animation="sleep"] .ian-ear-left',
    );
    expect(ianStageCss).toContain(
      '.ian-creature-surface[data-behavior-mode="quiet"]',
    );
    expect(ianStageCss).toContain("@media (prefers-reduced-motion: reduce)");
  });

  it("defines carry styling for active desktop dragging", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain(
      '.ian-creature-surface[data-dragging="true"] .ian-sprite',
    );
    expect(ianStageCss).toContain('[data-drag-phase="pickup"]');
    expect(ianStageCss).toContain('[data-drag-phase="carried"]');
    expect(ianStageCss).toContain('[data-drag-phase="dropping"]');
    expect(ianStageCss).toContain("@keyframes ian-carry-wiggle");
    expect(ianStageCss).toContain("@keyframes ian-pickup-flinch");
    expect(ianStageCss).toContain("@keyframes ian-drop-squash");
  });

  it("defines the tail wag micro-life visual effect", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain(
      '.ian-visual-effect[data-effect="tail_wag"]',
    );
    expect(ianStageCss).toContain("@keyframes ian-tail-wag");
  });

  it("caps micro-effect animation timing and disables dynamic effects in reduced motion", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain("--ian-effect-duration-cap: 1600ms");
    expect(ianStageCss).toMatch(/animation-duration: min\([^)]*var\(--ian-effect-duration-cap\)/);
    expect(ianStageCss).toMatch(
      /@media \(prefers-reduced-motion: reduce\)[\s\S]*\.ian-visual-effect\s*{[\s\S]*display: none;/,
    );
  });

  it("keeps high-energy animation loops readable", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toMatch(
      /\.ian-sprite\[data-animation="run"\]\s*{[^}]*animation: ian-run 460ms/s,
    );
    expect(ianStageCss).toMatch(
      /\.ian-sprite\[data-animation="zoomies"\]\s*{[^}]*animation: ian-zoomies 360ms/s,
    );
  });

  it("defines motion profile styling for gentle, playful, and settle movement", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain('[data-motion-profile="gentle"]');
    expect(ianStageCss).toContain('[data-motion-profile="playful"]');
    expect(ianStageCss).toContain('[data-motion-profile="settle"]');
    expect(ianStageCss).toContain("@keyframes ian-playful-move");
    expect(ianStageCss).toContain("@keyframes ian-settle-move");
  });

  it("reserves a desktop safe area for the full cloud bubble", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );
    const tauriConfig = JSON.parse(
      readFileSync(
        new URL("../../src-tauri/tauri.conf.json", import.meta.url),
        "utf8",
      ),
    );

    expect(ianStageCss).toContain(
      '.ian-stage[data-window-context="desktop"]',
    );
    expect(ianStageCss).toContain("align-items: end;");
    expect(tauriConfig.app.windows[0].height).toBeGreaterThanOrEqual(320);
  });

  it("opens desktop settings inside the transparent window safe area", () => {
    const ianStageCss = readFileSync(
      new URL("./ianStage.css", import.meta.url),
      "utf8",
    );

    expect(ianStageCss).toContain(
      '.ian-stage[data-window-context="desktop"] .ian-settings',
    );
    expect(ianStageCss).toMatch(
      /\.ian-stage\[data-window-context="desktop"\] \.ian-settings\s*{[^}]*left: auto;[^}]*right: 44px;[^}]*box-sizing: border-box;/s,
    );
  });

  it("allows the desktop pet surface to request native window dragging", () => {
    const capability = JSON.parse(
      readFileSync(
        new URL("../../src-tauri/capabilities/default.json", import.meta.url),
        "utf8",
      ),
    );

    expect(capability.permissions).toContain(
      "core:window:allow-start-dragging",
    );
  });

  it("allows desktop movement actions to move the native window", () => {
    const capability = JSON.parse(
      readFileSync(
        new URL("../../src-tauri/capabilities/default.json", import.meta.url),
        "utf8",
      ),
    );

    expect(capability.permissions).toContain("core:window:allow-set-position");
  });
});

function baseProps(): Parameters<typeof IanStage>[0] {
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
    diagnosticsEnabled: false,
    doNotDisturb: false,
    gitMetadataEnabled: false,
    activePetId: "ian-adventurer",
    memoryCandidates: [],
    memoryExportRecords: [],
    isMemoryLoading: false,
    memoryError: null,
    isSettingsOpen: false,
    isDesktopWindow: false,
    keyboardRhythmEnabled: false,
    movementIntensity: "normal",
    playfulEnergy: "normal",
    playfulSnoozedUntilMs: null,
    quietHours: {
      enabled: false,
      start_minute: 22 * 60,
      end_minute: 7 * 60,
    },
    remindersEnabled: true,
    reminderIntervalMinutes: 90,
    privacyOnboardingSeen: false,
    findIanShortcutEnabled: false,
    findIanShortcut: "Cmd+Shift+I",
    findIanShortcutStatus: "idle",
    resourcePack: null,
    restBehavior: "normal",
    surfaceScale: 1,
    viewState: createInitialIanViewState(),
    onBehaviorModeChange: vi.fn(),
    onBubbleInputEnded: vi.fn(),
    onBubbleInputStarted: vi.fn(),
    onCapabilityEnabledChange: vi.fn(),
    onCreatureSettingsChange: vi.fn(),
    onDeveloperSnoozeChange: vi.fn(),
    onDeveloperWorkspaceChange: vi.fn(),
    onDoNotDisturbChange: vi.fn(),
    onDragEnd: vi.fn(),
    onDragStart: vi.fn(),
    onIanClick: vi.fn(),
    onIanDoubleClick: vi.fn(),
    onIanLeave: vi.fn(),
    onIanNear: vi.fn(),
    onPetChange: vi.fn(),
    onQuietHoursChange: vi.fn(),
    onPrivacyOnboardingSeenChange: vi.fn(),
    onFindIan: vi.fn(),
    onFindIanShortcutEnabledChange: vi.fn(),
    onConfirmMemoryCandidate: vi.fn(),
    onDeleteMemoryCandidate: vi.fn(),
    onClearMemoryCandidates: vi.fn(),
    onRefreshMemoryExport: vi.fn(),
    onClearInteractionJournal: vi.fn(),
    onResetLocalSettings: vi.fn(),
    onMomentDebugTrigger: vi.fn(),
    onReminderIntervalMinutesChange: vi.fn(),
    onRemindersEnabledChange: vi.fn(),
    onResetAppearance: vi.fn(),
    onResetPosition: vi.fn(),
    onSettingsClose: vi.fn(),
    onSettingsToggle: vi.fn(),
    onSubmitMessage: vi.fn(),
  };
}
