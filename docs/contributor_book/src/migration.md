# Migration notes

Guidance for working in the current layout and moving code where it belongs.

## Snapshot (current state)
- Repo is library-only: shared crates (`sim_core`, `vision_core`/`vision_runtime`, `data_contracts`, `capture_utils`, `models`, `training`, `inference`, `colon_sim_tools`) intended for crates.io.
- Apps (e.g., `colon_sim`, `hello_substrate`) live in their own repo: https://github.com/via-balaena/Deep-Poo.
- Crates target: `0.1.1` (burn-core temporarily patched to vendored 0.14.0 until upstream fixes bincode).
- Tools live in `colon_sim_tools`; bins reuse helpers via `cortenforge-cli-support` and `colon_sim_tools::services`. Plan to split app-specific bins into app repos.
- Recorder defaults to `JsonRecorder`; apps supply metadata/world-state hooks and can inject sinks.
- Branding: substrate = “CortenForge”; apps consume it.
- See `MIGRATION.md` at repo root for detailed steps and notes.

## Porting a feature
1) Decide if it’s substrate (generic) or app (domain-specific).
2) If generic, add hooks/helpers to core crates; gate heavy deps with features.
3) If app-only, implement in the app repo and wire via app hooks/plugins.
4) Update docs (contributor book) and add a smoke test (NdArray) if applicable.

## PR checklist
- Docs updated (contributor book).
- Defaults/CLI examples verified.
- Tests: `cargo check --workspace`; add feature-gated tests if new features introduced.

## Adding a new app
- Use the app repo as a template; wire hooks, bins, and tests there.

## Extending tools
- Put helpers in `tools/src/services.rs` or `tools/src/warehouse_commands/`; keep bins thin; gate heavy deps with features. Split app-specific tooling into app repos when feasible.
