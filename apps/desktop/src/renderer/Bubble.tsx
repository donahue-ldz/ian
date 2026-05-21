import { useState, type FormEvent, type KeyboardEvent } from "react";
import type { IanViewState } from "../state/ianActions";
import { normalizeBubbleMessage } from "./bubbleModel";

type BubbleProps = {
  bubble: IanViewState["bubble"];
  onSubmitMessage: (text: string) => void;
};

export function Bubble({ bubble, onSubmitMessage }: BubbleProps) {
  const [draft, setDraft] = useState("");

  if (!bubble.isOpen || !bubble.text) {
    return null;
  }

  function submitDraft() {
    const message = normalizeBubbleMessage(draft);

    if (!message) {
      return;
    }

    onSubmitMessage(message);
    setDraft("");
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    submitDraft();
  }

  function handleInputKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key !== "Enter") {
      return;
    }

    event.preventDefault();
    submitDraft();
  }

  return (
    <div
      className="ian-bubble"
      data-mood={bubble.mood ?? "calm"}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <div className="ian-bubble-text">{bubble.text}</div>
      <form className="ian-bubble-form" onSubmit={handleSubmit}>
        <input
          aria-label="对 Ian 说一句话"
          className="ian-bubble-input"
          maxLength={80}
          placeholder="说一句..."
          value={draft}
          onChange={(event) => setDraft(event.currentTarget.value)}
          onKeyDown={handleInputKeyDown}
        />
      </form>
    </div>
  );
}
