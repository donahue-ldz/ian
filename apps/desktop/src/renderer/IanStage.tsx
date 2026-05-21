import { useRef, useState, type MouseEvent, type PointerEvent } from "react";
import type { IanViewState } from "../state/ianActions";
import type { PetResourcePack } from "../resources/resourceLoader";
import type { BehaviorMode } from "../protocol/generated";
import { Bubble } from "./Bubble";
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
  isSettingsOpen: boolean;
  onIanClick: (point: Point) => void;
  onIanDoubleClick: (point: Point) => void;
  onIanNear: (point: Point) => void;
  onSubmitMessage: (text: string) => void;
  onSettingsToggle: () => void;
  onSettingsClose: () => void;
  onBehaviorModeChange: (mode: BehaviorMode) => void;
  onDragEnd: (point: Point) => void;
};

export function IanStage({
  resourcePack,
  viewState,
  behaviorMode,
  isSettingsOpen,
  onIanClick,
  onIanDoubleClick,
  onIanNear,
  onSubmitMessage,
  onSettingsToggle,
  onSettingsClose,
  onBehaviorModeChange,
  onDragEnd,
}: IanStageProps) {
  const dragOrigin = useRef<Point | null>(null);
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
          transform: `translate(${dragOffset.x}px, ${dragOffset.y}px)`,
        }}
        onPointerDown={(event) => {
          if (!(event.target as HTMLElement).closest(".ian-click-target")) {
            return;
          }

          dragOrigin.current = pointFromPointer(event);
          event.currentTarget.setPointerCapture(event.pointerId);
        }}
        onPointerMove={(event) => {
          if (!dragOrigin.current) return;
          const point = pointFromPointer(event);
          setDragOffset({
            x: point.x - dragOrigin.current.x,
            y: point.y - dragOrigin.current.y,
          });
        }}
        onPointerUp={(event) => {
          const point = pointFromPointer(event);
          if (dragOrigin.current) {
            onDragEnd(point);
          }
          dragOrigin.current = null;
          setDragOffset({ x: 0, y: 0 });
          event.currentTarget.releasePointerCapture(event.pointerId);
        }}
      >
        <Bubble bubble={viewState.bubble} onSubmitMessage={onSubmitMessage} />
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
          isOpen={isSettingsOpen}
          onClose={onSettingsClose}
          onModeChange={onBehaviorModeChange}
        />
        <button
          className="ian-click-target"
          aria-label="Ian"
          onPointerEnter={(event) => onIanNear(pointFromPointer(event))}
          onClick={(event) => onIanClick(pointFromMouse(event))}
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
