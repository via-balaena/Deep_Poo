# Error Model (models)
Quick read: How errors are surfaced and handled.

## Errors defined
- None; model constructors/forwards are infallible and rely on Burn for panic/assert behavior.

## Patterns
- Construction uses Burn `init` APIs; panics would only arise from backend/device issues.
- Forward/multibox do not return `Result`; numeric issues (NaN, etc.) propagate through tensors.

## Recoverability
- No explicit error channels; callers must detect/handle invalid outputs (e.g., NaNs) at higher layers.

## Ergonomics
- Simplicity suits small demo models. If expanding to larger models or dynamic loading, consider returning `Result` from builders when IO/shape validation is involved.

## Links
- Source: `crates/models/src/lib.rs`
