# Error Model (burn_dataset)
Quick read: How errors are surfaced and handled.

## Errors defined
- `BurnDatasetError` enum: `Io`, `Json`, `Validation`, `MissingImage`, `MissingImageFile`, `Image`, `Other(String)`.
- Type alias `DatasetResult<T> = Result<T, BurnDatasetError>`.

## Patterns
- IO/serde/image errors wrapped with path context.
- Validation errors flag label inconsistencies (bbox ordering, missing fields) and are returned early.
- Batch/shard loaders propagate errors; permissive mode allows skipping with logging (controlled via env).
- Warehouse loaders may return `Other` for offset/shape issues; streaming/mmap errors bubble via `Io`.

## Recoverability
- Many errors are recoverable by skipping samples when `permissive` env toggles are set (`BURN_DATASET_PERMISSIVE`, `BURN_DATASET_WARN_ONCE`).
- Missing images / invalid labels abort the current sample; iterators continue when permissive.
- Shape mismatches (varying image sizes) are fatal for a batch unless target_size enforces consistency.

## Ergonomics
- Structured error with path context aids debugging.
- Some paths log and continue (permissive) vs. fail-fast; callers should choose mode based on use case.
- Mixing env-driven behavior can be surprising; document/override in callers for deterministic CI.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
