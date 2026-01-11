# Integration Contracts
**Why**: These are the promises between crates that keep the system stable.
**How it fits**: When these change, many pages and flows must update.
**Learn more**: See [Canonical Flows](canonical_flows.md).

Key assumptions and contracts between crates. Keep these in sync as APIs evolve.

## Types and data shapes
Shared contracts that downstream crates rely on; update with care.

| Contract | Downstream crates | Impact |
| --- | --- | --- |
| `vision_core::interfaces::{Frame, DetectionResult, Label, FrameRecord}` | `vision_runtime`, `inference`, `capture_utils`, recorders | Changes propagate across runtime, inference, and sinks. |
| `data_contracts::capture::{CaptureMetadata, PolypLabel}` | `capture_utils`, `burn_dataset`, tools | Recorder outputs and dataset loaders assume this schema; validators and ETL rely on fields being stable. |
| `burn_dataset` tensors | `training`, `inference`, `models` | `BatchIter`/`collate` expect consistent image dimensions per batch and `max_boxes` alignment. |
| `models` output | `training`, `inference` | `TinyDet`/`BigDet` scores align with `max_boxes`; inference and training must agree on this. |

## Feature expectations
Feature flags that have cross-crate implications.

| Expectation | Details |
| --- | --- |
| Backends | Default NdArray; enable `backend-wgpu` in `models`, `training`, `inference` to use GPU. |
| Models | `bigdet` feature switches inference model type; downstream code should not assume TinyDet dimensions when `bigdet` is on. |
| Thread safety | `vision_runtime` assumes detectors are `Send + Sync`; `InferenceFactory` supplies such detectors. |
| Tooling features | `cortenforge-tools` bins: `tui`, `scheduler`, `gpu-nvidia` (alias `gpu_nvidia`) gate heavy deps; default footprint is minimal. |

## Runtime assumptions
Operational assumptions that affect how runtime and tooling behave.

| Assumption | Implication |
| --- | --- |
| `vision_runtime::CapturePlugin` runs only in `Datagen`/`Inference` | App must set `sim_core::SimRunMode` accordingly. |
| Single detector in inference loop | Async tasks swap detector instances without locks; throughput is limited. |
| Capture layout (`run_dir/images`, `labels`, `overlays`) | Recorders/datasets/tools expect this structure. |
| `burn_dataset::BatchIter` size expectations | Consistent image sizes required unless `target_size` forces resizing. |
| Permissive env vars | `BURN_DATASET_*`/`WAREHOUSE_*` affect behavior; CI/publish should set explicitly. |
| `cortenforge-tools` unpublished | App-specific bins belong in app repos; shared helpers may migrate. |

## Error/compatibility expectations
How failures propagate and what must stay in sync.

| Expectation | Implication |
| --- | --- |
| Inference fallback | If model weights can’t be loaded, inference switches to a heuristic detector—apps should make that visible (e.g., a status banner or log). |
| Dataset errors | `BurnDatasetError` informs whether to skip (permissive) or fail-fast in training/ETL. |
| Schema/interface changes | Updates to `data_contracts` or `vision_core` require coordinated changes across recorders, loaders, and runtime. |
