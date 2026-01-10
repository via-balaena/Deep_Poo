# Ownership & Concurrency (vision_runtime)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Bevy resources own state: `DetectorHandle` (boxed detector), `BurnInferenceState` (pending task), `FrontCaptureTarget/State/Buffer`, `DetectionOverlayState`, thresholds, etc.
- Ownership of the detector moves into an async task and is returned to the resource when the task completes.

## Concurrency
- Uses Bevy’s async task pool (`AsyncComputeTaskPool`) to offload inference. Shared detector is stored in `DetectorHandle` as `Box<dyn Detector + Send + Sync>`.
- Pending inference runs in a `Task<InferenceJobResult>`; results are polled and swapped back into the resource.
- No explicit locking; ownership is transferred (taken from handle, moved into task, restored).
- Feature-gated hotkey handling mutates shared resources on the main thread.

## Borrowing boundaries
- Systems borrow resources mutably/exclusively via Bevy ECS; no long-lived borrows are kept beyond a tick.
- Capture readback buffers are owned resources; data is cloned when needed.

## Async boundaries
- `schedule_burn_inference` spawns async tasks; `poll_inference_task` polls them. Boundary is explicit via `Task`.
- No futures stored elsewhere; debounce timer ensures only one pending task at a time.

## Risks / notes
- If `DetectorHandle` is absent, systems early-return (safe). Swapping detector for heuristic during tasks avoids concurrent mutable access without locks.
- Ensure detectors used here truly satisfy `Send + Sync` due to cross-thread execution.

## Links
- Source: `crates/vision_runtime/src/lib.rs`
