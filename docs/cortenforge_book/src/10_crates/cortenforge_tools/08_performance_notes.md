# Performance Notes (cortenforge-tools)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- None in the helpers themselves; performance is dictated by underlying tools (capture_utils, warehouse/training binaries).

## Allocation patterns
- Services allocate small vectors/strings for command assembly; minimal.
- Warehouse command builder constructs strings; negligible.
- Overlay/recorder functions reuse upstream implementations (overlay/prune/recorder) with their own allocation patterns.

## Trait objects
- Uses upstream trait objects (recorders/detectors) when re-exported; overhead is negligible relative to IO/compute of the underlying crates.

## Assumptions
- This crate is not performance-critical; primarily orchestration/wrapper code.
