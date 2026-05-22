import { useRef, useState, type MouseEvent, type PointerEvent } from "react";
import type { IanViewState } from "../state/ianActions";
import type { PetResourcePack } from "../resources/resourceLoader";
import type { FindIanShortcutStatus } from "../lib/findIanShortcut";
import type {
  BehaviorMode,
  DeveloperSnooze,
  DeveloperWorkspace,
  PlayfulEnergy,
  QuietHours,
} from "../protocol/generated";
import { Bubble } from "./Bubble";
import {
  getDragOffset,
  shouldStartDrag,
} from "./dragGesture";
import { IanSprite } from "./IanSprite";
import { SettingsPanel } from "./SettingsPanel";
import "./ianStage.css";

type Point = {
  x: number;
  y: number;
};

type IanStageProps = {
  resourcePack: PetResourcePack | null;
  viewState: IanViewState;
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
  isSettingsOpen: boolean;
  isDesktopWindow: boolean;
  onIanClick: (point: Point) => void;
  onIanDoubleClick: (point: Point) => void;
  onIanNear: (point: Point) => void;
  onIanLeave: (point: Point) => void;
  onSubmitMessage: (text: string) => void;
  onBubbleInputStarted: () => void;
  onBubbleInputEnded: () => void;
  onSettingsToggle: () => void;
  onSettingsClose: () => void;
  onBehaviorModeChange: (mode: BehaviorMode) => void;
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
  onDragStart: (point: Point) => void;
  onDragEnd: (point: Point) => void;
};

