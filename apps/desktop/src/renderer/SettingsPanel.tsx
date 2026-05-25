import type {
  BehaviorMode,
  DeveloperSnooze,
  DeveloperWorkspace,
  PlayfulEnergy,
  QuietHours,
} from "../protocol/generated";
import type { FindIanShortcutStatus } from "../lib/findIanShortcut";
import { formatFindIanShortcut } from "../lib/findIanShortcut";
import type { MemoryCandidateView, MemoryExportRecord } from "../lib/tauriBridge";
import { BUILT_IN_PET_RESOURCE_PACKS } from "../resources/resourceLoader";
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
  reminderIntervalMinutes: number;
  doNotDisturb: boolean;
  byomEnabled: boolean;
  byomKeyConfigured: boolean;
  gitMetadataEnabled: boolean;
  buildTestEventsEnabled: boolean;
  keyboardRhythmEnabled: boolean;
  activeAppPresenceEnabled: boolean;
  privacyOnboardingSeen: boolean;
  findIanShortcutEnabled: boolean;
  findIanShortcut: string;
  findIanShortcutStatus: FindIanShortcutStatus;
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
  activePetId: string;
  memoryCandidates: MemoryCandidateView[];
  memoryExportRecords: MemoryExportRecord[];
  isMemoryLoading: boolean;
  memoryError: string | null;
  isOpen: boolean;
  onClose: () => void;
  onModeChange: (mode: BehaviorMode) => void;
  onRemindersEnabledChange: (enabled: boolean) => void;
  onReminderIntervalMinutesChange: (intervalMinutes: number) => void;
  onDoNotDisturbChange: (enabled: boolean) => void;
  onPrivacyOnboardingSeenChange: (seen: boolean) => void;
  onFindIan: () => void;
  onFindIanShortcutEnabledChange: (enabled: boolean) => void;
  onQuietHoursChange: (quietHours: QuietHours) => void;
  onDeveloperWorkspaceChange: (workspace: DeveloperWorkspace) => void;
  onDeveloperSnoozeChange: (snooze: DeveloperSnooze) => void;
  onPetChange: (petId: string) => void;
  onResetAppearance: () => void;
  onResetPosition: () => void;
  onConfirmMemoryCandidate: (id: number) => void;
  onDeleteMemoryCandidate: (id: number) => void;
  onClearMemoryCandidates: () => void;
  onRefreshMemoryExport: () => void;
  onClearInteractionJournal: () => void;
  onResetLocalSettings: () => void;
  onMomentDebugTrigger: (kind: string) => void;
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
  reminderIntervalMinutes,
  doNotDisturb,
  byomEnabled,
  byomKeyConfigured,
  gitMetadataEnabled,
  buildTestEventsEnabled,
  keyboardRhythmEnabled,
  activeAppPresenceEnabled,
  privacyOnboardingSeen,
  findIanShortcutEnabled,
  findIanShortcut,
  findIanShortcutStatus,
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
  activePetId,
  memoryCandidates,
  memoryExportRecords,
  isMemoryLoading,
  memoryError,
  isOpen,
  onClose,
  onModeChange,
  onRemindersEnabledChange,
  onReminderIntervalMinutesChange,
  onDoNotDisturbChange,
  onPrivacyOnboardingSeenChange,
  onFindIan,
  onFindIanShortcutEnabledChange,
  onQuietHoursChange,
  onDeveloperWorkspaceChange,
  onDeveloperSnoozeChange,
  onPetChange,
  onResetAppearance,
  onResetPosition,
  onConfirmMemoryCandidate,
  onDeleteMemoryCandidate,
  onClearMemoryCandidates,
  onRefreshMemoryExport,
  onClearInteractionJournal,
  onResetLocalSettings,
  onMomentDebugTrigger,
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
      <section className="ian-settings-section" aria-labelledby="ian-settings-interaction">
        <h3 id="ian-settings-interaction">互动</h3>
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
      <section className="ian-settings-section" aria-labelledby="ian-settings-appearance">
        <h3 id="ian-settings-appearance">外观</h3>
        <label className="ian-settings-select-row">
          <span>宠物</span>
          <select
            aria-label="选择宠物"
            value={activePetId}
            onChange={(event) => onPetChange(event.currentTarget.value)}
          >
            {BUILT_IN_PET_RESOURCE_PACKS.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <div className="ian-pet-preview-grid" aria-label="宠物预览">
          {BUILT_IN_PET_RESOURCE_PACKS.map((option) => (
            <button
              aria-label={`切换到${option.label}`}
              aria-pressed={option.id === activePetId}
              className="ian-pet-preview-card"
              data-active={option.id === activePetId}
              data-preview-animation={option.previewAnimation}
              key={option.id}
              type="button"
              onClick={() => onPetChange(option.id)}
            >
              <span className="ian-pet-preview-mark" aria-hidden="true">
                {option.label.slice(0, 1)}
              </span>
              <span>
                {option.label}
                <small>{option.previewTone}</small>
              </span>
            </button>
          ))}
        </div>
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
        <SettingsSelect
          label="提醒间隔"
          value={String(reminderIntervalMinutes)}
          options={[
            { label: "30 分钟", value: "30" },
            { label: "45 分钟", value: "45" },
            { label: "60 分钟", value: "60" },
            { label: "90 分钟", value: "90" },
            { label: "120 分钟", value: "120" },
          ]}
          onChange={(value) => onReminderIntervalMinutesChange(Number(value))}
        />
      </section>
      <section className="ian-settings-section" aria-labelledby="ian-settings-recovery">
        <h3 id="ian-settings-recovery">找回</h3>
        <div className="ian-settings-row">
          <span>
            找回 Ian
            <small>
              只监听一个找回 Ian 快捷键，不记录输入内容。可以从托盘菜单找回 Ian。
            </small>
          </span>
          <button type="button" aria-label="找回 Ian" onClick={onFindIan}>
            找回 Ian
          </button>
        </div>
        <label className="ian-settings-toggle-row">
          <span>
            快捷键
            <small>
              {formatFindIanShortcut(findIanShortcut)}
              {findIanShortcutStatus === "conflict" ? " · 快捷键被其他应用占用" : ""}
              {findIanShortcutStatus === "registered" ? " · 已启用" : ""}
              {" · 快捷键冲突时，可以先用设置或托盘找回。"}
            </small>
          </span>
          <input
            aria-label="启用找回 Ian 快捷键"
            checked={findIanShortcutEnabled}
            type="checkbox"
            onChange={(event) => onFindIanShortcutEnabledChange(event.currentTarget.checked)}
          />
        </label>
        <label className="ian-settings-toggle-row">
          <span>勿扰</span>
          <input
            aria-label="启用勿扰"
            checked={doNotDisturb}
            type="checkbox"
            onChange={(event) => onDoNotDisturbChange(event.currentTarget.checked)}
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
        <p className="ian-settings-note">找回时只显示短暂信标，不读取屏幕内容。</p>
      </section>
      <section className="ian-settings-section" aria-labelledby="ian-settings-privacy">
        <h3 id="ian-settings-privacy">隐私</h3>
        <p className="ian-settings-note">默认不读取代码、窗口标题、终端全文或按键内容。</p>
        <div className="ian-permission-center">
          <strong>隐私中心</strong>
          <p className="ian-settings-note">
            Ian 默认本地优先。高敏能力默认关闭，开启前会说明读取范围。
          </p>
          <p className="ian-settings-note">
            快捷键：{findIanShortcutEnabled ? "开启" : "关闭"} · 记忆：候选{" "}
            {memoryCandidates.length} 条 · 提醒：{remindersEnabled ? "开启" : "关闭"} ·
            开发者状态：{developerWorkspace.bound ? "已绑定" : "未绑定"}
          </p>
          <label className="ian-settings-toggle-row">
            <span>已了解隐私说明</span>
            <input
              aria-label="已了解隐私说明"
              checked={privacyOnboardingSeen}
              type="checkbox"
              onChange={(event) =>
                onPrivacyOnboardingSeenChange(event.currentTarget.checked)
              }
            />
          </label>
        </div>
        <details className="ian-settings-developer">
          <summary>记忆候选</summary>
          <p className="ian-settings-note">
            Ian 只会把低敏标签作为候选，确认后才会本地保存。本地保存，可删除。
          </p>
          <MemoryCandidateList
            candidates={memoryCandidates}
            error={memoryError}
            isLoading={isMemoryLoading}
            onConfirm={onConfirmMemoryCandidate}
            onDelete={onDeleteMemoryCandidate}
          />
          <div className="ian-settings-reset-row">
            <button
              type="button"
              aria-label="清空记忆候选"
              onClick={onClearMemoryCandidates}
            >
              清空候选
            </button>
            <button type="button" aria-label="导出记忆摘要" onClick={onRefreshMemoryExport}>
              导出摘要
            </button>
          </div>
          <MemoryExportSummary records={memoryExportRecords} />
        </details>
        <details className="ian-settings-developer">
          <summary>未来社交和插件能力</summary>
          <p className="ian-settings-note">
            Feishu、Pet Visit、插件系统默认关闭。默认无遥测，诊断需要手动导出。
          </p>
        </details>
        <details className="ian-settings-developer">
          <summary>开发者</summary>
          <p className="ian-settings-note">可选开发节奏，想让 Ian 理解一点开发节奏时再开启。</p>
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
        <summary>开发者</summary>
        <div className="ian-settings-row">
          <span>对话模式</span>
          <div className="ian-segmented" role="group" aria-label="对话模式">
            <button
              className="ian-segmented-option"
              data-active={!byomEnabled}
              type="button"
              onClick={() => onCapabilityEnabledChange("byom", false)}
            >
              Demo
            </button>
            <button
              className="ian-segmented-option"
              data-active={byomEnabled}
              disabled={!byomKeyConfigured}
              type="button"
              onClick={() => onCapabilityEnabledChange("byom", true)}
            >
              自带模型
            </button>
          </div>
        </div>
        <label className="ian-settings-toggle-row">
          <span>
            自带模型
            {!byomKeyConfigured ? <small>先保存本地 key 后才能启用</small> : null}
          </span>
          <input
            aria-label="启用自带模型"
            checked={byomEnabled}
            disabled={!byomKeyConfigured}
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
        <div className="ian-settings-reset-row">
          <button type="button" aria-label="恢复默认外观" onClick={onResetAppearance}>
            默认外观
          </button>
          <button type="button" aria-label="回到回家点" onClick={onResetPosition}>
            回家点
          </button>
        </div>
        <p className="ian-settings-note">回家点会保存到本地，下次启动继续回到这里。</p>
        <div className="ian-permission-center">
          <strong>能力状态</strong>
          <p className="ian-settings-note">
            资源包：内置可用 · 自带模型：
            {byomKeyConfigured ? "已配置本地 key" : "未配置本地 key"} ·
            快捷键：{findIanShortcutStatus === "registered" ? "已启用" : "未启用"} ·
            提醒：{remindersEnabled ? "开启" : "关闭"}
          </p>
          <p className="ian-settings-note">资源包异常时会退回内置宠物。</p>
        </div>
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
        {diagnosticsEnabled ? (
          <div className="ian-permission-center" aria-label="Moment 诊断">
            <strong>Moment 诊断</strong>
            <p className="ian-settings-note">只触发本地 Core action，不读取屏幕内容。</p>
            <div className="ian-settings-reset-row">
              {momentDebugOptions.map((option) => (
                <button
                  aria-label={`诊断触发${option.label}`}
                  key={option.kind}
                  type="button"
                  onClick={() => onMomentDebugTrigger(option.kind)}
                >
                  {option.label}
                </button>
              ))}
            </div>
          </div>
        ) : null}
      </details>
      <section className="ian-settings-section" aria-labelledby="ian-settings-data">
        <h3 id="ian-settings-data">数据</h3>
        <p className="ian-settings-note">
          只清理 Ian 本地数据，不触碰外部账号或远端内容。
        </p>
        <div className="ian-settings-reset-row">
          <button type="button" aria-label="清空交互日志" onClick={onClearInteractionJournal}>
            清空日志
          </button>
          <button type="button" aria-label="恢复设置默认值" onClick={onResetLocalSettings}>
            默认设置
          </button>
        </div>
      </section>
    </aside>
  );
}

