# Error Model (capture_utils)
Quick read: How errors are surfaced and handled.

## Errors defined
- Uses existing error types:
  - Implements `vision_core::Recorder::record` returning `std::io::Result<()>`.
  - `generate_overlays` returns `anyhow::Result<()>`.
  - `prune_run` returns `std::io::Result<(usize, usize)>`.
- Internal validation relies on `data_contracts::ValidationError` (converted to `io::Error` in recorder).

## Patterns
- Recorder maps validation failure to `io::Error::other("validation failed: …")`.
- Overlay generation skips invalid/missing entries silently (continues on errors).
- Pruning propagates IO errors; copies files best-effort.

## Recoverability
- Recorder errors are caller-visible; should be handled/retried by runtime.
- Overlay generation best-effort; failures on individual files don’t abort the run (may miss overlays).
- Prune errors bubble up; callers can report and stop.

## Ergonomics
- Mixed `anyhow` and `io::Error`; fine for tooling but consider consistent typed errors if this crate is reused in libraries.
- Silent skips in `generate_overlays` are convenient but may hide data issues; add logging if stricter guarantees are needed.

## Links
- Source: `capture_utils/src/lib.rs`
