# Data & models

Schemas, artifact layout, and model checkpoints used by the substrate.

## Schemas (data_contracts)
- Run manifest: id, seed, camera config, resize/letterbox, frame count, checksum.
- Frame label: bbox (norm/px), class, metadata, optional overlays.
- Warehouse manifest/shard: tensor paths, shapes, label mapping, shard ids.
- Principle: keep validation strict (bbox ranges, required fields); preserve compatibility for ETL/training/tools.

## Artifact layout (typical)
- Captures: `assets/datasets/captures/run_<ts>/` (frames, labels, overlays, manifest).
- Filtered: `assets/datasets/captures_filtered/run_<ts>/` (pruned/overlayed copies).
- Warehouse: `artifacts/tensor_warehouse/v<ts>/` (shards + manifest).
- Checkpoints: `checkpoints/<model>/<ts>/` (TinyDet/BigDet).

## Models
- Crate: `models` defines TinyDet/BigDet (Burn).
- Training crate: loads warehouse manifests, produces checkpoints.
- Inference crate: loads checkpoints; falls back to heuristic detector if none provided.
- Feature flags:
  - Training/inference: `backend-wgpu` (GPU optional; default NdArray).
  - Inference: `tinydet` / `bigdet` gates model variants.

## Versioning/migration
- Current target: `0.1.1` across crates.
- `burn-core` temporarily patched to vendored 0.14.0 due to bincode API change; drop patch when upstream releases a fix.
- When changing schemas, document breaking vs non-breaking changes and update ETL/tools/tests accordingly.

## Quick references
- Schema definitions: `data_contracts/src/`
- Model definitions: `models/src/`
- Training entrypoints: `training/src/bin/train.rs`, `training/src/bin/eval.rs`
- Inference factory: `inference/src/lib.rs`
