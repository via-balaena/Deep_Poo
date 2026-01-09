# training: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| DatasetConfig | struct | Config for dataset loading (paths, batch, splits) |
| RunSample | struct | Single sample from run/manifest |
| `CollatedBatch<B>` | struct | Batch of tensors for Burn backend B |
| `collate<B>` | fn | Collate run samples into a batch |
| TrainBackend | type | Backend alias (NdArray or WGPU) |
| TrainArgs | struct | CLI args for training |
| ModelKind | enum | Model variant selection (TinyDet/BigDet) |
| BackendKind | enum | Backend selection (NdArray/WGPU) |
| run_train | fn | Entry point to run training from args |
| load_tinydet_from_checkpoint | fn | Load TinyDet checkpoint |
| load_bigdet_from_checkpoint | fn | Load BigDet checkpoint |
| validate_backend_choice | fn | Validate backend choice |
| build_greedy_targets | fn | Build targets for training |
| Re-exports | re-export | TinyDet/BigDet configs and models from models crate |
| Modules (pub mod) | module | dataset, util |

## Links
- Source: `training/src/lib.rs`
- Module: `training/src/dataset.rs`
- Module: `training/src/util.rs`
- Docs.rs: https://docs.rs/cortenforge-training
