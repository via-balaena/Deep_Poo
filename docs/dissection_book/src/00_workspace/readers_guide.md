# Reader’s Guide

Use this book to get up to speed quickly on the CortenForge crates.

## If you have 10 minutes
- Read `Workspace Overview` and `Crate Index` to see what exists.
- Skim `Canonical Flows` to understand end-to-end paths (capture → train → inference).
- Check `integration_contracts.md` for key assumptions and feature gates.

## If you’re diving into a crate
1) Go to the crate’s chapter and read `01_overview` → `02_public_api` → `03_lifecycle`.
2) Consult `05_traits_and_generics`, `06_error_model`, and `07_ownership_and_concurrency` for extension points and safety.
3) Run the snippets in `09_examples` as a sanity check; skim `10_design_review` for pitfalls.

## For architecture/flow questions
- `canonical_flows.md`: how crates stitch together.
- `dependency_graph.md`: who depends on whom, core vs. leaf crates.
- `workspace_metadata.md`: workspace-wide resolver, patch overrides, and feature policy.
- `integration_contracts.md`: shared types, feature expectations, runtime assumptions.

## For docs maintenance
- Follow `quality_gates.md` to keep pages consistent.
- See `maintenance_routine.md` for weekly release hygiene.
- Use `how_to_feed_codex.md` when asking Codex to update or regenerate sections.

## Links and source
- Prefer repo-relative source links with line anchors (`crate/src/module.rs:L123`); see `linking_style.md`.
- docs.rs links are supplementary for exact signatures.
