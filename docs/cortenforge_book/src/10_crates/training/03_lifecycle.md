# training: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
1) Prepare args/config and backend:
   ```rust,ignore
   let args = TrainArgs::parse(); // from clap
   run_train(args)?;
   ```
2) Loader builds dataset from warehouse manifest, splits runs, collates batches.
3) Model (TinyDet/BigDet) is constructed and trained; checkpoints written to output dir.

## Execution flow
- CLI parses `TrainArgs` (model/backend selection, paths, output).
- Dataset loader reads warehouse manifest, builds `DatasetConfig`, collates batches (NdArray default; WGPU if enabled).
- Training loop runs forward/backward, logs metrics, saves checkpoints.
- Optional eval uses similar path with eval bin.

## Notes
- Backends/features: `backend-ndarray` default, `backend-wgpu` opt-in; `tinydet`/`bigdet` variants.

## Links
- Source: `crates/training/src/util.rs`
