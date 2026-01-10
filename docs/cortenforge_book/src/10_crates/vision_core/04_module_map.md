# vision_core: Module Map
Quick read: What each module owns and why it exists.

- `capture`: Capture-related resources/types (CaptureLimit, FrontCamera markers, FrontCaptureTarget/Readback).
- `interfaces`: Core vision types and traits.
  - Types: Frame, FrameRecord, DetectionResult, Label.
  - Traits: Detector, Recorder, FrameSource, BurnDetectorFactory.
- `overlay`: Overlay helpers (normalize_box, draw_rect) and related utilities.
- `prelude`: Convenience re-exports for downstream users.

Cross-module dependencies:
- interfaces define the shared contracts.
- capture provides resources used by runtime.
- overlay operates on data from interfaces.
- prelude re-exports common items.

## Links
- Source: `crates/vision_core/src/lib.rs`
