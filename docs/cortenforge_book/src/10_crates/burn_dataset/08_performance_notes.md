# Performance Notes (burn_dataset)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Image decode/augmentation in `load_sample` and `BatchIter`.
- Rayon parallel load in `BatchIter::next_batch`.
- Warehouse shard load (mmap/streaming) in `ShardBuffer::copy_sample` and `WarehouseBatchIter`.

## Allocation patterns
- Buffers (`images_buf`, `boxes_buf`, `mask_buf`, `frame_ids_buf`) are reused per batch but can grow; preallocated to capacity when possible.
- Shard loaders copy data from mmap/stream into owned vectors per batch.
- Transform pipeline allocates intermediate images during jitter/resize.

## Trait objects
- `WarehouseShardStore` trait object allows different backends with minor vtable overhead; negligible compared to IO/compute.

## Assumptions
- Images in a batch share dimensions; otherwise collation fails.
- Parallelism is helpful for CPU-bound decode; IO/cache behavior may dominate on large datasets.
- Env tuning for logging/permissive/trace impacts performance minimally.

## Improvements
- Further buffer pooling or preallocation could reduce reallocs for large batches.
- Consider SIMD-optimized aug if profiling shows jitter/blur as bottlenecks.
- Tune Rayon thread pool or chunk sizes for specific workloads.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
