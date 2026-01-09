# Error Model (cortenforge)
Quick read: How errors are surfaced and handled.

## Errors defined
- None; crate only re-exports other crates behind features.

## Patterns
- Error behavior is inherited from the underlying crates; this crate does not wrap or alter errors.

## Recoverability / Ergonomics
- Consumers should consult member crates’ error models. Using this facade does not change error surfaces.

## Links
- Source: `crates/cortenforge/src/lib.rs`
