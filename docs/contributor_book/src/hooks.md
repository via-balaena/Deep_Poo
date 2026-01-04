# Hooks / extension points

Where apps plug into the substrate without modifying core crates. Keep hooks narrow; wire them explicitly in your app.

## Sim hooks (controls/autopilot)
- `SimHooks` resource holds optional `ControlsHook` and `AutopilotHook` trait objects supplied by the app.
- Implement the traits in your app; register them during setup.
- Gate systems with `ModeSet`/`SimRunMode` (e.g., enable autopilot only in datagen).

## Recorder hooks
- Metadata: app provides `RecorderMetaProvider` (implements `RecorderMetadataProvider`) to supply run metadata (seed, scene id, etc.).
- World state: app updates `RecorderWorldState` (e.g., head_z, stop flag) each frame.
- Sinks: any `RecorderSink` can be inserted; default JSON sink comes from `capture_utils`. Apps may add/replace sinks (DB, custom format) but keep schema compatibility if ETL/training are reused.

<details>
<summary>Typical wiring</summary>

- Insert `RecorderMetaProvider` with your metadata.
- Add a system to write `RecorderWorldState`.
- (Optional) Insert custom sink(s) implementing `RecorderSink`.
</details>

## Vision / detector hooks
- Detectors implement `vision_core::Detector` (Burn-backed or heuristic). The `inference` crate provides an `InferenceFactory` to load checkpoints; apps can swap in their own factory/detector.
- `vision_runtime::InferencePlugin` expects a `DetectorHandle` + thresholds; apps insert these when entering inference mode.
- Capture target/readback are set up by `vision_runtime::CapturePlugin`; adjust target size or add observers if needed.

## Mode gating
- Use `SimRunMode` and `ModeSet` to include/exclude systems:
  - Common: runs in all modes.
  - SimDatagen: sim + datagen.
  - Inference: inference-only.
- Gate heavy systems (detectors, capture) to the appropriate sets to avoid unnecessary work.

## Extending safely
- Keep core crates domain-agnostic; put domain/world systems in your app crate.
- Favor explicit plugin wiring over global state; keep hooks small.
- When adding sinks or detectors, preserve interfaces so downstream tools (ETL/training) continue to work.

## Mini recipes
- Recorder meta/world:
  ```rust,ignore
  app.insert_resource(RecorderMetaProvider {
      provider: Box::new(MyMeta { seed, scene_id }),
  });
  app.add_systems(Update, my_world_state_updater.in_set(ModeSet::Common));
  // Optional: swap sink
  app.insert_resource(MySink::new(run_dir));
  ```

- Inference handle:
  ```rust,ignore
  let detector = my_factory.build(thresholds, weights_path);
  app.insert_resource(DetectorHandle { detector, kind: DetectorKind::Burn });
  app.insert_resource(InferenceThresholds { obj_thresh, iou_thresh });
  app.add_plugins(InferencePlugin);
  ```
