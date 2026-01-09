# cortenforge (umbrella): Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| sim_core | re-export | Runtime scaffolding crate |
| vision_core | re-export | Vision interfaces/overlay math |
| vision_runtime | re-export | Capture/inference plugins |
| data_contracts | re-export | Schemas/validation |
| capture_utils | re-export | Recorder sinks/helpers |
| models | re-export | Model definitions (TinyDet/BigDet) |
| inference | re-export | Detector factory |
| training | re-export | Training/eval pipeline |
| burn_dataset | re-export | Burn dataset loader |
| cli_support | re-export | Shared CLI helpers |

## Links
- Source: `crates/cortenforge/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge
