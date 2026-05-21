import type { IanViewState } from "../state/ianActions";

type BubbleProps = {
  bubble: IanViewState["bubble"];
};

export function Bubble({ bubble }: BubbleProps) {
  if (!bubble.isOpen || !bubble.text) {
    return null;
  }

  return (
    <div className="ian-bubble" data-mood={bubble.mood ?? "calm"}>
      {bubble.text}
    </div>
  );
}
