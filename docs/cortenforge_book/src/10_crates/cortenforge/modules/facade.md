# facade (cortenforge)

## Responsibility
- Minimal umbrella crate that re-exports other workspace crates behind features.

## Key items
- `pub use` gates for each crate: `sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`, `models`, `inference`, `training`, `burn_dataset`, `cli_support`.
- Each re-export is behind a crate-named feature (e.g., `sim-core`, `vision-core`, ...).

## Invariants / Gotchas
- No own logic; purely a facade. Ensuring feature names match downstream consumers is critical.
- Only crates whose feature is enabled are re-exported; cargo feature selection controls visibility.
- Keep this crate in sync with workspace membership when adding/removing crates.

## Cross-module deps
- Depends on all re-exported crates via optional features; provides a single dependency handle for consumers that prefer one umbrella crate.