export function IanStage({
  resourcePack,
  viewState,
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
  isSettingsOpen,
  isDesktopWindow,
  onIanClick,
  onIanDoubleClick,
  onIanNear,
  onIanLeave,
  onSubmitMessage,
  onBubbleInputStarted,
  onBubbleInputEnded,
  onSettingsToggle,
  onSettingsClose,
  onBehaviorModeChange,
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
  onCreatureSettingsChange,
  onCapabilityEnabledChange,
  onDragStart,
  onDragEnd,
}: IanStageProps) {
  const dragOrigin = useRef<Point | null>(null);
  const isDragging = useRef(false);
  const suppressNextClick = useRef(false);
  const [dragOffset, setDragOffset] = useState<Point>({ x: 0, y: 0 });
  const [isDraggingView, setIsDraggingView] = useState(false);

  function pointFromPointer(event: PointerEvent<HTMLElement>): Point {
    return pointFromClient(event);
  }

  function pointFromMouse(event: MouseEvent<HTMLElement>): Point {
    return pointFromClient(event);
  }

  function pointFromClient(event: { clientX: number; clientY: number }): Point {
    return {
      x: event.clientX,
      y: event.clientY,
    };
  }

  return (
    <main
      className="ian-stage"
      data-window-context={isDesktopWindow ? "desktop" : "preview"}
    >
      <section
        className="ian-creature-surface"
        data-behavior-mode={behaviorMode}
        data-dragging={isDraggingView ? "true" : "false"}
        style={{
          transform: `translate(${viewState.position.x + dragOffset.x}px, ${
            viewState.position.y + dragOffset.y
          }px) scale(${surfaceScale * viewState.appearanceScale})`,
        }}
        onPointerDown={(event) => {
          if (!(event.target as HTMLElement).closest(".ian-click-target")) {
            return;
          }

          dragOrigin.current = pointFromPointer(event);
          isDragging.current = false;
          setIsDraggingView(false);
        }}
        onPointerMove={(event) => {
          if (!dragOrigin.current) return;
          const point = pointFromPointer(event);
          if (!isDragging.current) {
            if (!shouldStartDrag(dragOrigin.current, point)) {
              return;
            }

            isDragging.current = true;
            setIsDraggingView(true);
            onDragStart(dragOrigin.current);
            event.currentTarget.setPointerCapture(event.pointerId);
          }

          const offset = getDragOffset(dragOrigin.current, point);
          if (isDesktopWindow) {
            return;
          } else {
            setDragOffset(offset);
          }
        }}
        onPointerUp={(event) => {
          const point = pointFromPointer(event);
          if (dragOrigin.current && isDragging.current) {
            onDragEnd(point);
            suppressNextClick.current = true;
          }
          dragOrigin.current = null;
          isDragging.current = false;
          setIsDraggingView(false);
          setDragOffset({ x: 0, y: 0 });
          if (event.currentTarget.hasPointerCapture(event.pointerId)) {
            event.currentTarget.releasePointerCapture(event.pointerId);
          }
        }}
        onPointerCancel={(event) => {
          dragOrigin.current = null;
          isDragging.current = false;
          setIsDraggingView(false);
          setDragOffset({ x: 0, y: 0 });
          if (event.currentTarget.hasPointerCapture(event.pointerId)) {
            event.currentTarget.releasePointerCapture(event.pointerId);
          }
        }}
      >
        <Bubble
          bubble={viewState.bubble}
          onInputEnded={onBubbleInputEnded}
          onInputStarted={onBubbleInputStarted}
          onSubmitMessage={onSubmitMessage}
        />
        <button
          aria-expanded={isSettingsOpen}
          aria-label="打开 Ian 设置"
          className="ian-settings-toggle"
          title="打开 Ian 设置"
          type="button"
          onPointerDown={(event) => event.stopPropagation()}
          onClick={onSettingsToggle}
        >
          <span aria-hidden="true">⚙</span>
        </button>
        <SettingsPanel
          behaviorMode={behaviorMode}
          remindersEnabled={remindersEnabled}
          reminderIntervalMinutes={reminderIntervalMinutes}
          doNotDisturb={doNotDisturb}
          byomEnabled={byomEnabled}
          byomKeyConfigured={byomKeyConfigured}
          gitMetadataEnabled={gitMetadataEnabled}
          buildTestEventsEnabled={buildTestEventsEnabled}
          keyboardRhythmEnabled={keyboardRhythmEnabled}
          activeAppPresenceEnabled={activeAppPresenceEnabled}
          privacyOnboardingSeen={privacyOnboardingSeen}
          findIanShortcutEnabled={findIanShortcutEnabled}
          findIanShortcut={findIanShortcut}
          findIanShortcutStatus={findIanShortcutStatus}
          quietHours={quietHours}
          developerWorkspace={developerWorkspace}
          developerSnooze={developerSnooze}
          movementIntensity={movementIntensity}
          bubbleFrequency={bubbleFrequency}
          restBehavior={restBehavior}
          playfulEnergy={playfulEnergy}
          playfulSnoozedUntilMs={playfulSnoozedUntilMs}
          surfaceScale={surfaceScale}
          diagnosticsEnabled={diagnosticsEnabled}
          activePetId={activePetId}
          isOpen={isSettingsOpen}
          onClose={onSettingsClose}
          onModeChange={onBehaviorModeChange}
          onRemindersEnabledChange={onRemindersEnabledChange}
          onReminderIntervalMinutesChange={onReminderIntervalMinutesChange}
          onDoNotDisturbChange={onDoNotDisturbChange}
          onPrivacyOnboardingSeenChange={onPrivacyOnboardingSeenChange}
          onFindIan={onFindIan}
          onFindIanShortcutEnabledChange={onFindIanShortcutEnabledChange}
          onQuietHoursChange={onQuietHoursChange}
          onDeveloperWorkspaceChange={onDeveloperWorkspaceChange}
          onDeveloperSnoozeChange={onDeveloperSnoozeChange}
          onPetChange={onPetChange}
          onResetAppearance={onResetAppearance}
          onResetPosition={onResetPosition}
          onCreatureSettingsChange={onCreatureSettingsChange}
          onCapabilityEnabledChange={onCapabilityEnabledChange}
        />
        <button
          className="ian-click-target"
          aria-label="Ian"
          onPointerEnter={(event) => onIanNear(pointFromPointer(event))}
          onPointerLeave={(event) => onIanLeave(pointFromPointer(event))}
          onClick={(event) => {
            if (suppressNextClick.current) {
              suppressNextClick.current = false;
              event.preventDefault();
              event.stopPropagation();
              return;
            }

            onIanClick(pointFromMouse(event));
          }}
          onDoubleClick={(event) => onIanDoubleClick(pointFromMouse(event))}
        >
          {viewState.visualEffect ? (
            <span
              aria-hidden="true"
              className="ian-visual-effect"
              data-effect={viewState.visualEffect.name}
              data-intensity={viewState.visualEffect.intensity}
            />
          ) : null}
          <IanSprite
            animation={viewState.animation.name}
            resourcePack={resourcePack}
          />
        </button>
      </section>
    </main>
  );
}
