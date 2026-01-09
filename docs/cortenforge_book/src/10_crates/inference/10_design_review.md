# Design Review (inference)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Clean factory abstraction that hides detector choice (heuristic vs. Burn) behind one API.
- Feature-gated backend/model selection keeps surface area stable.
- Minimal preprocessing keeps detector integration simple.

## Risks / gaps
- Mutex-guarded model serializes inference; not suitable for high-throughput unless duplicated per thread.
- Error handling is implicit (logs + heuristic fallback); hard for callers to detect/model-load failures programmatically.
- Preprocessing is extremely naive (RGB mean + aspect ratio); may not match trained model expectations for real data.

## Refactor ideas
- Return a structured result from factory load (e.g., enum with “LoadedBurn”/“Fallback”) to allow callers to act on failures.
- Add configurable preprocessing pipeline to align with training data; consider using `capture_utils`/`burn_dataset` transforms.
- Provide an option to create per-thread detectors to avoid mutex contention, or use a thread-safe queue of detectors.

## Links
- Source: `inference/src/factory.rs`
