# Performance Notes (vision_core)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- None; this crate defines interfaces and lightweight data structs. No compute-heavy logic.

## Allocation patterns
- `DetectionResult` and `Frame` own vectors (`boxes`, `scores`, optional `rgba`), allocated by implementers.
- No caching or pooling implemented here.

## Trait objects
- Traits are object-safe; dynamic dispatch cost is minimal. Implementers decide how to manage allocations/caching.

## Assumptions
- Performance is dominated by detector/recorder implementations in downstream crates; vision_core adds negligible overhead.

## Improvements
- If allocations in `DetectionResult` become hot, implementers can reuse buffers; interfaces permit that (caller-owned structs).

## Links
- Source: `vision_core/src/interfaces.rs`
