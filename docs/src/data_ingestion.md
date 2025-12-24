# Data ingestion safeguards

## Processing order (with controls)
1) **Decode labels** from JSON into in-memory structs.
   - Code: `load_sample` in `src/tools/burn_dataset.rs` deserializes `labels/frame_XXXXX.json` via `serde_json`.
   - CLI: `sim_view`/`datagen_headless` produce the label JSONs; no separate decode CLI needed.
2) **Prune/clip invalid boxes** (e.g., out-of-bounds, zero-area).
   - Code: `normalize_boxes` / `normalize_boxes_with_px` clamp coords to `[0,1]`, and resize/letterbox recomputes boxes with padding.
   - CLI: use `prune_empty` (below) if you want to strip empty-label frames ahead of time.
3) **Drop empty** (after pruning):
   - `DatasetConfig::skip_empty_labels` (default true) skips samples with no remaining labels and logs the label path, preventing zero-target batches.
   - To pre-filter datasets, use the `prune_empty` CLI to copy runs without empty-label frames:
     - Usage: `cargo run --release --bin prune_empty -- --input <in_dir> --output <out_dir>`
     - Copies `run_manifest.json`, labels/images/overlays for frames with at least one `polyp_labels` entry; skips empty-label frames (counts printed).
     - Example (captures): `cargo run --release --bin prune_empty -- --input assets/datasets/captures --output assets/datasets/captures_filtered`
     - Example (real val): `cargo run --release --bin prune_empty -- --input assets/datasets/real_val --output assets/datasets/real_val_filtered`
4) **Tensorize & batch** the remaining samples for the model.
   - Code: `BatchIter::next_batch` in `src/tools/burn_dataset.rs` builds `[B,3,H,W]` tensors and padded boxes.
   - CLI: run training with `--features burn_runtime` so the iterator can construct Burn tensors.
   - If an entire split ends up empty, the iterator yields no batches; regenerate data or point at runs with labels.
