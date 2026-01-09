# Ownership & Concurrency (capture_utils)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- `JsonRecorder` owns its `run_dir`; records are written synchronously.
- Overlay/prune operations own their buffers and operate eagerly on disk.
- No shared state across calls; helper trait `GetPixelChecked` is implemented on `RgbaImage`.

## Concurrency
- All functions are synchronous; no `Send/Sync` requirements beyond what the caller imposes.
- Recorder is typically used as a boxed trait object in other crates; must be `Send + Sync` to fit those contexts, but this crate does not add interior mutability.

## Borrowing boundaries
- `Recorder::record` borrows `&FrameRecord` for the duration of serialization; no retained borrows.
- Overlay/prune read/write files and clone data as needed; no long-lived references.

## Async boundaries
- None; intended for blocking, file-based workflows. If used in async contexts, wrap in spawn/blocking tasks.

## Risks / notes
- Concurrent writes to the same run directory from multiple recorders could collide; caller must coordinate if using in multithreaded scenarios.

## Links
- Source: `capture_utils/src/lib.rs`
