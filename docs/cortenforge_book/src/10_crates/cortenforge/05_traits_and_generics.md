# Traits & Generics (cortenforge)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None; this crate only re-exports others behind feature flags.

## Generics and bounds
- No generics; all items are `pub use` of other crates gated by features (`sim-core`, `vision-core`, etc.).

## Design notes
- Keep feature names aligned with member crate names; adding/removing crates requires updating the feature list and re-exports.
- This crate intentionally exposes no additional APIs or traits; consumers should depend on member crates directly if they need fine-grained control.

## Links
- Source: `crates/cortenforge/src/lib.rs`
