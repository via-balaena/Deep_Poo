# Design Review (vision_core)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Clear interfaces for `FrameSource`, `Detector`, `Recorder`, with lightweight data structs.
- Trait-object-friendly design keeps embedding easy (Bevy plugins, CLI tools).
- `BurnDetectorFactory` provides an optional hook without forcing Burn dependency on consumers.

## Risks / gaps
- Interfaces lack `Send + Sync`; downstream crates must remember to add bounds when crossing threads.
- `DetectionResult`/`Frame` have fully owned vectors; no buffer reuse guidance—may encourage alloc churn in high-throughput scenarios.
- Error signaling is implicit (detectors cannot return failures), so runtime must invent its own fallback channels.

## Refactor ideas
- Consider adding optional `Send + Sync` marker traits or aliases for threaded contexts to reduce mistakes.
- Provide a “borrowed”/pooled variant or guidance for reusing buffers in hot paths.
- Offer an optional error-aware detector trait (or return `Result`) for cases where distinguishing failure vs. negative detection matters.

## Links
- Source: `vision_core/src/interfaces.rs`
