# Error Model (inference)
Quick read: How errors are surfaced and handled.

## Errors defined
- Uses existing error channels; no custom error enums.
- Factory load path:
  - `InferenceFactory::try_load_burn_detector` returns `Option<Box<Detector>>`; errors (missing weights, load failures) are logged (`eprintln!`) and fall back to heuristic.
- Tests rely on detector.detect not panicking.

## Patterns
- Heuristic fallback on any checkpoint issue; model load errors are not propagated.
- Mutex poisoning on detector lock would panic; otherwise infallible detect path.

## Recoverability
- Fully recoverable: absence/bad weights yields heuristic detector with a log message.
- No structured error for callers to differentiate failure reasons.

## Ergonomics
- Silent (logged) fallback keeps CLI usable but hides error specifics; consider returning a typed error or result alongside the detector when model availability matters.

## Links
- Source: `crates/inference/src/factory.rs`
