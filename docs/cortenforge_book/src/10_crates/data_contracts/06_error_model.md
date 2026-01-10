# Error Model (data_contracts)
Quick read: How errors are surfaced and handled.

## Errors defined
- `ValidationError` (enum): invalid bbox order/range (`InvalidBboxPx`, `InvalidBboxNorm`), `MissingImage`.
- `RunManifest::validate` returns `Result<(), String>` for timestamp/max_frames sanity.

## Patterns
- Validation errors are sync and deterministic; no IO inside validation.
- Serde (de)serializes structs; serde errors are not wrapped here (handled by callers).

## Recoverability
- Validation errors are recoverable: callers can log/skip/repair input. No panics or fatal errors.

## Ergonomics
- `ValidationError` uses `thiserror` for display strings; suitable for user-facing logs.
- Manifest validation returning `String` is less typed; consider moving to a typed error if extended.

## Links
- Source: `crates/data_contracts/src/capture.rs`
- Source: `crates/data_contracts/src/manifest.rs`
