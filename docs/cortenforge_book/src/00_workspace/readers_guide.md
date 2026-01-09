# Reader’s Guide
**Why**: Get a gentle path through the book without overwhelm.
**How it fits**: Use this when you only have a few minutes.
**Learn more**: Start with [Building Apps](../20_building_apps/README.md).

Use this book to get up to speed quickly on the CortenForge crates.

## If you have 10 minutes
1) Read `Workspace Overview` to see what exists.
2) Skim `Canonical Flows` to see end-to-end paths (capture → train → inference).
3) Open [Crate Overview](../10_crates/README.md) and read the crate you care about most.

Naming note: package names use hyphens (e.g., `cortenforge-tools`), while Rust crate imports use underscores (e.g., `cortenforge_tools`).

## If you’re diving into a crate
1) Start in [Crate Overview](../10_crates/README.md) and read the crate’s README.
2) If you need depth, jump to [Crate Deep Dives](../10_crates/deep_dives.md) for public API, lifecycle, and design notes.
3) Use docs.rs for exact signatures.

## If you’re building an app
1) Read `Building Apps` in order (Step 1 → Step 9).
2) Follow the story to add one small piece at a time.
3) Jump into crate pages only when you need implementation detail.

## For architecture/flow questions
- `canonical_flows.md`: how crates stitch together.
- `workspace_metadata.md`: workspace-wide resolver, patch overrides, and feature policy.
- `integration_contracts.md`: shared types, feature expectations, runtime assumptions.

## For docs maintenance
1) Follow `quality_gates.md` to keep pages consistent.
2) See `maintenance_routine.md` for weekly release hygiene.

## Links and source
- Prefer repo-relative source links with line anchors (`crate/src/module.rs:L123`); see `linking_style.md`.
- docs.rs links are supplementary for exact signatures.
