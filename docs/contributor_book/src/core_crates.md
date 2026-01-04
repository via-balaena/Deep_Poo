# Crate deep dives

What lives in each shared crate, how they interact, and where to extend. Aim: understand boundaries, features, current gaps, and have quickstart snippets.

## sim_core
- Purpose: Bevy scaffolding (app builder, mode sets, config), hooks for controls/autopilot, recorder types. Detector wiring is intentionally absent here.
- Key types:
  - `SimConfig` / `SimRunMode` / `ModeSet`.
  - `SimPlugin` (registers config + sets) and `SimRuntimePlugin` (runtime systems).
  - `SimHooks`, `ControlsHook`, `AutopilotHook` (app-supplied callbacks).
  - Recorder: `RecorderConfig`, `RecorderState`, `RecorderMotion`, `RecorderWorldState`, `RecorderMetaProvider`.
- Modules to know: `config`, `hooks`, `recorder`, `app` (builder), `modes`.
- Flow:
  1) `build_app` seeds Bevy with DefaultPlugins + Rapier + mode sets.
  2) Apps inject hooks/resources (controls, autopilot, recorder meta/world state).
  3) Recorder sink defaults to JSON (from capture_utils) but can be replaced.
- Extend: implement hooks in your app crate; add systems to the relevant sets; supply recorder meta/world updates and custom sinks.
- Quickstart:
```rust,ignore
let mut app = sim_core::build_app(SimConfig::default());
app.insert_resource(RecorderMetaProvider { provider: Box::new(MyMeta {}) });
app.add_plugins((SimPlugin, SimRuntimePlugin));
app.add_systems(Update, my_world_state.in_set(ModeSet::Common));
app.run();
  ```
- Does/doesn’t:
  - Does: mode gating, hooks for controls/autopilot, recorder scaffolding.
  - Doesn’t: detectors or domain/world logic (apps supply those).
- Tests/fixtures: NdArray-focused; validate recorder wiring and mode sets.
- Depends on: Bevy/Rapier; pairs with `capture_utils` for sinks; upstream Burn via `inference` only indirectly (detector wiring lives elsewhere).

## vision_core
- Purpose: detector/capture data model and overlay math; no Bevy dependency.
- Key types: `Frame`, `FrameRecord`, `DetectionResult`, `Label`, `Recorder`, `Detector`, `CaptureLimit`, `draw_rect`/overlay helpers.
- Extend: implement `Detector`/`Recorder`; reuse overlay helpers; keep this crate free of engine/runtime concerns.
- Modules to know: `capture` (frame/label types), `detector`, `overlay`, `recorder`.
- Quickstart:
```rust,ignore
use vision_core::{DetectionResult, Detector};
struct MyDet;
impl Detector for MyDet {
    fn detect(&mut self, frame: &Frame) -> DetectionResult { /* ... */ }
}
  ```
- Does/doesn’t:
  - Does: data model, detector traits, overlay math.
  - Doesn’t: Bevy/runtime concerns, capture/inference scheduling.
- Tests/fixtures: pure Rust tests; overlay math, serialization shapes.
- Depends on: serde; consumed by `vision_runtime`, `capture_utils`, `inference`.

## vision_runtime
- Purpose: Bevy plugins for capture and inference built on vision_core.
- Capture pipeline:
  - `CapturePlugin` sets up render target + GPU readback resources (`FrontCaptureTarget`, `FrontCaptureReadback`, camera tracking).
  - Systems capture frames → `FrameRecord` → recorder sink; gated by `SimRunMode`.
- Inference pipeline:
  - `InferencePlugin` holds `DetectorHandle` (Burn or heuristic), thresholds, overlay state.
  - Schedules detector tasks on captured frames; updates overlay UI state; gated to inference mode.
- Extend: swap detector handle creation (via `inference` factory), adjust capture target sizing, or add new observers/overlays.
- Modules to know: `capture_plugin`, `inference_plugin`, `resources` (targets/readback), `systems`.
- Does/doesn’t:
  - Does: capture/inference plugins, detector scheduling, overlay state.
  - Doesn’t: define detectors (delegates to inference/models), own domain logic.
- Tests/fixtures: smoke tests around capture/inference wiring; feature-gate GPU paths.
- Depends on: `vision_core` types; `inference` for detector handles; Bevy/WGPU; uses `capture_utils` sinks via recorder pipeline.
- Quickstart:
```rust,ignore
app.insert_resource(DetectorHandle::heuristic());
app.add_plugins((CapturePlugin, InferencePlugin));
```

## data_contracts
- Purpose: serde/validation schemas for captures, manifests, shards.
- Key schemas: run manifest (id, seed, camera, resize/letterbox, frame count, checksum), frame label (bbox_norm/bbox_px/class/metadata), warehouse shard/manifest.
- Extend: add fields as needed; keep validation strict (bbox ranges, required fields).
- Modules to know: `capture` (run/label schemas), `warehouse` (manifest/shard).
- Does/doesn’t:
  - Does: define and validate schemas; serialize/deserialize.
  - Doesn’t: own IO or recorder logic.
