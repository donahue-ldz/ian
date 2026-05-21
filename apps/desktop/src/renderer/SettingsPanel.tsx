import type { BehaviorMode, QuietHours } from "../protocol/generated";
import {
  behaviorModeOptions,
  bubbleFrequencyOptions,
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
        <span>能力</span>
        <CapabilityToggle
          checked={byomEnabled}
          label="BYOM"
          onChange={(enabled) => onCapabilityEnabledChange("byom", enabled)}
        />
        <CapabilityToggle
          checked={gitMetadataEnabled}
          label="Git 元数据"
          onChange={(enabled) => onCapabilityEnabledChange("git_metadata", enabled)}
        />
        <CapabilityToggle
          checked={buildTestEventsEnabled}
          label="构建测试摘要"
          onChange={(enabled) => onCapabilityEnabledChange("build_test_events", enabled)}
        />
        <CapabilityToggle
          checked={keyboardRhythmEnabled}
          label="键盘节奏"
          onChange={(enabled) => onCapabilityEnabledChange("keyboard_rhythm", enabled)}
        />
        <CapabilityToggle
          checked={activeAppPresenceEnabled}
          label="应用类别"
          onChange={(enabled) => onCapabilityEnabledChange("active_app_presence", enabled)}
        />
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
  label: string;
  onChange: (enabled: boolean) => void;
};

function CapabilityToggle({ checked, label, onChange }: CapabilityToggleProps) {
  return (
    <label className="ian-settings-toggle-row">
      <span>{label}</span>
      <input
        aria-label={label}
        checked={checked}
        type="checkbox"
        onChange={(event) => onChange(event.currentTarget.checked)}
      />
    </label>
  );
}
