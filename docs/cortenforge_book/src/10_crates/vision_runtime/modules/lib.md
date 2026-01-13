# vision_runtime (lib): Deep Dive

## Responsibility
Implements Bevy systems/plugins for capture and inference built on vision_core. Manages capture render targets/readback, detector scheduling, thresholds, overlay state, and integrates with sim_core run modes.

## Key structs/resources
- Capture state: `PrimaryCameraFrame`, `PrimaryCameraState`, `PrimaryCameraFrameBuffer`, `PrimaryCaptureTarget`, `PrimaryCaptureReadback`.
- Inference state: `DetectorHandle` (boxed Detector + kind), `InferenceThresholdsResource`, `ModelLoadState` (flag), `AsyncInferenceState` (pending task, last result, debounce timer), `DetectionOverlayState` (boxes/scores/size/fallback/inference_ms).
- Detector types: `DetectorKind` (Burn/Heuristic), `RuntimeDetectionResult`.
- Plugins: `CapturePlugin`, `InferencePlugin`.

## Important systems/functions
- Capture setup/readback:
  - `setup_front_capture`: create render target, spawn capture camera, set target resources.
  - `track_front_camera_state`: track transform/frame counter/time into buffers.
  - `capture_front_camera_frame`: issue readback requests in Datagen/Inference modes.
  - `on_front_capture_readback`: store latest readback bytes.
- Inference pipeline:
  - `schedule_burn_inference`: debounce, pop latest frame, spawn async detector task.
  - `poll_inference_task`: poll async task, update detector handle, overlay state, last result.
  - `threshold_hotkeys`: adjust thresholds via keyboard; allow switching to heuristic detector.
- Helpers:
  - `recorder_draw_rect`: wrapper to draw overlays.
  - `HeuristicDetector`: simple detector used as fallback.

## Invariants / Gotchas
- Mode gating: capture only in Datagen/Inference; inference systems run in ModeSet::Inference.
- Debounce timer prevents overlapping inference tasks; uses AsyncComputeTaskPool.
- Capture target must be initialized once; guard against re-init by checking size.
- DetectorHandle must be inserted externally (e.g., from inference factory); otherwise inference skips.
- Readback data may be None/missing; systems gracefully skip.

## Cross-module deps
- Depends on vision_core (capture interfaces/overlay helpers), sim_core (SimRunMode/ModeSet), inference crate for detectors (provided via DetectorHandle), and Bevy render/task APIs.

## Suggested usage
- Apps insert DetectorHandle/InferenceThresholds, add CapturePlugin + InferencePlugin, and run under sim_core-built App. Ensure ModeSet sets are configured.
