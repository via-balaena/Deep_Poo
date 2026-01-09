# interfaces (vision_core)

## Responsibility
Core vision data model and traits for detectors/recorders/frame sources.

## Key types/traits
- `Frame`: Captured frame metadata (id, timestamp, rgba optional, size, path).
- `FrameRecord<'a>`: Frame plus labels and camera state for recorder sinks.
- `Label`: Polyp label metadata (center_world, bbox_px/norm optional).
- `DetectionResult`: Detector output (positive, confidence, boxes, scores).
- Traits:
  - `FrameSource`: produce frames.
  - `Detector`: consume Frame → DetectionResult; optional `set_thresholds`.
  - `Recorder`: persist FrameRecord.
  - `BurnDetectorFactory`: load Burn-backed detectors.

## Invariants / Gotchas
- `Frame` may have rgba or path; consumers handle whichever is present.
- `DetectionResult` boxes/scores aligned by index; ensure lengths match.

## Cross-module deps
- Used by vision_runtime (plugins), capture_utils (recorder sinks), inference (detectors) and tools.
