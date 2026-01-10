# Design Review (models)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Minimal configs and implementations; easy to read and extend.
- Backend-generic design keeps CPU/GPU options open without changing APIs.
- `forward_multibox` enforces box ordering/clamping, improving downstream robustness.

## Risks / gaps
- Models are very small; suitable as placeholders but may underfit real tasks.
- No serialization/loading helpers here; relies on external tooling to persist checkpoints.
- `max_boxes` is fixed at construction; dynamic box counts require padding/truncation.

## Refactor ideas
- Add simple load/save helpers (aligned with Burn recorders) to reduce boilerplate in consumers.
- Consider exposing configurable activations/heads for experimentation.
- Add basic tests/benchmarks to track perf/regressions as models evolve.

## Links
- Source: `crates/models/src/lib.rs`
