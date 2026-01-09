# burn_dataset: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| `DatasetResult<T>` | type | Result alias with BurnDatasetError |
| BurnDatasetError | enum | Error variants for dataset ops |
| DatasetSample | struct | Single sample from dataset |
| DatasetConfig | struct | Config for dataset loading/splitting |
| ResizeMode | enum | Resize behavior for images |
| SampleIndex | struct | Index of samples in runs |
| CacheableTransformConfig | struct | Config for caching transforms |
| TransformPipeline | struct | Pipeline of transforms |
| TransformPipelineBuilder | struct | Builder for transform pipelines |
| RunSummary | struct | Summary of a run |
| DatasetSummary | struct | Summary across runs |
| ValidationOutcome | enum | Validation result |
| ValidationThresholds | struct | Thresholds for validation |
| ValidationReport | struct | Validation report |
| ShardMetadata | struct | Metadata for warehouse shard |
| ShardDType | enum | Shard data type |
| Endianness | enum | Endianness for shard data |
| WarehouseStoreMode | enum | Storage mode for warehouse |
| WarehouseManifest | struct | Manifest for warehouse shards |
| `BurnBatch<B>` | struct | Batch for Burn backend B |
| BatchIter | struct | Iterator over batches |
| WarehouseBatchIter | struct | Iterator over warehouse batches |
| WarehouseShardStore | trait | Interface for shard storage backends |
| WarehouseLoaders | struct | Loaders for warehouse shards |
| split_runs | fn | Split runs into train/val |
| split_runs_stratified | fn | Stratified split of runs |
| count_boxes | fn | Count boxes in samples |
| validate_summary | fn | Validate summary against thresholds |
| summarize_with_thresholds | fn | Summarize with thresholds |
| summarize_root_with_thresholds | fn | Summarize root with thresholds |
| summarize_runs | fn | Summarize runs from indices |
| index_runs | fn | Index runs under a root |
| load_run_dataset | fn | Load dataset from a run dir |
| load_sample_for_etl | fn | Load a sample for ETL |
| build_train_val_iters | fn | Build train/val iterators |
| build_greedy_targets | fn | Build targets for training |

## Links
- Source: `crates/burn_dataset/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge-burn-dataset
