# capture (data_contracts)

## Responsibility
Define frame-level capture metadata and label schemas with validation.

## Key types
- `PolypLabel`: label with center_world and optional bbox_px/bbox_norm.
- `CaptureMetadata`: frame metadata (ids/timestamps/image info/camera_active/polyp_seed/polyp_labels).
- `ValidationError`: validation failures for bbox_px/bbox_norm/missing image.

## Important functions
- `PolypLabel::validate`: ensures bbox ordering/nan checks and norm ranges.
- `CaptureMetadata::validate`: ensures image present when flagged and validates all labels.

## Invariants / Gotchas
- bbox_px must have proper ordering; bbox_norm must be 0..1 and ordered.
- image must be non-empty if image_present is true.

## Cross-module deps
- Used by capture_utils/recorder sinks, tools overlay/prune, and ETL/training pipelines.
