# Migration Notes (Refactor #1)

This summarizes the recent refactor for contributors:

- Root crate (`colon_sim`) is orchestration/CLI only: `src/cli/*`, `run_app`, no domain systems.
- App crates live under `apps/`: `apps/colon_sim` (reference world) and `apps/hello_substrate` (minimal demo). App-specific bins (`sim_view`, `inference_view`) now live under their app crate.
- Core crates are domain-agnostic: `sim_core` (Bevy plumbing/hooks), `vision_core` (interfaces/overlay), `vision_runtime` (capture/inference plugins), `models` (TinyDet/BigDet), `data_contracts` (schemas), `capture_utils` (recorder/prune/overlay).
- Tools moved to the `colon_sim_tools` crate; CLI bins import shared helpers via `colon_sim::cli` and `colon_sim_tools::services`.
- Recorder lives in the substrate with a default `JsonRecorder` sink; apps provide recorder metadata/world state and can inject custom sinks.
- Tests and docs updated; `cargo test --workspace --all-features` and `cargo check --workspace` are green.