const momentDebugOptions = [
  { kind: "find_ian_entrance", label: "找回入场" },
  { kind: "pointer_curiosity", label: "鼠标好奇" },
  { kind: "drag_carry", label: "抱起" },
  { kind: "drop_settle", label: "放下" },
  { kind: "rare_idle_surprise", label: "Idle 惊喜" },
  { kind: "memory_echo", label: "记忆回响" },
  { kind: "idle_peek_around", label: "偷偷张望" },
  { kind: "idle_tiny_patrol", label: "小巡逻" },
  { kind: "idle_pretend_innocent", label: "装作无事" },
] as const;

type MemoryCandidateListProps = {
  candidates: MemoryCandidateView[];
  error: string | null;
  isLoading: boolean;
  onConfirm: (id: number) => void;
  onDelete: (id: number) => void;
};

function MemoryCandidateList({
  candidates,
  error,
  isLoading,
  onConfirm,
  onDelete,
}: MemoryCandidateListProps) {
  if (error) {
    return <p className="ian-settings-note">{error}</p>;
  }

  if (isLoading) {
    return <p className="ian-settings-note">正在读取本地候选。</p>;
  }

  if (candidates.length === 0) {
    return <p className="ian-settings-note">现在没有待确认的小记忆。</p>;
  }

  return (
    <ul className="ian-memory-candidate-list" aria-label="记忆候选列表">
      {candidates.map((candidate) => (
        <li className="ian-memory-candidate" key={candidate.id}>
          <span>
            {candidate.tags.join(" · ")}
            <small>{formatMemoryCandidateTime(candidate.created_at_ms)}</small>
          </span>
          <span className="ian-memory-actions">
            <button
              type="button"
              aria-label={`确认记忆候选 ${candidate.id}`}
              onClick={() => onConfirm(candidate.id)}
            >
              确认
            </button>
            <button
              type="button"
              aria-label={`删除记忆候选 ${candidate.id}`}
              onClick={() => onDelete(candidate.id)}
            >
              删除
            </button>
          </span>
        </li>
      ))}
    </ul>
  );
}

type MemoryExportSummaryProps = {
  records: MemoryExportRecord[];
};

function MemoryExportSummary({ records }: MemoryExportSummaryProps) {
  if (records.length === 0) {
    return <p className="ian-settings-note">导出记忆摘要：暂无已保存摘要。</p>;
  }

  return (
    <ul className="ian-memory-candidate-list" aria-label="记忆摘要导出预览">
      {records.map((record) => (
        <li className="ian-memory-candidate" key={`${record.status}-${record.id}`}>
          <span>
            {record.tags.join(" · ")}
            <small>
              {record.status} · {formatMemoryCandidateTime(record.created_at_ms)}
            </small>
          </span>
        </li>
      ))}
    </ul>
  );
}

function formatMemoryCandidateTime(timestampMs: number): string {
  if (!Number.isFinite(timestampMs) || timestampMs <= 0) {
    return "本地候选";
  }

  return new Date(timestampMs).toLocaleDateString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
  });
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
