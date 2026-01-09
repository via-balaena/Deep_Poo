# util (training)

## Responsibility
CLI parsing and training orchestration for TinyDet/BigDet; checkpoint load/save; backend/model selection; target building helpers.

## Key items
- CLI enums/args: `ModelKind` (Tiny/Big), `BackendKind` (NdArray/Wgpu), `TrainArgs` (paths, hyperparams, variants).
- Entrypoint: `run_train` drives dataset loading, backend/model selection, and training loop; writes checkpoints.
- Checkpoint helpers: `load_tinydet_from_checkpoint`, `load_bigdet_from_checkpoint`.
- Backend/model aliases: `TrainBackend`, ADBackend for training.
- Target builder: `build_greedy_targets` (IoU-based matching of preds to GT).
- Validation: `validate_backend_choice` ensures features match requested backend.

## Training loops (high level)
- TinyDet: collate batch → take first box as input → predict objectness → MSE loss vs has_box mask.
- BigDet: collate batch → concatenate first box + features → forward_multibox → greedy target matching → BCE for objectness + regression loss on matched boxes → optimize with Adam.
- Checkpoint save via `BinFileRecorder`.

## Invariants / Gotchas
- `run_train` bails early if no samples found.
- BackendKind vs compiled features: warns/errors if mismatch (e.g., requesting Wgpu without feature).
- BigDet input_dim set to 4+8 (first box + features); adjust if features change.
- Greedy matching is simple/deterministic; may be improved later.

## Cross-module deps
- Uses dataset::collate/DatasetConfig; models for TinyDet/BigDet; Burn for backend/optimizers; clap for CLI.
