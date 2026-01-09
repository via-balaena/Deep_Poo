# cortenforge (umbrella): Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Single-module facade; re-exports member crates: sim_core, vision_core, vision_runtime, data_contracts, capture_utils, models, inference, training, burn_dataset, cli_support.
- No additional submodules; feature wiring aligns with member crates.

Cross-module dependencies:
- none internally.
- this crate is purely a facade delegating to underlying crates.

## Links
- Source: `crates/cortenforge/src/lib.rs`
