# Performance Notes (cli_support)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- None; crate is configuration/data only.

## Allocation patterns
- Minimal: allocates paths/strings when parsing CLI args or constructing opts.

## Trait objects
- None.

## Assumptions
- Overhead is negligible; performance driven by consumer binaries.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
