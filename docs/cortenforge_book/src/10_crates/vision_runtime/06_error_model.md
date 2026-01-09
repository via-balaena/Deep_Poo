# Error Model (vision_runtime)
Quick read: How errors are surfaced and handled.

## Errors defined
- None; runtime uses best-effort fallbacks and logs rather than returning `Result` from systems.

## Patterns
- Inference scheduling is fallible only through the detector implementation; failures are expected to be handled inside `vision_core::Detector` impls (e.g., return heuristic/fallback).
- Capture readback and Bevy systems are written to early-return on missing state; no explicit errors surfaced.
- Overlay/debounce state uses option types (`Option<Task<_>>`, `Option<BurnDetectionResult>`) to represent absence/in-progress work.

## Recoverability
- Missing detectors or failed loads should be represented by swapping to `DetectorKind::Heuristic` (as in `inference`); runtime itself won’t panic.
- Readback mismatch silently ignores events not matching the target.

## Ergonomics
- System-level code avoids panics but also avoids surfacing structured errors. Operational issues are signaled via state (`fallback` message) or logs.
- If richer observability is needed (e.g., metrics/alerts), add events/resources to record failure counts instead of panicking.

## Links
- Source: `vision_runtime/src/lib.rs`
