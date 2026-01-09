# Ownership & Concurrency (inference)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- `InferenceFactory` produces boxed detectors; ownership is transferred to callers.
- Burn-backed detector holds `Arc<Mutex<InferenceModel<InferenceBackend>>>` to share the model between calls.
- Heuristic detector owns simple fields; no sharing.

## Concurrency
- Model access synchronized via `Mutex`; per-detect lock to guard forward pass. Suitable for multi-threaded use but serializes inference calls.
- `Arc` allows detector clones (if added) to share the same model; current code boxes a single detector.

## Borrowing boundaries
- Detector methods take `&mut self`; combined with mutex, they serialize access. No long-lived borrows are held.

## Async boundaries
- None in this crate; async scheduling is handled in `vision_runtime`.

## Risks / notes
- Mutex poisoning would panic; consider handling if long-running use cases matter.
- If high-throughput is needed, consider removing the mutex by using cloneable models or separate detectors per thread.

## Links
- Source: `inference/src/factory.rs`
