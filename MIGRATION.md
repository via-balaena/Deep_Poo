# Migration Notes (Refactor #1)

This summarizes the recent refactor for contributors:

- The `colon_sim` reference app (and other demos) now live in their own repository; this workspace is library-only.
- Core crates remain domain-agnostic: `sim_core` (Bevy plumbing/hooks), `vision_core` (interfaces/overlay), `vision_runtime` (capture/inference plugins), `models` (TinyDet/BigDet), `data_contracts` (schemas), `capture_utils` (recorder/prune/overlay).
- Tools stay in `cortenforge-tools`; use them from the app repo to run headless capture/ETL/inference.
- Recorder lives in the substrate with a default `JsonRecorder` sink; apps provide recorder metadata/world state and can inject custom sinks.
- Tests and docs updated for the crates; `cargo test --workspace --all-features` and `cargo check --workspace` are green.
