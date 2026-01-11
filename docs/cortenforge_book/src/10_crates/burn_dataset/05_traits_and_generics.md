# Traits & Generics (burn_dataset)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- `WarehouseShardStore` (feature `burn-runtime`, alias `burn_runtime`): trait abstraction for providing shard iterators. Methods: `train_iter`, `val_iter`, `train_len`, `val_len`, `total_shards`, `mode`.

## Implementations
- `InMemoryStore`: stores shard buffers in memory; implements `WarehouseShardStore`.
- `StreamingStore`: mmap/streaming-backed shard access; implements `WarehouseShardStore`.
- `WarehouseLoaders` wraps a boxed `WarehouseShardStore`.

## Generics and bounds
- `BatchIter<B: Backend>` and `BurnBatch<B>` depend on Burn `Backend` for tensor creation.
- Loader abstractions use concrete structs; trait object (`Box<dyn WarehouseShardStore>`) allows swapping store strategies.
- Most APIs are concrete; no higher-rank or lifetime-heavy generics.

## Design notes
- Trait object for `WarehouseShardStore` keeps CLI/runtime selection simple (stream vs memory).
- Backend-generic batch iteration mirrors training/inference expectations; consumers must align `max_boxes`/`target_size` configs with models.
- If additional store types are added (e.g., remote), implement `WarehouseShardStore` to plug in without changing callers.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
