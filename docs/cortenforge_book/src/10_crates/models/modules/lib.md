# lib (models)

## Responsibility
Define Burn-based model architectures/configs for TinyDet and BigDet, plus a prelude for re-exports.

## Key types
- `TinyDetConfig` / `TinyDet<B>`: Small detector with two linear layers; configurable hidden size.
- `BigDetConfig` / `BigDet<B>`: Larger detector with configurable hidden size/depth/max_boxes/input_dim; multibox heads.
- `prelude`: Re-exports model types for consumers.

## Important functions
- `TinyDet::new`, `TinyDet::forward`: Build and run forward pass.
- `BigDet::new`: Build with stem/blocks/heads; clamps multibox output to valid ranges.
- `BigDet::forward`: Forward to scores.
- `BigDet::forward_multibox`: Returns boxes/scores with sigmoid and ordering/clamp to [0,1].

## Invariants / Gotchas
- BigDet clamps/reorders boxes to enforce x0<=x1, y0<=y1 in [0,1]; useful for stable outputs.
- max_boxes is clamped to at least 1.
- input_dim defaults to 4 if not provided.

## Cross-module deps
- Pure model definitions; consumed by training/inference crates.
