# burn_dataset: Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Single-module crate defining dataset types, transforms, summaries/validation, shard metadata/manifest, iterators, and helpers.
  - Types: DatasetConfig, SampleIndex, DatasetSample.
  - Iterators: BatchIter, WarehouseBatchIter.
  - Helpers: index/summarize/load/split/build iterators.

Cross-module dependencies:
- pure Rust with Burn/backends.
- consumed by training/tools.
- no submodules beyond lib.rs.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
