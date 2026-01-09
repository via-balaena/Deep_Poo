# Performance Notes (capture_utils)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Recording captures: JSON serialization per frame; disk IO dominates.
- Overlay generation: per-image decode + draw loops over bounding boxes.
- Pruning: file copy operations across run directories.

## Allocation patterns
- `JsonRecorder` allocates per-frame JSON buffer; uses `BufWriter` to reduce syscalls.
- Overlay generation allocates decoded image buffers; no reuse/pooling.
- Prune copies files; no large in-memory buffers beyond image decode.

## Trait objects
- Recorder invoked via trait object; negligible overhead compared to IO.

## Assumptions
- Throughput bounded by filesystem. Image decode/encode via `image` crate is the main CPU cost in overlays.

## Improvements
- Add buffer reuse/pooling for overlay images if processing large datasets.
- Parallelize overlay generation/pruning per file for speedups (currently sequential).

## Links
- Source: `capture_utils/src/lib.rs`
