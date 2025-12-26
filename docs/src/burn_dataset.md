# Burn dataset loader usage

## Quick start
Load capture runs, split train/val by run, and iterate batches with letterboxed resizing and padded boxes:

```rust
use colon_sim::tools::burn_dataset::{BatchIter, DatasetConfig, ResizeMode, split_runs, index_runs};

let cfg = DatasetConfig {
    target_size: Some((512, 512)),
    resize_mode: ResizeMode::Letterbox,
    ..Default::default()
};

let all = index_runs(std::path::Path::new("assets/datasets/captures"))?;
let (train_idx, val_idx) = split_runs(all, 0.2);

let mut train = BatchIter::from_indices(train_idx, cfg.clone())?;
let mut val = BatchIter::from_indices(val_idx, cfg)?;

if let Some(batch) = train.next_batch::<burn_ndarray::NdArrayBackend<f32>>(8)? {
    // batch.images: [B,3,H,W], batch.boxes: [B,max_boxes,4], box_mask: [B,max_boxes]
}
```

## Feature flags
- `BatchIter`/`BurnBatch` require `--features burn_runtime` to pull in Burn (e.g., `cargo run --features burn_runtime --bin ...`).  
- Lower-level loaders (`load_run_dataset`, `DatasetSample`) work without that feature if you only need plain Rust structs.

## Augmentations
- `flip_horizontal_prob`: optional horizontal flip with box updates (after resize/letterbox).
- `scale_jitter_*`: optional zoom in/out with bbox-safe padding/cropping.
- `color_jitter_*`: optional brightness/contrast jitter.
- `noise_*`: optional uniform noise per channel.
- `blur_*`: optional blur.
- `skip_empty_labels` (default true): drop frames with no boxes; warns instead of emitting zero-target batches.
- `drop_last`: drop the last partial training batch if set (validation should keep partials).

## Reproducibility
- `seed` controls shuffling in `BatchIter` so splits/batches are deterministic when set.
 - CLI example (train): `cargo train_hp --seed 1234`

## Progress logging
- Default: emits loader progress about every 1k samples (`[dataset] batches=… samples=… skipped=… rate=… img/s`).
- Override: set `BURN_DATASET_LOG_EVERY=<N>` (samples) for a custom cadence.
- Disable: set `BURN_DATASET_LOG_EVERY=0` or `off`.
 - Example: `BURN_DATASET_LOG_EVERY=2000 cargo train_hp`
 - Logs include skip counts (`skipped_empty`, `skipped_missing`, `skipped_errors`) plus timing/rate metrics.

## Error handling
- Default: permissive; malformed/missing labels are skipped with a warning.
- Strict: set `BURN_DATASET_PERMISSIVE=0` or `off` to fail fast on bad labels/images.
- Examples:
   - Permissive (default): `BURN_DATASET_PERMISSIVE=1 cargo train_hp`
   - Strict: `BURN_DATASET_PERMISSIVE=0 cargo train_hp`
- Warning aggregation: set `BURN_DATASET_WARN_ONCE=1` to suppress per-sample warnings and emit only aggregated progress logs.
- Optional trace: set `BURN_DATASET_TRACE=<path/to/trace.jsonl>` to write per-batch JSONL records (batch idx, samples, dims, skips, load/assemble ms).

## Transform pipeline builder
- Compose a custom pipeline (resize/letterbox + augments + max_boxes) and attach it to `DatasetConfig.transform`:
```rust
use colon_sim::tools::burn_dataset::{DatasetConfig, TransformPipelineBuilder, ResizeMode};

let pipeline = TransformPipelineBuilder::new()
    .target_size(Some((256, 256)))
    .resize_mode(ResizeMode::Letterbox)
    .flip_horizontal_prob(0.5)
    .color_jitter(0.2, 0.1)
    .max_boxes(8)
    .seed(Some(42))
    .build();

let cfg = DatasetConfig {
    transform: Some(pipeline),
    ..Default::default()
};
```
- If no transform is provided, one is built from the other `DatasetConfig` fields (default behavior).
- The train binary logs the active transform config for train/val (target size, resize mode, aug probs/strengths, max_boxes, seed).
- Val-specific overrides (train CLI):
  - `--val-target-size <WxH>` (e.g., `256x256`; defaults to train target_size)
  - `--val-resize-mode <force|letterbox>` (defaults to train resize_mode)
  - `--val-flip-prob <p>` (defaults to 0.0 for val)
  - `--val-max-boxes <N>` (defaults to train max_boxes)
- Example: `cargo train_hp --val-target-size 256x256 --val-resize-mode letterbox --val-max-boxes 8`

## Helper entrypoint
- Build train/val iterators with defaults in one call:
```rust
use colon_sim::tools::burn_dataset::{DatasetConfig, build_train_val_iters};
let train_cfg = DatasetConfig {
    target_size: Some((128, 128)),
    max_boxes: 8,
    flip_horizontal_prob: 0.5,
    ..Default::default()
};
let (mut train_iter, mut val_iter) = build_train_val_iters(
    std::path::Path::new("assets/datasets/captures_filtered"),
    0.2,
    train_cfg,
    None, // Optional val cfg override
)?;
```
Validation defaults: no shuffle, no aug, drop_last=false, flip_prob=0 unless overridden via `val_cfg`.
- Requires `burn_runtime` feature (same as `BatchIter`).

<details>
<summary>Improvement roadmap (ranked)</summary>

1) Throughput & memory
   - ❌ Optional on-disk cache of resized/letterboxed images + normalized boxes (deferred).
     <details>
     <summary>Implementation plan</summary>

     - Scope: cache deterministic steps only (decode + resize/letterbox + normalize boxes); skip augments.
     - Key: dataset root + run path + target_size + resize_mode + max_boxes + skip_empty + version hash; treat any change as a miss.
     - Storage: per-label binary blob under a cache root (e.g., `logs/cache/burn_dataset`), header with dims/max_boxes/count/checksum.
     - Invalidation: recompute if source mtime > cache mtime; support `BURN_DATASET_CACHE_CLEAR=1` to wipe cache.
     - Controls: opt-in via env; disabled by default.

     </details>


</details>