- Tests/fixtures: schema validation tests; keep fixtures small.
- Depends on: serde; consumed by capture/tools/ETL/training.
- Quickstart:
```rust,ignore
use data_contracts::capture::RunManifest;
let manifest: RunManifest = serde_json::from_str(json_str)?;
manifest.validate()?;
```

## capture_utils
- Purpose: recorder sinks and capture helpers.
- Contents: default `JsonRecorder` sink, overlay and prune helpers used by tools and tests.
- Extend: add sinks (e.g., custom DB writer) while preserving schema compatibility for ETL.
- Modules to know: `recorder` (sinks), `overlay`, `prune`.
- Does/doesn’t:
  - Does: default sink, overlay/prune helpers.
  - Doesn’t: define recorder meta/world state (comes from sim_core/app).
- Tests/fixtures: sink writes against sample labels; overlay/prune helpers.
- Depends on: `data_contracts` for schemas; used by `sim_core` recorder pipeline and tools.
- Quickstart:
```rust,ignore
use capture_utils::recorder::JsonRecorder;
let recorder = JsonRecorder::new(run_dir)?;
recorder.write_label(&frame_label)?;
```

## models
- Purpose: Burn model definitions/configs (TinyDet, BigDet).
- Extend: add model variants/configs; keep domain logic out.
- Modules to know: `tiny`, `big` (model configs/defs), shared layers/utils.
- Does/doesn’t:
  - Does: define model architectures/configs.
  - Doesn’t: own training loops beyond configs; domain-specific heads.
- Tests/fixtures: forward-shape smokes (NdArray).
- Depends on: Burn; consumed by `training` and `inference`.
- Quickstart:
```rust,ignore
use models::tiny::TinyDetConfig;
let cfg = TinyDetConfig::default();
let model = cfg.build::<burn::backend::ndarray::NdArrayBackend<f32>>();
```

## training
- Purpose: training/eval CLI and pipeline on top of warehouse manifests.
- Contents: dataset loader (manifest → tensors), collate, loss/matching, optimizer/checkpoint I/O; CLI (`train.rs`, `eval.rs`) drives `run_train`.
- Extend: new losses, augmentations, schedulers; keep CLI flags in sync.
- Modules to know: `data` (loader/collate), `loss`, `train`/`eval` pipelines, `cli` bins.
- Does/doesn’t:
  - Does: training/eval pipeline and CLI.
  - Doesn’t: define model architectures (uses models crate).
- Tests/fixtures: NdArray smokes; keep datasets synthetic/small.
- Depends on: `models` for architectures; `data_contracts` manifests; Burn backend.
- Quickstart:
```bash
cargo run -p cortenforge-training --bin train -- --manifest path/to/manifest.json --output checkpoints/run1
```

## inference
- Purpose: detector factory that loads checkpoints (via `models`) and returns a `Detector` (Burn-backed) or a heuristic fallback when weights are absent.
- Extend: add backends or selection logic; keep the interface consistent for runtime/tools.
- Modules to know: `factory`, `heuristic`, backend-specific loaders.
- Does/doesn’t:
  - Does: load checkpoints, return detector handle.
  - Doesn’t: own capture or training.
- Tests/fixtures: heuristic fallback smokes; feature-gate Burn-backed paths.
- Depends on: `models` (checkpoint shapes), Burn; used by `vision_runtime` and tools.
- Quickstart:
```rust,ignore
use inference::factory::InferenceFactory;
let factory = InferenceFactory::default();
let detector = factory.load_from_checkpoint("checkpoints/run1").unwrap_or_else(|| factory.heuristic());
```

## colon_sim_tools
- Purpose: CLI utilities: overlay/prune, warehouse_etl/export/cmd, single_infer, tui, datagen_scheduler, helper binaries.
- Feature flags: `tui`, `scheduler`, `gpu_nvidia` gate heavier deps.
- Extend: add bins under `tools/src/bin/`; reuse shared parsers/services to avoid duplication. (Plan: split app-specific pieces into app repos; keep shared utilities here.)
- Modules to know: `services` (shared CLI helpers), `warehouse_commands`, bins under `src/bin/`.
- Does/doesn’t:
  - Does: ship shared tooling bins and services.
  - Doesn’t: include app-specific tooling once split (keep those in app repos).
- Tests/fixtures: CLI smokes in `tools/tests/`; prefer synthetic fixtures.
- Depends on: `data_contracts`, `capture_utils`, `vision_core`, `inference`/`training` for certain commands; feature flags gate heavy deps.

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
- Note burn-core dependency: currently patched to vendored 0.14.0; drop when upstream publishes a fixed release.

## Current limitations / upcoming work
- Burn dependency: burn-core 0.14.0 is patched locally; unblock publishes once upstream releases a fixed version.
- colon_sim_tools: contains app-specific bins; plan to split/trim so only shared tooling remains here.
- GPU paths: WGPU/GPU smokes are opt-in; default pipeline is NdArray-only to keep CI fast.
- Models: TinyDet/BigDet only; add guidance/checkpoints for new variants as they appear.
