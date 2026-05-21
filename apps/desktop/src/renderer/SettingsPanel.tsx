import type {
  BehaviorMode,
  DeveloperSnooze,
  DeveloperWorkspace,
  PlayfulEnergy,
  QuietHours,
} from "../protocol/generated";
import {
  behaviorModeOptions,
  bubbleFrequencyOptions,
  developerCapabilityOptions,
  developerSnoozeOptions,
  fromTimeInputValue,
  movementIntensityOptions,
  playfulEnergyOptions,
  restBehaviorOptions,
  toTimeInputValue,
} from "./settingsModel";

const SURFACE_SCALE_MIN = 0.8;
const SURFACE_SCALE_MAX = 1.4;
const SURFACE_SCALE_STEP = 0.1;

type SettingsPanelProps = {
  behaviorMode: BehaviorMode;
  remindersEnabled: boolean;
  byomEnabled: boolean;
  gitMetadataEnabled: boolean;
  buildTestEventsEnabled: boolean;
  keyboardRhythmEnabled: boolean;
  activeAppPresenceEnabled: boolean;
  quietHours: QuietHours;
  developerWorkspace: DeveloperWorkspace;
  developerSnooze: DeveloperSnooze;
  movementIntensity: string;
  bubbleFrequency: string;
  restBehavior: string;
  playfulEnergy: PlayfulEnergy;
  playfulSnoozedUntilMs: number | null;
  surfaceScale: number;
  diagnosticsEnabled: boolean;
  isOpen: boolean;
  onClose: () => void;
  onModeChange: (mode: BehaviorMode) => void;
  onRemindersEnabledChange: (enabled: boolean) => void;
  onQuietHoursChange: (quietHours: QuietHours) => void;
  onDeveloperWorkspaceChange: (workspace: DeveloperWorkspace) => void;
  onDeveloperSnoozeChange: (snooze: DeveloperSnooze) => void;
  onCreatureSettingsChange: (settings: {
    movementIntensity: string;
    bubbleFrequency: string;
    restBehavior: string;
    playfulEnergy: PlayfulEnergy;
    playfulSnoozedUntilMs: number | null;
    surfaceScale: number;
    diagnosticsEnabled: boolean;
  }) => void;
  onCapabilityEnabledChange: (capability: string, enabled: boolean) => void;
};

