# factory (inference)

## Responsibility
- Build a `Detector` implementation backed by Burn weights when available, or fall back to a heuristic detector.
- Convert `vision_core::interfaces::Frame` data into Burn `TensorData` for TinyDet-style models.
- Manage model loading (`BinFileRecorder`, `InferenceModel`, `InferenceBackend`) and thread-safe access via `Arc<Mutex<_>>`.

## Key items
- `InferenceThresholds`: objectness + IoU thresholds (defaults: 0.3 / 0.5).
- `HeuristicDetector`: placeholder detector; always returns a result using the threshold as confidence.
- `BurnTinyDetDetector`: wraps `InferenceModel` with mutex; forwards frames through the model.
- `InferenceFactory::build`: chooses Burn-backed detector if weights are present and loadable; otherwise uses heuristic.
- `InferenceFactory::try_load_burn_detector`: loads checkpoint via `BinFileRecorder<FullPrecisionSettings>` and returns a boxed `Detector`.

## Invariants / Gotchas
- If `weights` is `None` or the path does not exist, the factory always returns the heuristic detector (emits an `eprintln!`).
- Model access is mutex-guarded; any poisoned mutex will panic on lock.
- `frame_to_tensor` currently encodes only RGB mean and aspect ratio; no real image tensor pipeline yet.
- IoU threshold is stored but unused in the current detector implementation.
- Errors during checkpoint load are swallowed with a fallback to heuristic; callers should log/alert if a real model is required.

## Cross-module deps
- Implements `vision_core::interfaces::Detector` for both heuristic and Burn-backed detectors.
- Consumed by higher-level orchestration (e.g., vision_runtime pipelines or CLI) to obtain a detector instance.
- Uses `InferenceBackend`/`InferenceModel` defined in the `inference` crate; cooperates with `vision_core::interfaces::Frame` / `DetectionResult`.
