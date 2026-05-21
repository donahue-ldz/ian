import { useState, type FormEvent, type KeyboardEvent } from "react";
import type { IanViewState } from "../state/ianActions";
import { formatBubbleText, normalizeBubbleMessage } from "./bubbleModel";

type BubbleProps = {
  bubble: IanViewState["bubble"];
  onSubmitMessage: (text: string) => void;
  onInputStarted?: () => void;
  onInputEnded?: () => void;
};

export function Bubble({
  bubble,
  onSubmitMessage,
  onInputEnded,
  onInputStarted,
}: BubbleProps) {
  const [draft, setDraft] = useState("");
  const [isInputVisible, setIsInputVisible] = useState(false);

  if (!bubble.isOpen || !bubble.text) {
    return null;
  }

  const formattedText = formatBubbleText(bubble.text);
  const cloudSize = cloudSizeForText(formattedText);

  function submitDraft(rawText = draft) {
    const message = normalizeBubbleMessage(rawText);

    if (!message) {
      return;
    }

    onSubmitMessage(message);
    setDraft("");
    setIsInputVisible(false);
    onInputEnded?.();
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const input = event.currentTarget.elements.namedItem("bubble-message");
    submitDraft(input instanceof HTMLInputElement ? input.value : draft);
  }

  function handleInputKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key !== "Enter") {
      return;
    }

    event.preventDefault();
    submitDraft(event.currentTarget.value);
  }

  function openInput() {
    if (!isInputVisible) {
      setIsInputVisible(true);
      onInputStarted?.();
    }
  }

  function handleInputBlur() {
    if (!draft.trim()) {
      setIsInputVisible(false);
    }
    onInputEnded?.();
  }

  return (
    <div
      className="ian-bubble"
      data-bubble-role="pet-status"
      data-bubble-shape="thought-cloud"
      data-cloud-size={isInputVisible ? "input" : cloudSize}
      data-mood={bubble.mood ?? "calm"}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <span
        aria-hidden="true"
        className="ian-bubble-tail-dot"
        data-cloud-part="tail-large"
      />
      <span
        aria-hidden="true"
        className="ian-bubble-tail-dot"
        data-cloud-part="tail-small"
      />
      <div className="ian-bubble-text">{formattedText}</div>
      {isInputVisible ? (
        <form className="ian-bubble-form" onSubmit={handleSubmit}>
          <input
            aria-label="对 Ian 说一句话"
            autoFocus
            className="ian-bubble-input"
            maxLength={80}
            name="bubble-message"
            placeholder="说一句..."
            value={draft}
            onChange={(event) => setDraft(event.currentTarget.value)}
            onBlur={handleInputBlur}
            onFocus={openInput}
            onKeyDown={handleInputKeyDown}
          />
        </form>
      ) : (
        <button
          aria-label="打开气泡输入"
          className="ian-bubble-reply"
          data-control-treatment="low-interruption"
          title="对 Ian 说一句话"
          type="button"
          onClick={openInput}
        >
          <span aria-hidden="true">+</span>
        </button>
      )}
    </div>
  );
}

function cloudSizeForText(text: string): "short" | "medium" | "long" {
  const length = Array.from(text).length;

  if (length <= 8) {
    return "short";
  }

  if (length <= 20) {
    return "medium";
  }

  return "long";
}
