import { useRef, useState, type MouseEvent, type PointerEvent } from "react";
import type { IanViewState } from "../state/ianActions";
import type { PetResourcePack } from "../resources/resourceLoader";
import type { BehaviorMode, QuietHours } from "../protocol/generated";
import { Bubble } from "./Bubble";
import { getDragOffset, shouldStartDrag } from "./dragGesture";
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
  isSettingsOpen: boolean;
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
  onQuietHoursChange: (quietHours: QuietHours) => void;
  onCreatureSettingsChange: (settings: {
    movementIntensity: string;
    bubbleFrequency: string;
    restBehavior: string;
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
  isSettingsOpen,
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
  onQuietHoursChange,
  onCreatureSettingsChange,
  onCapabilityEnabledChange,
  onDragStart,
  onDragEnd,
}: IanStageProps) {
  const dragOrigin = useRef<Point | null>(null);
  const isDragging = useRef(false);
  const suppressNextClick = useRef(false);
  const [dragOffset, setDragOffset] = useState<Point>({ x: 0, y: 0 });

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
    <main className="ian-stage">
      <section
        className="ian-creature-surface"
        style={{
          transform: `translate(${viewState.position.x + dragOffset.x}px, ${
            viewState.position.y + dragOffset.y
          }px) scale(${surfaceScale})`,
        }}
        onPointerDown={(event) => {
          if (!(event.target as HTMLElement).closest(".ian-click-target")) {
            return;
          }

          dragOrigin.current = pointFromPointer(event);
          isDragging.current = false;
        }}
        onPointerMove={(event) => {
          if (!dragOrigin.current) return;
          const point = pointFromPointer(event);
          if (!isDragging.current) {
            if (!shouldStartDrag(dragOrigin.current, point)) {
              return;
            }

            isDragging.current = true;
            onDragStart(dragOrigin.current);
            event.currentTarget.setPointerCapture(event.pointerId);
          }

          setDragOffset(getDragOffset(dragOrigin.current, point));
        }}
        onPointerUp={(event) => {
          const point = pointFromPointer(event);
          if (dragOrigin.current && isDragging.current) {
            onDragEnd(point);
            suppressNextClick.current = true;
          }
          dragOrigin.current = null;
          isDragging.current = false;
          setDragOffset({ x: 0, y: 0 });
          if (event.currentTarget.hasPointerCapture(event.pointerId)) {
            event.currentTarget.releasePointerCapture(event.pointerId);
          }
        }}
        onPointerCancel={(event) => {
          dragOrigin.current = null;
          isDragging.current = false;
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
          aria-label="打开设置"
          className="ian-settings-toggle"
          type="button"
          onPointerDown={(event) => event.stopPropagation()}
          onClick={onSettingsToggle}
        >
          设置
        </button>
        <SettingsPanel
          behaviorMode={behaviorMode}
          remindersEnabled={remindersEnabled}
          byomEnabled={byomEnabled}
          gitMetadataEnabled={gitMetadataEnabled}
          buildTestEventsEnabled={buildTestEventsEnabled}
          keyboardRhythmEnabled={keyboardRhythmEnabled}
          activeAppPresenceEnabled={activeAppPresenceEnabled}
          quietHours={quietHours}
          movementIntensity={movementIntensity}
          bubbleFrequency={bubbleFrequency}
          restBehavior={restBehavior}
          surfaceScale={surfaceScale}
          diagnosticsEnabled={diagnosticsEnabled}
          isOpen={isSettingsOpen}
          onClose={onSettingsClose}
          onModeChange={onBehaviorModeChange}
          onRemindersEnabledChange={onRemindersEnabledChange}
          onQuietHoursChange={onQuietHoursChange}
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
          <IanSprite
            animation={viewState.animation.name}
            resourcePack={resourcePack}
          />
        </button>
      </section>
    </main>
  );
}
