# Traits & Generics (vision_core)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- `FrameSource`: pull next frame from camera/files/generator. Return `Option<Frame>`.
- `Detector`: run inference over a `Frame`, returning `DetectionResult`; optional `set_thresholds` hook.
- `Recorder`: persist `FrameRecord` (frame + labels + metadata) to a sink (disk/stream/etc.).
- `BurnDetectorFactory`: feature-flagged hook for constructing a Burn-backed `Detector` from a checkpoint path (`type Detector: Detector` associated type).

## Glue types
- Data structs (`Frame`, `DetectionResult`, `Label`, `FrameRecord`) define the shared contract between runtime, detectors, and recorders.
- No internal traits beyond the public interfaces above.

## Generics and bounds
- Traits are object-safe (no generic methods), enabling trait objects for detectors/recorders/frame sources.
- `BurnDetectorFactory` uses an associated type constrained by `Detector` so consumers can select concrete detector types while keeping API simple.
- No lifetime gymnastics; `FrameRecord` uses a label slice borrow to avoid copies in recorder implementations.

## Design notes
- Interface set is intentionally minimal to keep runtime embedding straightforward (Bevy plugins, CLI tools).
- Trait-object friendliness allows swapping implementations at runtime (heuristic vs. Burn, file vs. camera source, JSON vs. binary recorder).
- If adding async paths, consider separate async traits or adapters to keep current sync trait objects intact.

## Links
- Source: `crates/vision_core/src/interfaces.rs`
