import type { BehaviorMode } from "../protocol/generated";
import { behaviorModeOptions } from "./settingsModel";

type SettingsPanelProps = {
  behaviorMode: BehaviorMode;
  isOpen: boolean;
  onClose: () => void;
  onModeChange: (mode: BehaviorMode) => void;
};

export function SettingsPanel({
  behaviorMode,
  isOpen,
  onClose,
  onModeChange,
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
    </aside>
  );
}
