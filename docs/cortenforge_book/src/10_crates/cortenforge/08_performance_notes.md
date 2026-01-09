# Performance Notes (cortenforge)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- None; facade crate only re-exports members.

## Allocation patterns
- None beyond what member crates perform.

## Trait objects
- None added; uses member crates’ APIs directly.

## Assumptions
- No performance impact; use member crates directly for fine-grained control.

## Links
- Source: `crates/cortenforge/src/lib.rs`
