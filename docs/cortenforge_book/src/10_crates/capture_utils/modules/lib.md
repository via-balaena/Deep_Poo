# lib (capture_utils)

## Responsibility
Provide default recorder sink and helpers for overlays/pruning captures aligned with data_contracts schemas.

## Key items
- `JsonRecorder`: implements `vision_core::Recorder`, writes labels/metadata to `run_dir/labels/frame_XXXXX.json`.
- `generate_overlays`: renders overlay PNGs from label JSONs in a run dir.
- `prune_run`: copies a run into a filtered output, pruning empty-label frames and preserving manifests/images/overlays.

## Invariants / Gotchas
- `JsonRecorder` validates metadata against data_contracts; will error on invalid bboxes or missing images when image_present is true.
- Overlay generation skips frames without images or invalid labels; uses vision_core overlay helpers.
- Prune relies on directory layout (`labels`, `images`, `overlays`, `run_manifest.json`); ensure inputs adhere.

## Cross-module deps
- Depends on data_contracts (schemas), vision_core overlay helpers (via capture_utils re-export), image/fs.
- Used by sim_core recorder pipeline and tools (overlay/prune bins).
