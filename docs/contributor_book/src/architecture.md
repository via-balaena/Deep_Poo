# Architecture: substrate + apps

How CortenForge is structured, how data and control flow through it, and where apps attach.

## Big picture
- **Substrate crates**: `sim_core` (Bevy scaffolding, mode sets, hooks, recorder types), `vision_core` (detector interfaces + overlay math), `vision_runtime` (capture/inference plugins), `data_contracts` (schemas), `capture_utils` (sinks/overlay/prune), `models` (TinyDet/BigDet), `training`, `inference`, `colon_sim_tools`.
- **Apps**: `apps/colon_sim` (reference world) and `apps/hello_substrate` (minimal demo). Apps supply domain systems; substrate stays detector- and domain-agnostic.
- **Root crate**: glue only (`src/cli/*`, `run_app`), wires `SimConfig` + plugins + app hooks.

## Runtime flow (per run)
1. CLI parses `AppArgs` → choose `RunMode` (Sim/Datagen/Inference) → call `run_app`.
2. Build base Bevy app via `sim_core::build_app` + `SimPlugin` (mode sets/config) + `SimRuntimePlugin` (runtime systems).
3. Add `vision_runtime` plugins:
   - `CapturePlugin` (image target/readback setup, camera tracking, capture loop).
   - `InferencePlugin` (optional; schedules detector on captured frames).
4. Add app plugins/systems (controls/autopilot/HUD/world, recorder world state).
5. Recorder (in `sim_core`) runs with default `JsonRecorder`; app provides metadata/world state and can inject sinks.
6. Run Bevy app; mode sets gate systems (Common/SimDatagen/Inference).

## Data flow (happy path)
- **Capture** (`sim_view` interactive or `datagen` headless) → writes `run_<ts>` dirs under `assets/datasets/captures/` with frames, labels, overlays (optional), manifest.
- **Prune/overlay** (tools via `capture_utils`) → optional `captures_filtered/` and overlays.
- **ETL** (`warehouse_etl`) → tensor warehouse shards + manifest under `artifacts/tensor_warehouse/v<ts>/`.
- **Training** consumes warehouse manifest → checkpoints in `checkpoints/` (TinyDet/BigDet).
- **Inference** (live via `inference_view` or single-image) loads checkpoint; falls back to heuristic detector if none provided.

## Call graph (high level)
- `run_app` → `sim_core::build_app` → add `SimPlugin`/`SimRuntimePlugin` → add `vision_runtime` capture (+ inference when requested) → add app plugins (`AppSystemsPlugin`, `AppBootstrapPlugin`, etc.) → `App::run`.
- Recorder pipeline: app injects `RecorderMetaProvider` + `RecorderWorldState` updates; substrate installs default sink; frames/labels flow to disk; tools/ETL read manifests.

## Design principles
- Keep substrate detector-free and domain-agnostic; apps own world/systems.
- Small, explicit surfaces: SimHooks, recorder meta/world state, vision hooks; avoid sprawling abstraction layers.
- Defaults first; feature-gate heavy deps; NdArray backend for tests.
- Make wiring obvious (plugins, sets, resources), favor clarity over indirection.

## Repository map (at a glance)
```
src/              # glue: cli, run_app (orchestration only)
apps/colon_sim/   # reference app (world, HUD, autopilot, bins)
apps/hello_substrate/ # minimal demo app
sim_core/         # Bevy plumbing + hooks + recorder types
vision_core/      # detector interfaces + overlay math
vision_runtime/   # capture/inference plugins for Bevy
data_contracts/   # schemas + validation
capture_utils/    # recorder sinks + overlay/prune helpers
models/           # TinyDet/BigDet definitions
training/         # training CLI + dataset/loss
inference/        # detector factory (Burn + heuristic)
tools/            # CLI tools (overlay/prune/etl/cmd/scheduler/tui)
docs/user_book/   # user-facing guide
docs/contributor_book/ # contributor guide
```
