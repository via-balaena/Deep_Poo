# Performance Notes (training)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Image decode and tensor construction in `collate`.
- Tensor operations inside training loop (in `run_train`, not shown here); dominated by backend compute.

## Allocation patterns
- `collate` allocates image buffers (`Vec<f32>`) sized to batch * pixels * 3; reused per call scope, not pooled.
- Features/boxes/masks allocated per batch; zero-filled buffers for boxes/masks.

## Trait objects
- None; static dispatch via backend generics.

## Assumptions
- Batch images must share dimensions; otherwise collation bails. No dynamic resizing, so preprocessed datasets are assumed.
- CPU path may be slow for large images/batches; WGPU backend should be used for acceleration.

## Improvements
- Add buffer pooling or use `Vec::with_capacity` reuse between batches if integrating into a long-running trainer.
- Parallelize image decode/loading across batches if IO becomes the bottleneck.

## Links
- Source: `crates/training/src/dataset.rs`
