# overlay (cortenforge-tools)

## Responsibility
- Re-export overlay helpers from `vision_core` for drawing/normalizing bounding boxes.

## Key items
- `draw_rect`, `normalize_box` re-exported from `vision_core::overlay`.

## Invariants / Gotchas
- Thin re-export; follow `vision_core::overlay` semantics (box coords normalized 0..1, etc.).

## Cross-module deps
- Used by overlay-related bins (e.g., `overlay_labels`) to annotate captures.
