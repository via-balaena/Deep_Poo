# training: Module Map
Quick read: What each module owns and why it exists.

- `dataset`: Data types and collation for training batches.
  - Types: DatasetConfig, RunSample, CollatedBatch.
  - Function: `collate` (labels/images → tensors).
- `util`: Training utilities and entrypoints.
  - Types: TrainArgs, ModelKind, BackendKind.
  - Functions: run_train, checkpoint loaders (TinyDet/BigDet), target builders.
- `lib.rs`: Re-exports dataset/util, aliases backend/model types, pulls TinyDet/BigDet from models.

Cross-module dependencies:
- dataset feeds util/run_train.
- util constructs models from `models` and uses dataset loaders. Consumers are training bin and external callers.

## Links
- Source: `training/src/lib.rs`
- Module: `training/src/dataset.rs`
- Module: `training/src/util.rs`
