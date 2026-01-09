# overlay (vision_core)

## Responsibility
Helpers for normalizing bounding boxes and drawing overlays onto images.

## Key functions
- `normalize_box(bbox_norm, dims) -> Option<[u32;4]>`: Convert 0..1 bbox coords to pixel coords, clamped to image bounds.
- `draw_rect(img, bbox_px, color, thickness)`: Draw rectangle border on an RGBA image.

## Invariants / Gotchas
- `normalize_box` returns None if bbox is out of bounds or inverted; callers should check.
- `draw_rect` clamps per thickness; skips if out of bounds.

## Cross-module deps
- Used by tools/capture_utils for overlays; complements detector outputs from interfaces.
