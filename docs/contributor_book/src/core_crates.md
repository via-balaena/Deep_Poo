# Core crates (CortenForge substrate)

What lives in each shared crate, how they interact, and where to extend.

## sim_core
- Purpose: Bevy scaffolding (app builder, mode sets, config), hooks for controls/autopilot, recorder types. Detector wiring is intentionally absent here.
- Key types:
  - `SimConfig` / `SimRunMode` / `ModeSet`.
  - `SimPlugin` (registers config + sets) and `SimRuntimePlugin` (runtime systems).
  - `SimHooks`, `ControlsHook`, `AutopilotHook` (app-supplied callbacks).
  - Recorder: `RecorderConfig`, `RecorderState`, `RecorderMotion`, `RecorderWorldState`, `RecorderMetaProvider`.
- Flow:
  1) `build_app` seeds Bevy with DefaultPlugins + Rapier + mode sets.
  2) Apps inject hooks/resources (controls, autopilot, recorder meta/world state).
  3) Recorder sink defaults to JSON (from capture_utils) but can be replaced.
- Extend: implement hooks in your app crate; add systems to the relevant sets; supply recorder meta/world updates and custom sinks.

## vision_core
- Purpose: detector/capture data model and overlay math; no Bevy dependency.
- Key types: `Frame`, `FrameRecord`, `DetectionResult`, `Label`, `Recorder`, `Detector`, `CaptureLimit`, `draw_rect`/overlay helpers.
- Extend: implement `Detector`/`Recorder`; reuse overlay helpers; keep this crate free of engine/runtime concerns.

## vision_runtime
- Purpose: Bevy plugins for capture and inference built on vision_core.
- Capture pipeline:
  - `CapturePlugin` sets up render target + GPU readback resources (`FrontCaptureTarget`, `FrontCaptureReadback`, camera tracking).
  - Systems capture frames → `FrameRecord` → recorder sink; gated by `SimRunMode`.
- Inference pipeline:
  - `InferencePlugin` holds `DetectorHandle` (Burn or heuristic), thresholds, overlay state.
  - Schedules detector tasks on captured frames; updates overlay UI state; gated to inference mode.
- Extend: swap detector handle creation (via `inference` factory), adjust capture target sizing, or add new observers/overlays.

## data_contracts
- Purpose: serde/validation schemas for captures, manifests, shards.
- Key schemas: run manifest (id, seed, camera, resize/letterbox, frame count, checksum), frame label (bbox_norm/bbox_px/class/metadata), warehouse shard/manifest.
- Extend: add fields as needed; keep validation strict (bbox ranges, required fields).

## capture_utils
- Purpose: recorder sinks and capture helpers.
- Contents: default `JsonRecorder` sink, overlay and prune helpers used by tools and tests.
- Extend: add sinks (e.g., custom DB writer) while preserving schema compatibility for ETL.

## models
- Purpose: Burn model definitions/configs (TinyDet, BigDet).
- Extend: add model variants/configs; keep domain logic out.

## training
- Purpose: training/eval CLI and pipeline on top of warehouse manifests.
- Contents: dataset loader (manifest → tensors), collate, loss/matching, optimizer/checkpoint I/O; CLI (`train.rs`, `eval.rs`) drives `run_train`.
- Extend: new losses, augmentations, schedulers; keep CLI flags in sync.

## inference
- Purpose: detector factory that loads checkpoints (via `models`) and returns a `Detector` (Burn-backed) or a heuristic fallback when weights are absent.
- Extend: add backends or selection logic; keep the interface consistent for runtime/tools.

## colon_sim_tools
- Purpose: CLI utilities: overlay/prune, warehouse_etl/export/cmd, single_infer, tui, datagen_scheduler, helper binaries.
- Feature flags: `tui`, `scheduler`, `gpu_nvidia` gate heavier deps.
- Extend: add bins under `tools/src/bin/`; reuse shared parsers/services to avoid duplication.

## Cross-crate interactions (happy path)
1) sim_core builds the app and recorder scaffolding; app injects hooks + recorder meta/world state.
2) vision_runtime capture turns renders into `FrameRecord`s; capture_utils writes JSON labels/images to disk.
3) data_contracts defines the run/label schema; tools/ETL validate against it.
4) warehouse_etl (tools) + training consume manifests/shards; models define TinyDet/BigDet; inference factory loads checkpoints; vision_runtime runs detectors live.

## Notes for contributors
- Keep sim_core and vision_core domain-agnostic and detector-free; domain/world logic lives in apps.
- Prefer explicit plugin wiring and mode gating over hidden magic.
- Gate heavy deps behind features; default to NdArray where possible for tests.
- Preserve schema compatibility (data_contracts) so ETL/training/tools remain stable.
