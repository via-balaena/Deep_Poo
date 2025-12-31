# Dev Guide

For developers working on the data/training pipeline: setup, standards (fmt/clippy/tests), how to extend schemas/warehouse, and where to find key code paths. Update docs whenever interfaces change.

## Crate map (where code lives)
- Root (`colon_sim`): orchestration/CLI glue (`src/cli/*`, `run_app`); no domain systems.
- App (`apps/colon_sim`): reference world/entities, HUD, controls/autopilot hooks.
- Core: `sim_core` (Bevy plumbing), `vision_core`/`vision_runtime` (detector interfaces + capture/inference plugins), `models` (TinyDet/BigDet).
- Training/Inference: `training` (loop/CLI), `inference` (Burn-backed detector factory).
- Tools: `colon_sim_tools` (overlay/prune/warehouse/datagen/scheduler/tui) plus shared helpers.
- Build your own sim: add an app crate that registers domain systems/hooks; keep the root crate as glue only.
 - Bins: run specific bins (`sim_view`, `inference_view`, tools bins) via `cargo run --bin ...`; `main` is just a thin wrapper over `run_app`.
- Sample app: `apps/hello_substrate` shows a minimal plugin on the substrate (no colon-specific systems).
- Migration summary: see `MIGRATION.md` at repo root for the refactor overview.

## Recorder defaults & hooks
- Recorder runs in the substrate (`src/sim/recorder.rs`) and installs a default `JsonRecorder` sink (from `capture_utils`) when a run starts. You can inject your own sink via `RecorderSink.writer`.
- Apps provide metadata via `RecorderMetaProvider` (e.g., seed) and world state via `RecorderWorldState` (head_z/stop flag); update them in your app systems.
