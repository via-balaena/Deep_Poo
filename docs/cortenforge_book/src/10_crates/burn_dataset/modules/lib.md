# lib (burn_dataset)

## Responsibility
- Provide dataset ingestion, augmentation, validation, and batching utilities for capture datasets.
- Support both eager dataset loading and Burn-ready batch iterators/sharded “warehouse” loaders (feature `burn_runtime`).

## Key items
- Error/modeling:
  - `BurnDatasetError`, `DatasetResult`.
  - `DatasetSample` (CHW image f32 + boxes), `SampleIndex`, `DatasetConfig`, `TransformPipeline(+Builder)`.
  - `RunSummary`, `DatasetSummary`, `ValidationThresholds`, `ValidationReport`.
- Splitting/indexing:
  - `split_runs`, `split_runs_stratified`, `index_runs`, `summarize_runs`, `summarize_with_thresholds`, `summarize_root_with_thresholds`.
- Loading/augmentation:
  - `load_run_dataset`, `load_sample_for_etl`, `load_sample` (internal), `build_sample_from_image`.
  - Aug helpers: `maybe_hflip`, `maybe_jitter`, `maybe_noise`, `maybe_scale_jitter`, `maybe_blur`, `letterbox_resize`, box normalization helpers.
- Warehousing / streaming (feature `burn_runtime`):
  - `BurnBatch`, `BatchIter` (in-memory iterator over captures with augmentation).
  - Sharded warehouse metadata: `ShardMetadata`, `WarehouseManifest`, `WarehouseStoreMode`.
  - Shard loaders/backing: `ShardBuffer`, `WarehouseShardStore` (trait), `WarehouseBatchIter`, `StreamingStore`, `InMemoryStore`, `WarehouseLoaders`.

## Invariants / Gotchas
- `DatasetConfig` defaults to letterbox 512x512, max_boxes=16, shuffle=true, skip_empty=true; adjust for eval vs. training.
- `load_sample` validates labels (bbox ordering, presence); missing images yield explicit errors.
- `BatchIter` expects consistent image sizes per batch; set `target_size` to avoid shape mismatches.
- Augmentations mutate boxes; clamping to [0,1] is applied but extreme jitter/scale can still collapse boxes.
- Warehouse shard reading supports owned, mmap, and streaming modes; offsets are computed from binary header—corruption will surface as `Other` errors.
- Env knobs: `BURN_DATASET_LOG_EVERY`, `BURN_DATASET_PERMISSIVE`, `BURN_DATASET_WARN_ONCE`, `BURN_DATASET_TRACE`, `WAREHOUSE_STORE`, `WAREHOUSE_PREFETCH`, validation thresholds env vars.

## Cross-module deps
- Consumed by `training` and downstream ETL/warehouse tooling.
- Uses `rand`, `image`, `serde`, `sha2`, `crossbeam_channel`/`rayon`/`memmap2` when `burn_runtime` enabled.
- Shard loaders output Burn tensors; align `max_boxes` and `target_size` with model expectations in `models`/`inference`.
