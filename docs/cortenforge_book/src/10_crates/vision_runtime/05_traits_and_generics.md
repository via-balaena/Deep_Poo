# Traits & Generics (vision_runtime)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None defined here; runtime composes Bevy systems/resources and consumes interfaces from `vision_core`.

## Glue types / resources
- `DetectorHandle`: boxed `vision_core::Detector` trait object + `DetectorKind` enum to track backend.
- `BurnInferenceState`: holds async inference task (`Task<InferenceJobResult>`) and last result.
- `InferenceThresholds`: simple thresholds resource; mutable via hotkeys.
- `BurnDetector`, `DetectionOverlayState`, `FrontCameraState`, `FrontCameraFrameBuffer`, `FrontCaptureTarget`, `FrontCaptureReadback`: resources to coordinate capture/inference.

## Generics and bounds
- Uses trait objects (`Box<dyn Detector + Send + Sync>`) to allow swapping heuristic/Burn detectors at runtime.
- Async job returns tuple with boxed detector to restore ownership; leverages `Task<InferenceJobResult>` without exposing lifetimes.
- No user-facing generics; types are concrete Bevy resources.

## Design notes
- Trait-object approach matches Bevy resource storage and allows runtime replacement of detectors (hotkeys, load/fallback).
- No custom traits; extension is via providing different `Detector` implementations to `DetectorHandle`.
- Future extensibility: could generalize inference scheduling via a trait, but current code is purposely concrete to keep Bevy wiring simple.

## Links
- Source: `crates/vision_runtime/src/lib.rs`
