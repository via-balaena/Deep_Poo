# Migration

Guidance for moving code/features into the current layout and branding.

## Refactor snapshot
- Root crate is orchestration-only (`src/cli/*`, `run_app`); domain systems live in app crates under `apps/`.
- Core crates are domain-agnostic (sim_core, vision_core/runtime, data_contracts, capture_utils, models, training, inference).
- Tools live in `colon_sim_tools`; bins reuse shared helpers via `colon_sim::cli` and `colon_sim_tools::services`.
- Recorder defaults to `JsonRecorder`; apps supply metadata/world-state hooks and can inject sinks.
- Reference app bins live under `apps/colon_sim/bin`; minimal demo at `apps/hello_substrate`.
- Branding: substrate is “CortenForge”; app crates consume it.
- See `MIGRATION.md` at repo root for detailed steps and notes.

## Porting a feature to the new layout
1) Decide if it belongs in substrate (generic) or app (domain-specific).
2) If generic, add hooks/helpers to core crates; gate heavy deps with features.
3) If app-only, put code under `apps/your_app/src` and register via hooks/plugins.
4) Update docs (user/contributor) and add a smoke test (NdArray) if applicable.

## PR checklist
- Docs updated (user and/or contributor book).
- Defaults and CLI examples verified.
- Tests: `cargo check --workspace`; add feature-gated tests if new features introduced.

## Adding a new app
- Start from `apps/hello_substrate` layout; wire hooks; add bins; add a short README and a smoke test.

## Extending tools
- Put helpers in `tools/src/services.rs` or `tools/src/warehouse_commands/`; keep bins thin; gate heavy deps with features.
