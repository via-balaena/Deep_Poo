# Ownership & Concurrency (vision_core)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Trait objects (`Detector`, `Recorder`, `FrameSource`) are expected to be owned by callers; the crate only defines the interfaces and data structs (`Frame`, `DetectionResult`, etc.).
- `FrameRecord` borrows labels slice (`&[Label]`) to avoid copies when recording.

## Concurrency
- Traits are object-safe but have no `Send/Sync` bounds at the interface level; callers decide whether to enforce thread safety. Downstream crates (vision_runtime/inference) wrap detectors/recorders in `Send + Sync`.
- No internal concurrency; interfaces are synchronous.

## Borrowing boundaries
- `FrameRecord` ties the lifetime of labels to the record call; recorder implementers must not store references beyond the call unless they clone.

## Async boundaries
- None; all APIs are synchronous. Async execution (e.g., inference tasks) is layered on elsewhere.

## Risks / notes
- Lack of `Send + Sync` bounds keeps the core interfaces flexible but requires downstream crates to add bounds when used across threads (as they do).

## Links
- Source: `vision_core/src/interfaces.rs`
