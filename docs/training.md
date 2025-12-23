# Burn training harness

Basic usage (enable Burn runtime):
```bash
cargo run --features burn_runtime --bin train -- --help
```
GPU (wgpu) build: add `burn_wgpu` alongside `burn_runtime`:
```bash
cargo run --features "burn_runtime,burn_wgpu" --bin train -- --help
```

End-to-end workflow:
- Capture/prune data: run `sim_view --mode datagen` to create runs under `assets/datasets/captures`, then prune empties with `cargo run --bin prune_empty -- --input assets/datasets/captures --output assets/datasets/captures_filtered`. Do the same for any real-val root you want (`assets/datasets/real_val_filtered`).
- Point training at filtered roots: e.g., `--input-root assets/datasets/captures_filtered` and `--real-val-dir assets/datasets/real_val_filtered`, or rely on the split manifest/seed for repeatable splits.
- Train: pick a batch/epochs schedule and run (see sample commands below). Metrics/logs print to stdout; optional `--metrics-out` appends JSONL with IoU/PR/mAP per epoch.
- Outputs: model/optim/scheduler checkpoints land in `--ckpt-dir` (default `checkpoints/`); metrics JSONL if requested. A demo checkpoint can be loaded via `--demo-checkpoint`.

Common flags:
- `--batch-size <N>`: training/val batch size (default 2). Batch >1 is supported; targets are built per-image.
- `--epochs <N>`: number of epochs (default 1)
- `--log-every <N>`: log loss/IoU every N steps (default 1)
- `--lr-start <f64>`, `--lr-end <f64>`: linear LR schedule across total steps
- `--scheduler <linear|cosine>`: pick LR schedule type
- `--ckpt-every-steps <usize>`: checkpoint cadence in steps (0 disables)
- `--ckpt-every-epochs <usize>`: checkpoint cadence in epochs
- `--val-ratio <f32>`: fraction of runs for validation split (default 0.2)
- `--seed <u64>`: deterministic shuffle/splits; omit for random
- `--ckpt-dir <path>`: where model/optim/scheduler checkpoints are read/written (default `checkpoints`)
- `--val-obj-thresh <f32>`: objectness threshold for val matching (default 0.3)
- `--val-iou-thresh <f32>`: IoU threshold for NMS/matching (default 0.5); mAP is computed over objectness thresholds 0.05..0.95.
- `--patience <usize>` / `--patience-min-delta <f32>`: optional early stop on val IoU plateau
- `--real-val-dir <path>`: optional separate validation root; if set, uses all runs under this path for validation instead of a split.
- `--input-root <path>`: capture root to train from (default `assets/datasets/captures`); point to a filtered/pruned root if desired.
- `--stratify-split`: stratify train/val by box count buckets (0/1/2+ boxes) instead of a pure run-level split.
- `--split-manifest <path>`: optional JSON manifest; if present, load train/val label lists from it; if absent, save the current split for reuse.
- Seeds: if `--seed` is omitted, a default seed (42) is used and logged for repeatability.
- `--demo-checkpoint <path>`: optional model checkpoint to load at startup (model only; skips optimizer/scheduler). Use to run with a bundled/demo weight file if available.
- `--metrics-out <path>`: optional JSONL output; if set, appends per-epoch val metrics (IoU/PR/mAP, tp/fp/fn) with seed and thresholds.
- `--demo-checkpoint <path>`: optional model checkpoint to load at startup (model only; skips optimizer/scheduler). Use this to try a bundled/demo weight if provided.

Sample run (CPU backend):
```bash
cargo run --features burn_runtime --bin train -- \
  --batch-size 4 \
  --epochs 5 \
  --scheduler cosine \
  --lr-start 1e-3 \
  --lr-end 1e-4 \
  --val-ratio 0.2 \
  --ckpt-every-epochs 1 \
  --val-obj-thresh 0.3 \
  --val-iou-thresh 0.5 \
  --real-val-dir assets/datasets/real_val \
  --input-root assets/datasets/captures_filtered \
  --stratify-split \
  --demo-checkpoint assets/checkpoints/tinydet_demo.bin
```

Sample run (wgpu backend):
```bash
cargo run --features "burn_runtime,burn_wgpu" --bin train -- \
  --batch-size 4 \
  --epochs 5 \
  --scheduler cosine \
  --lr-start 1e-3 \
  --lr-end 1e-4 \
  --val-ratio 0.2
```

What it does today:
- Loads capture runs via `BatchIter` (train with aug; val without), builds TinyDet, AdamW, and a linear LR scheduler.
- Runs epoch/batch loop with per-step optimizer updates; logs loss and mean IoU each log interval.
- Validation: decodes per-cell predictions, applies sigmoid + NMS, matches to GT boxes with IoU threshold, and reports mean IoU plus precision/recall and an approximate mAP via a small PR sweep (tp/fp/fn).
- Checkpoints: on start, loads model/optim/scheduler from `ckpt_dir` if present; saves them per configured cadence (steps/epochs). Optional early stop tracks best val IoU.

Notes:
- Requires `--features burn_runtime` to pull in Burn and the training harness. Add `burn_wgpu` to use the GPU backend (wgpu) when available; otherwise NdArray CPU is used.
- Val metric thresholds are tunable via CLI; adjust to trade off recall/precision during evaluation.
- Runtime inference will attempt to load `checkpoints/tinydet.bin`; if missing or failed, it logs a warning and falls back to the heuristic detector.
- Demo/bundled checkpoint: if you have a packaged checkpoint (e.g., `assets/checkpoints/tinydet_demo.bin`), pass `--demo-checkpoint <path>` during training/eval. For runtime, place it at `checkpoints/tinydet.bin` (or update the CLI flags) so the sim loads it automatically.
- Runtime knobs: during sim, adjust thresholds with `-`/`=` (obj) and `[`/`]` (IoU); press `B` to toggle between Burn/heuristic detectors. The HUD shows mode, box count, and inference latency, plus a fallback banner if Burn is unavailable.
- Eval-only: use `cargo run --features burn_runtime --bin eval -- --checkpoint <path> --input-root <val_root> [--val-iou-sweep ...] [--metrics-out ...]` to score a checkpoint without training.
- Runtime thresholds: during sim, adjust detection thresholds with `-`/`=` (objectness down/up) and `[`/`]` (IoU down/up); HUD will reflect updates.
- Runtime detector toggle: press `B` to switch between Burn and heuristic detectors; the HUD VISION line shows the active mode and box stats.

Next steps (nice-to-haves):
- Expose predicted boxes/confidence to HUD/`DetectionResult` so runtime shows actual detections, not just a bool. ✅
- Bundle a small demo checkpoint or fall back to the heuristic detector with a clear log when no Burn model is available. ✅ (warns + heuristic fallback when checkpoint missing)
- Tighten validation metrics with per-image precision/recall or mAP in addition to mean IoU. ✅ (mean IoU + precision/recall + approximate mAP sweep logged)
- Add a sample `train` command here with typical flags, and expose inference thresholds via CLI/env.
