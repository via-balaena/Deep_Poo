# cli_support: Module Map
Quick read: What each module owns and why it exists.

- `common`: Shared CLI option structs (thresholds, weights, capture/warehouse outputs, WgpuEnvHints).
- `seed`: SeedState helper and resolve_seed function.
- `lib.rs`: Re-exports common/seed modules.

Cross-module dependencies:
- used by tooling/apps for consistent CLI parsing.
- optional Bevy resource features extend usage.

## Links
- Source: `crates/cli_support/src/lib.rs`
