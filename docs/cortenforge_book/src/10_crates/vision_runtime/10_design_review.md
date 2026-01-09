# Design Review (vision_runtime)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Clear separation of capture vs. inference systems; resources encapsulate state cleanly.
- Async inference via Bevy task pool with explicit debounce avoids hammering the model.
- Trait-object detectors make it easy to swap heuristic/Burn without changing systems.

## Risks / gaps
- Single pending inference and mutex-guarded detector mean throughput is limited; scaling to higher FPS would need redesign.
- Readback/inference allocations aren’t pooled; could churn in long runs.
- Error/fallback signaling is implicit (overlay fallback string); lacks structured telemetry or counters.

## Refactor ideas
- Add buffer pooling for readback bytes and detection vectors.
- Support multiple in-flight inference tasks or a small queue; allow batch processing for throughput.
- Expose metrics/events for failures, fallback usage, inference latency distributions.

## Links
- Source: `vision_runtime/src/lib.rs`
