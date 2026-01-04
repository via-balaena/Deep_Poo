# colon_sim_tools: split/retire plan

Goal: phase out app-specific code from `colon_sim_tools` and keep only app-agnostic utilities here. Move domain-specific bins/logic into the app repo (e.g., Deep-Poo).

## What’s inside (current)
- Binaries under `tools/src/bin/`:
  - Shared-ish: `overlay_labels`, `prune_empty`, `warehouse_etl`, `warehouse_export`, `warehouse_cmd`, `single_infer`.
  - App-facing: `datagen_scheduler`, `tui`, and any bins with app/domain assumptions.
- Shared helpers under `tools/src/services.rs` and `tools/src/warehouse_commands/`.
- Feature flags: `tui`, `scheduler`, `gpu_nvidia` gate heavier deps and app-specific functionality.

## What to keep (app-agnostic)
- Shared services: CLI parsing, warehouse command builders, overlay/prune helpers that only depend on substrate crates (`capture_utils`, `data_contracts`, `vision_core`).
- Bins that operate purely on captures/warehouse artifacts without domain logic: `overlay_labels`, `prune_empty`, `warehouse_etl`, `warehouse_export`, `warehouse_cmd`, `single_infer` (if detector selection remains generic).

## What to move to app repo
- Bins that assume app/world context: `datagen_scheduler`, `tui`, any bin that pulls app configs or domain assets.
- Any code paths that reference app-specific schemas, asset layouts, or controls.
- Feature-flagged GPU/app logic (`gpu_nvidia` telemetry, app-specific schedulers/UI).

## Roadmap to split/retire
1) Inventory bins and helpers:
   - Tag each bin as “shared” vs “app-specific”.
   - Identify helper functions/types that are only used by app-specific bins.
2) Extract app-specific code to the app repo:
   - Create equivalent bins/modules in the app repo.
   - Copy/move app-only helpers; adjust imports to use app crates.
3) Slim `colon_sim_tools` to shared utilities:
   - Remove app-specific bins/features (`tui`, `scheduler`, `gpu_nvidia`) or gate them behind a new “app” feature that defaults off.
   - Update `Cargo.toml` to reflect unpublished status and clean feature list.
4) Refactor shared bins to be app-agnostic:
   - Ensure inputs are captures/warehouse manifests; no app configs.
   - Keep schemas aligned with `data_contracts`; minimize hardcoded paths.
5) Update docs and release notes:
   - Contributor book: note what remains in `colon_sim_tools` and what moved to app repo.
   - README: clarify that app-specific tooling lives in the app repo.
6) Clean up tests:
   - Move app-specific tests/fixtures to the app repo.
   - Keep shared CLI smokes in `tools/tests/` with synthetic fixtures.
7) Optional: rename the shared tools crate once app-specific pieces are gone (e.g., `cortenforge-tools`), or keep name but document scope.

## Notes
- Do not publish `colon_sim_tools` to crates.io in its current form.
- Keep feature-gated heavy deps off by default; align with shared-only scope.
