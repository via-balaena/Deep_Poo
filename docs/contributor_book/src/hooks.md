# Hooks / extension points

Where apps plug into the substrate without modifying core crates.

## Sim hooks (controls/autopilot)
- `SimHooks` resource holds optional `ControlsHook` and `AutopilotHook` trait objects supplied by the app.
- Apps implement these traits to register control/autopilot systems and insert them during setup (e.g., in `run_app`).
- Use `ModeSet`/`SimRunMode` to gate your systems (e.g., enable autopilot only in datagen).

## Recorder hooks
- Metadata: app provides `RecorderMetaProvider` (implements `RecorderMetadataProvider`) to supply run metadata (seed, scene id, etc.).
- World state: app updates `RecorderWorldState` (e.g., head_z, stop flag) each frame.
- Sinks: any `RecorderSink` can be inserted; default JSON sink comes from `capture_utils`. Apps may add/replace sinks (DB, custom format) but should maintain schema compatibility if ETL/training are reused.
- Typical app wiring:
  - Insert `RecorderMetaProvider` with your metadata.
  - Add a system to write `RecorderWorldState`.
  - (Optional) Insert custom sink(s) implementing `RecorderSink`.

## Vision / detector hooks
- Detectors implement `vision_core::Detector` (Burn-backed or heuristic). The `inference` crate provides `InferenceFactory` to load checkpoints; apps can swap in their own factory or detector.
- `vision_runtime::InferencePlugin` expects a `DetectorHandle` resource; apps insert it (and thresholds) when entering inference mode.
- Capture target/readback are set up by `vision_runtime::CapturePlugin`; you can adjust target size or add observers if needed.

## Mode gating
- Use `SimRunMode` and `ModeSet` to include/exclude systems:
  - Common: runs in all modes.
  - SimDatagen: sim + datagen.
  - Inference: inference-only.
- Gate heavy systems (detectors, capture) to the appropriate sets to avoid unnecessary work.

## Extending safely
- Keep core crates domain-agnostic; put domain types/systems in your app crate.
- Favor narrow hooks and explicit plugin wiring over global state.
- When adding sinks or detectors, preserve interfaces so downstream tools (ETL/training) continue to work.

## Mini recipes
- Recorder meta/world:
  ```rust
  app.insert_resource(RecorderMetaProvider {
      provider: Box::new(MyMeta { seed, scene_id }),
  });
  app.add_systems(Update, my_world_state_updater.in_set(ModeSet::Common));
  ```
  Then optionally: `app.insert_resource(MySink::new(run_dir));`

- Inference handle:
  ```rust
  let detector = my_factory.build(thresholds, weights_path);
  app.insert_resource(DetectorHandle { detector, kind: DetectorKind::Burn });
  app.insert_resource(InferenceThresholds { obj_thresh, iou_thresh });
  app.add_plugins(InferencePlugin);
  ```
