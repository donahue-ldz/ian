# Privacy Source Denylist Scan v2

Current decision: high-sensitive sources remain denied by default and outside P0.

Denylist scan terms:

- clipboard
- screen OCR
- code body
- code diffs
- terminal full output
- private chat content
- arbitrary file content

Accepted current state:

- these sources are not part of the default local-first creature loop.
- adapter skeletons must pass permission and sanitizer checks before any IanEvent can affect behavior.
- unknown sources are denied.
- future capability docs must continue to state default-off or not product-ready.

