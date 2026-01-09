# dataset (training)

## Responsibility
Load capture datasets from disk (labels/images per data_contracts), collate into batches for Burn backends, and provide basic summaries.

## Key types
- `DatasetConfig`: root/labels/images paths.
- `RunSample`: image path + CaptureMetadata.
- `CollatedBatch<B>`: tensors for images, boxes, mask, and features (mean/std RGB, aspect, box count).

## Important functions
- `DatasetConfig::load`: reads label JSONs, validates via data_contracts, pairs with images.
- `collate<B>`: loads batch of images, normalizes to CHW tensors, computes features, normalizes boxes, pads/truncates to max_boxes, builds mask.

## Invariants / Gotchas
- Assumes consistent image dimensions within a batch; errors if mismatch.
- bbox_norm vs bbox_px handling: falls back to px conversion when norm missing.
- max_boxes is clamped to at least 1; masks indicate valid boxes.
- Loads all images into memory during collate; watch batch sizes for memory.

## Cross-module deps
- Uses data_contracts::capture for metadata, image crate for loading, Burn tensors for outputs. Consumed by util/run_train.
