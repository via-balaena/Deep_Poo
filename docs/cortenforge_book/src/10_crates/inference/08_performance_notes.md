# Performance Notes (inference)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Model forward pass in `BurnTinyDetDetector::detect`.
- Frame preprocessing (`frame_to_tensor`) computes RGB means/aspect ratio; lightweight.

## Allocation patterns
- Each detect builds a small `TensorData` (shape [1,4]) and copies scores out of the output tensor.
- Uses `Arc<Mutex<...>>` around the model; contention can serialize calls.
- Fallback detector does minimal work.

## Trait objects
- Detector is a trait object; dynamic dispatch negligible compared to model compute.

## Assumptions
- Single-threaded inference by default due to mutex guard; designed for low-QPS use.
- Score extraction clones into `Vec<f32>`; acceptable for small outputs.

## Improvements
- Remove/relax mutex if using per-thread detectors for higher throughput.
- Cache/reuse input/output buffers if detecting many frames in a tight loop.

## Links
- Source: `inference/src/factory.rs`
