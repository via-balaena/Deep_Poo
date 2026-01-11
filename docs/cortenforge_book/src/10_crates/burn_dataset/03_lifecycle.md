# burn_dataset: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
- Index and load runs into samples:
  ```rust,ignore
  let indices = index_runs(root)?;
  let samples = load_run_dataset(&indices[0].run_dir)?;
  ```
- Split and build iterators:
  ```rust,ignore
  let (train, val) = split_runs(&indices, 0.8)?;
  let loaders = build_train_val_iters(&train, &val, &cfg)?;
  ```
- Validate/summarize warehouse:
  ```rust,ignore
  let summary = summarize_runs(&indices)?;
  validate_summary(&summary, &thresholds)?;
  ```

## Execution flow
- Index runs → create SampleIndex list.
- Summarize/validate with thresholds as needed.
- Load samples for ETL or build train/val iterators to feed Burn training (requires `burn-runtime`, alias `burn_runtime`).
- Optionally use shard metadata/manifest helpers for warehouse storage/loading.

## Notes
- Backends/features: iterators/tensors require `burn-runtime` (alias `burn_runtime`); otherwise use indexing/summary helpers.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
