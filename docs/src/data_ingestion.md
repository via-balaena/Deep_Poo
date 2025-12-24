# Data ingestion safeguards

## Processing order (with controls)
1) **Decode labels** from JSON into in-memory structs.
2) **Prune/clip invalid boxes** (e.g., out-of-bounds, zero-area).
3) **Drop empty** (after pruning):
   - `DatasetConfig::skip_empty_labels` (default true) skips samples with no remaining labels and logs the label path, preventing zero-target batches.
   - To pre-filter datasets, use the `prune_empty` CLI to copy runs without empty-label frames:
     - Usage: `cargo run --release --bin prune_empty -- --input <in_dir> --output <out_dir>`
     - Copies `run_manifest.json`, labels/images/overlays for frames with at least one `polyp_labels` entry; skips empty-label frames (counts printed).
     - Example (captures): `cargo run --release --bin prune_empty -- --input assets/datasets/captures --output assets/datasets/captures_filtered`
     - Example (real val): `cargo run --release --bin prune_empty -- --input assets/datasets/real_val --output assets/datasets/real_val_filtered`
4) **Tensorize & batch** the remaining samples for the model.
   - If an entire split ends up empty, the iterator yields no batches; regenerate data or point at runs with labels.
