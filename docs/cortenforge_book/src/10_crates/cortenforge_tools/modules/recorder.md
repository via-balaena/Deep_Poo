# recorder (cortenforge-tools)

## Responsibility
- Re-export recording/overlay/prune helpers from `capture_utils` for use in tooling binaries.

## Key items
- Re-exports: `JsonRecorder`, `generate_overlays`, `prune_run`.

## Invariants / Gotchas
- Semantics come from `capture_utils`; ensure paths/output roots align with capture dataset layout.

## Cross-module deps
- Used by CLI bins to write overlays or prune empty-label frames after datagen.
