import type {
  BehaviorMode,
  DeveloperSnooze,
  DeveloperWorkspace,
  QuietHours,
} from "../protocol/generated";
import {
  behaviorModeOptions,
  bubbleFrequencyOptions,
  developerCapabilityOptions,
  developerSnoozeOptions,
  fromTimeInputValue,
  movementIntensityOptions,
  restBehaviorOptions,
  toTimeInputValue,
} from "./settingsModel";

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
      <div className="ian-settings-row">
        <span>行为</span>
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
      <label className="ian-settings-toggle-row">
        <span>提醒</span>
        <input
          aria-label="启用提醒"
          checked={remindersEnabled}
          type="checkbox"
          onChange={(event) => onRemindersEnabledChange(event.currentTarget.checked)}
        />
      </label>
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
      <SettingsSelect
        label="移动"
        value={movementIntensity}
        options={movementIntensityOptions}
        onChange={(value) =>
          onCreatureSettingsChange({
            movementIntensity: value,
            bubbleFrequency,
            restBehavior,
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
            surfaceScale,
            diagnosticsEnabled,
          })
        }
      />
      <SettingsSelect
        label="休息"
        value={restBehavior}
        options={restBehaviorOptions}
        onChange={(value) =>
          onCreatureSettingsChange({
            movementIntensity,
            bubbleFrequency,
            restBehavior: value,
            surfaceScale,
            diagnosticsEnabled,
          })
        }
      />
      <label className="ian-settings-toggle-row">
        <span>大小</span>
        <input
          aria-label="Ian 大小"
          type="range"
          min="0.8"
          max="1.4"
          step="0.1"
          value={surfaceScale}
          onChange={(event) =>
            onCreatureSettingsChange({
              movementIntensity,
              bubbleFrequency,
              restBehavior,
              surfaceScale: Number(event.currentTarget.value),
              diagnosticsEnabled,
            })
          }
        />
      </label>
      <div className="ian-settings-capabilities" aria-label="能力状态">
        <span>开发者节奏</span>
        <label className="ian-settings-toggle-row">
          <span>工作区</span>
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
        <CapabilityToggle
          checked={byomEnabled}
          label="BYOM"
          onChange={(enabled) => onCapabilityEnabledChange("byom", enabled)}
        />
        {developerCapabilityOptions.map((option) => (
          <CapabilityToggle
            checked={capabilityChecked(option.capability, {
              gitMetadataEnabled,
              buildTestEventsEnabled,
              keyboardRhythmEnabled,
              activeAppPresenceEnabled,
            })}
            description={option.scope}
            key={option.capability}
            label={option.label}
            onChange={(enabled) => onCapabilityEnabledChange(option.capability, enabled)}
          />
        ))}
      </div>
    </aside>
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
  description?: string;
  label: string;
  onChange: (enabled: boolean) => void;
};

function CapabilityToggle({ checked, description, label, onChange }: CapabilityToggleProps) {
  return (
    <label className="ian-settings-toggle-row">
      <span>
        {label}
        {description ? <small>{description}</small> : null}
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
