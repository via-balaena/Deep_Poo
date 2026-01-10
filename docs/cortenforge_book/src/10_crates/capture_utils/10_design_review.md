# Design Review (capture_utils)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Simple, focused helpers for recording, overlay generation, and pruning.
- Recorder validates against shared contracts; avoids writing bad labels.
- Uses standard `image` crate and serde; easy to reason about.

## Risks / gaps
- Overlay generation and pruning are sequential and silent on per-file failures; large datasets may hide errors.
- Recorder supports only JSON output; no configurable formats or compression.
- No pooling/parallelism; could be slow at scale.

## Refactor ideas
- Add logging/metrics for skipped overlays and errors to improve visibility.
- Introduce optional parallel overlay/prune processing and buffer reuse for speed.
- Consider pluggable recorder formats (e.g., msgpack) to reduce size or improve speed while keeping contract compatibility.

## Links
- Source: `crates/capture_utils/src/lib.rs`
