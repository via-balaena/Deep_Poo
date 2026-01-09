# Ownership & Concurrency (burn_dataset)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Most data is owned per loader/iterator: `DatasetSample`, buffers inside `BatchIter`, shard buffers.
- Shard data may be memory-mapped (`memmap2::Mmap`) or owned `Vec<f32>`; accessors copy into caller-owned buffers when iterating.
- `ShardBuffer` encapsulates backing storage; `WarehouseBatchIter` holds either direct order or streaming channel.

## Concurrency
- `BatchIter` uses Rayon `par_iter` to load samples in parallel; internal buffers are reused per batch (owned by the iterator, not shared externally).
- `StreamingStore` spawns a thread feeding a bounded channel (`crossbeam_channel`) for streaming shards.
- No shared mutable state across threads beyond controlled buffers; channel boundaries provide synchronization.

## Borrowing boundaries
- Iterators own buffers and return owned tensors; no references escape.
- Mmap/streamed shard access copies data into caller-owned vectors before constructing tensors.

## Async boundaries
- Uses threads/channels for streaming; otherwise synchronous. No async/await.

## Risks / notes
- BatchIter buffers are reused; not `Send + Sync` safe to share the iterator across threads.
- StreamingStore thread will stop on errors; callers should handle missing data if the channel closes prematurely.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
