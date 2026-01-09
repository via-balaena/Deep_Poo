# manifest (data_contracts)

## Responsibility
Define run manifest schema and validation for capture runs.

## Key types
- `RunManifestSchemaVersion`: schema version enum (V1).
- `RunManifest`: run-level metadata (schema_version, seed, output_root, run_dir, started_at_unix, max_frames).

## Important functions
- `RunManifest::validate`: checks non-negative start time and max_frames > 0 if present.

## Invariants / Gotchas
- Maintain schema_version when evolving manifest; extend enum carefully.
- Paths (output_root/run_dir) are opaque PathBufs; validation is minimal.

## Cross-module deps
- Written/read by recorder/tools; consumed by ETL/training for run context.
