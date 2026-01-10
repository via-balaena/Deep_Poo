# Performance Notes (vision_runtime)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Capture pipeline: GPU readback of the front camera each frame; limited by render/copy bandwidth.
- Inference scheduling: per-frame async task executing detector forward pass.
- Overlay updates: updating `DetectionOverlayState` and optionally drawing boxes (if recorder uses it).

## Allocation patterns
- Minimal per-frame allocations: clones of readback bytes and detection results; `DetectionOverlayState` reuses vectors but not explicitly preallocated.
- Detector task captures frame data (RGBA bytes) by move; could be reused via buffers if needed.

## Trait objects
- Detector stored as trait object; swapping detectors incurs dynamic dispatch but negligible compared to inference cost.

## Assumptions
- Debounce timer (0.18s) throttles inference to reduce load.
- Single pending inference at a time; parallelism limited to one async task.

## Improvements
- Add buffer reuse/pooling for readback and detection vectors to cut allocations.
- Allow multiple in-flight inference tasks or batch processing if throughput is needed.
- Consider dropping heuristic detector logs/allocations when model loaded to reduce noise.

## Links
- Source: `crates/vision_runtime/src/lib.rs`
