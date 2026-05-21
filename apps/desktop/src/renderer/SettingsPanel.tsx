import type { BehaviorMode } from "../protocol/generated";
import { behaviorModeOptions } from "./settingsModel";

type SettingsPanelProps = {
  behaviorMode: BehaviorMode;
  remindersEnabled: boolean;
  byomEnabled: boolean;
  gitMetadataEnabled: boolean;
  buildTestEventsEnabled: boolean;
  keyboardRhythmEnabled: boolean;
  activeAppPresenceEnabled: boolean;
  isOpen: boolean;
  onClose: () => void;
  onModeChange: (mode: BehaviorMode) => void;
  onRemindersEnabledChange: (enabled: boolean) => void;
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
  isOpen,
  onClose,
  onModeChange,
  onRemindersEnabledChange,
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