export function SettingsPanel({
  behaviorMode,
  remindersEnabled,
  byomEnabled,
  gitMetadataEnabled,
  buildTestEventsEnabled,
  keyboardRhythmEnabled,
  activeAppPresenceEnabled,
  quietHours,
  developerWorkspace,
  developerSnooze,
  movementIntensity,
  bubbleFrequency,
  restBehavior,
  playfulEnergy,
  playfulSnoozedUntilMs,
  surfaceScale,
  diagnosticsEnabled,
  isOpen,
  onClose,
  onModeChange,
  onRemindersEnabledChange,
  onQuietHoursChange,
  onDeveloperWorkspaceChange,
  onDeveloperSnoozeChange,
  onCreatureSettingsChange,
  onCapabilityEnabledChange,
}: SettingsPanelProps) {
  if (!isOpen) {
    return null;
  }

  return (
    <aside
      className="ian-settings"
      aria-label="Ian 设置"
      onPointerDown={(event) => event.stopPropagation()}
    >
      <div className="ian-settings-header">
        <span>设置</span>
        <button className="ian-icon-button" type="button" onClick={onClose} aria-label="关闭设置">
          x
        </button>
      </div>
      <section className="ian-settings-section" aria-labelledby="ian-settings-personality">
        <h3 id="ian-settings-personality">性格</h3>
        <div className="ian-settings-row">
          <span>陪伴方式</span>
          <div className="ian-segmented" role="group" aria-label="行为模式">
            {behaviorModeOptions.map((option) => (
              <button
                className="ian-segmented-option"
                data-active={option.value === behaviorMode}
                key={option.value}
                type="button"
                onClick={() => onModeChange(option.value)}
              >
                {option.label}
              </button>
            ))}
          </div>
        </div>
        <SettingsSelect
          label="移动"
          value={movementIntensity}
          options={movementIntensityOptions}
          onChange={(value) =>
            onCreatureSettingsChange({
              movementIntensity: value,
              bubbleFrequency,
              restBehavior,
              playfulEnergy,
              playfulSnoozedUntilMs,
              surfaceScale,
              diagnosticsEnabled,
            })
          }
        />
        <SettingsSelect
          label="气泡"
          value={bubbleFrequency}
          options={bubbleFrequencyOptions}
          onChange={(value) =>
            onCreatureSettingsChange({
              movementIntensity,
              bubbleFrequency: value,
              restBehavior,
              playfulEnergy,
              playfulSnoozedUntilMs,
              surfaceScale,
              diagnosticsEnabled,
            })
          }
        />
      </section>
      <section className="ian-settings-section" aria-labelledby="ian-settings-life">
        <h3 id="ian-settings-life">生活</h3>
        <SettingsSelect
          label="休息"
          value={restBehavior}
          options={restBehaviorOptions}
          onChange={(value) =>
            onCreatureSettingsChange({
              movementIntensity,
              bubbleFrequency,
              restBehavior: value,
              playfulEnergy,
              playfulSnoozedUntilMs,
              surfaceScale,
              diagnosticsEnabled,
            })
          }
        />
        <SettingsSelect
          label="高能"
          value={playfulEnergy}
          options={playfulEnergyOptions}
          onChange={(value) =>
            onCreatureSettingsChange({
              movementIntensity,
              bubbleFrequency,
              restBehavior,
              playfulEnergy: value as PlayfulEnergy,
              playfulSnoozedUntilMs,
              surfaceScale,
              diagnosticsEnabled,
            })
          }
        />
        <label className="ian-settings-toggle-row">
          <span>提醒</span>
          <input
            aria-label="启用提醒"
            checked={remindersEnabled}
            type="checkbox"
            onChange={(event) => onRemindersEnabledChange(event.currentTarget.checked)}
          />
        </label>
      </section>
      <section className="ian-settings-section" aria-labelledby="ian-settings-disturbance">
        <h3 id="ian-settings-disturbance">打扰</h3>
        <label className="ian-settings-toggle-row">
          <span>安静时段</span>
          <input
            aria-label="启用安静时段"
            checked={quietHours.enabled}
            type="checkbox"
            onChange={(event) =>
              onQuietHoursChange({
                ...quietHours,
                enabled: event.currentTarget.checked,
              })
            }
          />
        </label>
        <div className="ian-settings-time-row">
          <input
            aria-label="安静开始"
            type="time"
            value={toTimeInputValue(quietHours.start_minute)}
            onChange={(event) =>
              onQuietHoursChange({
                ...quietHours,
                start_minute: fromTimeInputValue(event.currentTarget.value),
              })
            }
          />
          <input
            aria-label="安静结束"
            type="time"
            value={toTimeInputValue(quietHours.end_minute)}
            onChange={(event) =>
              onQuietHoursChange({
                ...quietHours,
                end_minute: fromTimeInputValue(event.currentTarget.value),
              })
            }
          />
        </div>
      </section>
      <section className="ian-settings-section" aria-labelledby="ian-settings-privacy">
        <h3 id="ian-settings-privacy">隐私</h3>
        <p className="ian-settings-note">默认不读取代码、窗口标题、终端全文或按键内容。</p>
        <details className="ian-settings-developer">
          <summary>可选开发节奏</summary>
          <p className="ian-settings-note">想让 Ian 理解一点开发节奏时再开启。</p>
          <label className="ian-settings-toggle-row">
            <span>
              工作区
              <small>只保存本地授权路径</small>
            </span>
            <input
              aria-label="启用工作区绑定"
              checked={developerWorkspace.bound && developerWorkspace.enabled}
              type="checkbox"
              onChange={(event) =>
                onDeveloperWorkspaceChange({
                  bound: event.currentTarget.checked,
                  enabled: event.currentTarget.checked,
                  workspace_id: event.currentTarget.checked ? "local-workspace" : null,
                  display_name: event.currentTarget.checked ? "当前项目" : null,
                  root_path: event.currentTarget.checked ? "本地授权路径" : null,
                })
              }
            />
          </label>
          <label className="ian-settings-select-row">
            <span>暂停反应</span>
            <select
              aria-label="开发者节奏暂停"
              value={snoozeValue(developerSnooze)}
              onChange={(event) => {
                const option = developerSnoozeOptions.find(
                  (candidate) => candidate.value === event.currentTarget.value,
                );
                onDeveloperSnoozeChange(
                  optionToSnooze(option?.value ?? "off", Date.now()),
                );
              }}
            >
              {developerSnoozeOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          {developerCapabilityOptions.map((option) => (
            <CapabilityToggle
              checked={capabilityChecked(option.capability, {
                gitMetadataEnabled,
                buildTestEventsEnabled,
                keyboardRhythmEnabled,
                activeAppPresenceEnabled,
              })}
              canDisable={option.canDisable}
              description={option.scope}
              doesNotRead={option.doesNotRead}
              key={option.capability}
              label={option.label}
              reads={option.reads}
              onChange={(enabled) => onCapabilityEnabledChange(option.capability, enabled)}
            />
          ))}
        </details>
      </section>
      <details className="ian-settings-section ian-settings-advanced">
        <summary>高级</summary>
        <label className="ian-settings-toggle-row">
          <span>自带模型</span>
          <input
            aria-label="启用自带模型"
            checked={byomEnabled}
            type="checkbox"
            onChange={(event) => onCapabilityEnabledChange("byom", event.currentTarget.checked)}
          />
        </label>
        <SizeController
          value={surfaceScale}
          onChange={(value) =>
            onCreatureSettingsChange({
              movementIntensity,
              bubbleFrequency,
              restBehavior,
              playfulEnergy,
              playfulSnoozedUntilMs,
              surfaceScale: value,
              diagnosticsEnabled,
            })
          }
        />
        <label className="ian-settings-toggle-row">
          <span>本地诊断</span>
          <input
            aria-label="启用本地诊断"
            checked={diagnosticsEnabled}
            type="checkbox"
            onChange={(event) =>
              onCreatureSettingsChange({
                movementIntensity,
                bubbleFrequency,
                restBehavior,
                playfulEnergy,
                playfulSnoozedUntilMs,
                surfaceScale,
                diagnosticsEnabled: event.currentTarget.checked,
              })
            }
          />
        </label>
      </details>
    </aside>
  );
}

export function stepSurfaceScale(value: number, direction: -1 | 1): number {
  return normalizeSurfaceScale(value + direction * SURFACE_SCALE_STEP);
}

export function formatSurfaceScalePercent(value: number): string {
  return `${Math.round(normalizeSurfaceScale(value) * 100)}%`;
}

function normalizeSurfaceScale(value: number): number {
  const clamped = Math.min(Math.max(value, SURFACE_SCALE_MIN), SURFACE_SCALE_MAX);
  return Math.round(clamped * 10) / 10;
}

type SizeControllerProps = {
  value: number;
  onChange: (value: number) => void;
};

function SizeController({ value, onChange }: SizeControllerProps) {
  const normalizedValue = normalizeSurfaceScale(value);

  return (
    <div className="ian-settings-size-row">
      <span>大小</span>
      <div className="ian-size-control">
        <button
          aria-label="缩小 Ian"
          className="ian-size-button"
          disabled={normalizedValue <= SURFACE_SCALE_MIN}
          type="button"
          onClick={() => onChange(stepSurfaceScale(normalizedValue, -1))}
        >
          -
        </button>
        <input
          aria-label="Ian 大小"
          className="ian-size-range"
          max={SURFACE_SCALE_MAX}
          min={SURFACE_SCALE_MIN}
          step={SURFACE_SCALE_STEP}
          type="range"
          value={normalizedValue}
          onChange={(event) =>
            onChange(normalizeSurfaceScale(Number(event.currentTarget.value)))
          }
        />
        <button
          aria-label="放大 Ian"
          className="ian-size-button"
          disabled={normalizedValue >= SURFACE_SCALE_MAX}
          type="button"
          onClick={() => onChange(stepSurfaceScale(normalizedValue, 1))}
        >
          +
        </button>
        <span className="ian-size-value">{formatSurfaceScalePercent(normalizedValue)}</span>
      </div>
    </div>
  );
}

type SettingsSelectProps = {
  label: string;
  value: string;
  options: ReadonlyArray<{ label: string; value: string }>;
  onChange: (value: string) => void;
};

function SettingsSelect({ label, value, options, onChange }: SettingsSelectProps) {
  return (
    <label className="ian-settings-select-row">
      <span>{label}</span>
      <select
        aria-label={label}
        value={value}
        onChange={(event) => onChange(event.currentTarget.value)}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  );
}

type CapabilityToggleProps = {
  checked: boolean;
  canDisable?: string;
  description?: string;
  doesNotRead?: string;
  label: string;
  reads?: string;
  onChange: (enabled: boolean) => void;
};

function CapabilityToggle({
  canDisable,
  checked,
  description,
  doesNotRead,
  label,
  reads,
  onChange,
}: CapabilityToggleProps) {
  return (
    <label className="ian-settings-toggle-row">
      <span>
        {label}
        {reads && doesNotRead ? (
          <small>
            会读取：{reads}
            <br />
            不会读取：{doesNotRead}
            <br />
            {canDisable}
          </small>
        ) : description ? (
          <small>{description}</small>
        ) : null}
      </span>
      <input
        aria-label={label}
        checked={checked}
        type="checkbox"
        onChange={(event) => onChange(event.currentTarget.checked)}
      />
    </label>
  );
}

function capabilityChecked(
  capability: string,
  state: {
    gitMetadataEnabled: boolean;
    buildTestEventsEnabled: boolean;
    keyboardRhythmEnabled: boolean;
    activeAppPresenceEnabled: boolean;
  },
): boolean {
  switch (capability) {
    case "git_metadata":
      return state.gitMetadataEnabled;
    case "build_test_events":
      return state.buildTestEventsEnabled;
    case "keyboard_rhythm":
      return state.keyboardRhythmEnabled;
    case "active_app_presence":
      return state.activeAppPresenceEnabled;
    default:
      return false;
  }
}

function snoozeValue(snooze: DeveloperSnooze): string {
  if (!snooze.enabled) {
    return "off";
  }

  return snooze.reason ?? "30m";
}

function optionToSnooze(value: string, nowMs: number): DeveloperSnooze {
  if (value === "off") {
    return { enabled: false, until_ms: null, reason: null };
  }

  const option = developerSnoozeOptions.find((candidate) => candidate.value === value);
  const untilMs =
    option?.durationMs === null
      ? endOfLocalDayMs(nowMs)
      : nowMs + (option?.durationMs ?? 30 * 60 * 1000);

  return { enabled: true, until_ms: untilMs, reason: value };
}

function endOfLocalDayMs(nowMs: number): number {
  const date = new Date(nowMs);
  date.setHours(23, 59, 59, 999);
  return date.getTime();
}
