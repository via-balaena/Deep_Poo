# Error Model (vision_core)
Quick read: How errors are surfaced and handled.

## Errors defined
- None directly; error handling is delegated to trait methods’ return types:
  - `Detector::detect` returns a `DetectionResult` (not fallible).
  - `Recorder::record` returns `std::io::Result<()>`.
  - `BurnDetectorFactory::load` returns `anyhow::Result<Self::Detector>`.

## Patterns
- Detectors are expected to be infallible at the API surface; recoverable errors should be embedded in the `DetectionResult` (e.g., fallback flags) or logged internally.
- Recorders surface IO errors directly via `std::io::Error`.
- Burn factory uses `anyhow::Result` to aggregate load/config errors without defining a crate-specific error type.

## Recoverability
- Recording failures are caller-visible (`io::Error`); runtime should decide whether to retry/skip.
- Burn detector loading is caller-visible; consumers should fall back (as `inference` and `vision_runtime` do) when load fails.

## Ergonomics
- Keeping `DetectionResult` infallible simplifies detector implementations but shifts error signaling to logging/fallback modes.
- `anyhow` in factory allows rich context; consider a typed error if multiple backends are added and need programmatic handling.

## Links
- Source: `vision_core/src/interfaces.rs`
