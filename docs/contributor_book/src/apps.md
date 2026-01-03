# App patterns

How apps sit on the substrate, with a reference tour and a starter template.

## Reference app: `apps/colon_sim` (Deep Poo)
- Domain systems: world/entities, HUD, controls/autopilot, recorder world-state updates.
- Bins: `sim_view`, `inference_view` under `apps/colon_sim/bin`, call `run_app` with plugins/hook resources.
- Uses `SimHooks` to register controls/autopilot; updates `RecorderWorldState` and meta; relies on default JSON sink.
- Good for: seeing a full integration of capture + inference + recorder + UI.

## Minimal demo: `apps/hello_substrate`
- Tiny plugin that adds a system to the substrate without domain code.
- Good for: a clean starter layout and minimal bin wiring.

## Build your own app
- Create `apps/your_app` with:
  - `src/lib.rs`: plugins + systems.
  - `src/prelude.rs`: re-exports for bins/tests.
  - `bin/sim_view.rs` / `bin/inference_view.rs`: CLI → `run_app` (reuse `colon_sim::cli` if helpful).
- Wiring steps:
  1) Start from `apps/hello_substrate` as a template.
  2) Add your world/entities and systems; register controls/autopilot via `SimHooks`.
  3) Add a system to update `RecorderWorldState`; provide `RecorderMetaProvider` if you need custom metadata.
  4) Optionally insert custom recorder sinks (keep schemas compatible with data_contracts).
  5) Include capture (`vision_runtime::CapturePlugin`) and inference plugins as needed.
  6) Run `cargo check --workspace`; add a README describing systems/controls.
  7) Add a smoke test or CLI example to ensure the app builds after changes.
- Principles:
  - Keep domain logic in the app crate; core crates stay detector- and domain-agnostic.
  - Use `ModeSet`/`SimRunMode` to gate systems (e.g., only inference in inference mode).

## Bin wiring (example)
```rust
fn main() {
    let args = AppArgs::parse();
    run_app(args);
}
```
- `run_app` (root crate) builds the base app via `sim_core::build_app`, inserts `SimHooks`, recorder meta/world, adds `vision_runtime` plugins, and app plugins.
- Bins can be thin: parse args, maybe tweak defaults, then call `run_app`.
