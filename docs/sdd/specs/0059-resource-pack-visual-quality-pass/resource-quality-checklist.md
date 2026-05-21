# Resource Pack Quality Checklist

## Required Animations

- `idle`
- `walk`
- `run`
- `happy`
- `rest`
- `sleep`

## Visual Checks

- Every required animation is listed in `pet.json` capabilities.
- Every required animation has at least one frame in `animations.json`.
- Animation frames stay inside the declared sprite sheet width.
- `rest` and `sleep` do not reuse the exact `idle` frame set.
- Sprite sheet frames share the same frame width, frame height, scale, anchor, and transparent boundary.
- Light and dark backgrounds keep Ian readable without adding product UI chrome.

## Fallback Checks

- Unknown animation names resolve to `idle`.
- Missing optional expressions resolve to `idle` expression or a null overlay.
- Resource validation reports manifest field errors with concrete field names.
