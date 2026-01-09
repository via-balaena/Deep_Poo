# Error Model (cortenforge-tools)
Quick read: How errors are surfaced and handled.

## Errors defined
- No custom error types; relies on underlying crates:
  - Services: `Result<_, ServiceError>` (wraps `io` and `serde_json` errors).
  - Warehouse command builder: pure string building (no errors).
  - Overlay/recorder modules re-export fallible functions from `capture_utils` (io/anyhow).

## Patterns
- Listing runs/reading manifests: bubbles IO/JSON errors via `ServiceError`.
  - Some operations are best-effort (counting artifacts) and may ignore individual failures.
- Command builders are infallible.
- Recorder/overlay use upstream error models (IO/anyhow).

## Recoverability
- Service errors are caller-visible; bins should handle/log them and continue or exit.
- Most tooling binaries prefer to continue when partial data is missing (e.g., missing manifest).

## Ergonomics
- `ServiceError` is typed (`thiserror`), improving display vs. bare `io::Error`.
- Mixed best-effort behavior means some issues are silent; add logging in bins if stricter behavior is desired.
