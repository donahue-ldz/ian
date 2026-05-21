import { renderToStaticMarkup } from "react-dom/server";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";
import type { IanViewState } from "../state/ianActions";
import { Bubble } from "./Bubble";

describe("Bubble view", () => {
  it("does not show the input field until the user asks to reply", () => {
    const html = renderToStaticMarkup(
      <Bubble
        bubble={openBubble()}
        onInputEnded={vi.fn()}
        onInputStarted={vi.fn()}
        onSubmitMessage={vi.fn()}
      />,
    );

    expect(html).toContain('aria-label="打开气泡输入"');
    expect(html).not.toContain('aria-label="对 Ian 说一句话"');
  });

  it("uses a cloud thought bubble visual treatment", () => {
    const css = readFileSync("src/renderer/ianStage.css", "utf8");

    expect(css).toContain("--ian-cloud-bubble-fill: #fff");
    expect(css).toContain("--ian-cloud-bubble-ink: rgba(17, 17, 17, 0.64)");
    expect(css).toContain("border: 1.5px solid var(--ian-cloud-bubble-ink)");
    expect(css).not.toContain("--ian-cloud-bubble-ink: #111");
    expect(css).not.toContain("border: 3px solid var(--ian-cloud-bubble-ink)");
    expect(css).not.toContain("border: 4px solid var(--ian-cloud-bubble-ink)");
    expect(css).toMatch(/\.ian-bubble::before\s*{/);
    expect(css).toContain("data-cloud-part=\"tail-large\"");
    expect(css).toContain("data-cloud-part=\"tail-small\"");
    expect(css).toContain("ian-cloud-lobes");
  });

  it("uses extra-thin outlines for all pet bubble strokes", () => {
    const css = readFileSync("src/renderer/ianStage.css", "utf8");

    expect(css).toContain("border: 1.5px solid var(--ian-cloud-bubble-ink)");
    expect(css).toContain("border: 1.5px solid var(--ian-cloud-bubble-ink)");
    expect(css).not.toContain("border: 2px solid var(--ian-cloud-bubble-ink)");
    expect(css).not.toContain("border: 3px solid var(--ian-cloud-bubble-ink)");
  });

  it("centers bubble text while keeping two-line clamping", () => {
    const css = readFileSync("src/renderer/ianStage.css", "utf8");

    expect(css).toMatch(/\.ian-bubble\s*{[^}]*display: grid;/s);
    expect(css).toMatch(/\.ian-bubble\s*{[^}]*place-items: center;/s);
    expect(css).toMatch(/\.ian-bubble-text\s*{[^}]*text-align: center;/s);
    expect(css).toMatch(/\.ian-bubble-text\s*{[^}]*margin: 0;/s);
    expect(css).toMatch(/\.ian-bubble-text\s*{[^}]*align-self: center;/s);
    expect(css).toMatch(/\.ian-bubble-reply\s*{[^}]*position: absolute;/s);
    expect(css).toMatch(/\.ian-bubble-text\s*{[^}]*-webkit-line-clamp: 2;/s);
    expect(css).toContain('.ian-bubble[data-cloud-size="input"]');
    expect(css).toMatch(
      /\.ian-bubble\[data-cloud-size="input"\]\s*{[^}]*place-items: stretch;/s,
    );
  });

  it("marks the default bubble as a desktop pet status bubble", () => {
    const html = renderToStaticMarkup(
      <Bubble
        bubble={openBubble()}
        onInputEnded={vi.fn()}
        onInputStarted={vi.fn()}
        onSubmitMessage={vi.fn()}
      />,
    );

    expect(html).toContain('data-bubble-role="pet-status"');
    expect(html).toContain('data-bubble-shape="thought-cloud"');
    expect(html).toContain('data-control-treatment="low-interruption"');
  });

  it("keeps the desktop pet bubble compact and anchored to Ian", () => {
    const css = readFileSync("src/renderer/ianStage.css", "utf8");

    expect(css).toContain("--ian-pet-bubble-anchor-bottom: 152px");
    expect(css).toContain("bottom: var(--ian-pet-bubble-anchor-bottom)");
    expect(css).toContain("box-shadow: 0 8px 14px rgba(28, 34, 30, 0.08)");
    expect(css).toContain(".ian-bubble-reply[data-control-treatment=\"low-interruption\"]");
    expect(css).toContain("opacity: 0.52;");
  });

  it("renders cloud tail dots outside the text content", () => {
    const html = renderToStaticMarkup(
      <Bubble
        bubble={openBubble()}
        onInputEnded={vi.fn()}
        onInputStarted={vi.fn()}
        onSubmitMessage={vi.fn()}
      />,
    );

    expect(html).toContain('data-cloud-part="tail-large"');
    expect(html).toContain('data-cloud-part="tail-small"');
  });

  it("sizes the cloud bubble to the formatted text length", () => {
    const shortHtml = renderToStaticMarkup(
      <Bubble
        bubble={openBubble({ text: "嗯。" })}
        onInputEnded={vi.fn()}
        onInputStarted={vi.fn()}
        onSubmitMessage={vi.fn()}
      />,
    );
    const longHtml = renderToStaticMarkup(
      <Bubble
        bubble={openBubble({
          text: "我在这里陪你，不用急，慢慢来，先把手上的事情做好就可以。",
        })}
        onInputEnded={vi.fn()}
        onInputStarted={vi.fn()}
        onSubmitMessage={vi.fn()}
      />,
    );
    const css = readFileSync("src/renderer/ianStage.css", "utf8");

    expect(shortHtml).toContain('data-cloud-size="short"');
    expect(longHtml).toContain('data-cloud-size="long"');
    expect(css).toContain('.ian-bubble[data-cloud-size="short"]');
    expect(css).toContain('width: max-content;');
    expect(css).not.toContain('width: 178px;');
  });
});

function openBubble(
  overrides: Partial<IanViewState["bubble"]> = {},
): IanViewState["bubble"] {
  return {
    isOpen: true,
    text: "我在这儿。",
    mood: "calm",
    visibleUntil: null,
    ...overrides,
  };
}
