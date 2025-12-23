# Data ingestion safeguards

- `DatasetConfig::skip_empty_labels` (default true) skips frames whose `polyp_labels` are empty, logging the label path. This prevents training on zero-target batches.
- If all samples in a split are empty, the iterator returns no batches; consider regenerating data or pointing the loader at runs with labels.
- `prune_empty` CLI (new): copy runs into a new folder while omitting frames/overlays with empty labels, so you can keep originals intact and train against a filtered dataset.
  - Usage: `cargo run --release --bin prune_empty -- --input assets/datasets/captures --output assets/datasets/captures_filtered`
  - The tool copies `run_manifest.json`, labels/images/overlays for frames that have at least one `polyp_labels` entry, and skips empty-label frames (counts are printed).
  - Example for captures: `cargo run --release --bin prune_empty -- --input assets/datasets/captures --output assets/datasets/captures_filtered`
  - Example for real val: `cargo run --release --bin prune_empty -- --input assets/datasets/real_val --output assets/datasets/real_val_filtered`
