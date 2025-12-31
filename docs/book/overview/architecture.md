# Architecture: substrate + apps

This codebase splits into a reusable substrate (soon branded as “pipelinea”) and app crates that build worlds on top of it (e.g., `apps/colon_sim`, `apps/hello_substrate`). The substrate owns the plumbing for sim runtime, capture, inference, data schemas, and tools; apps supply domain systems (entities, HUD, controls, autopilot, recorder world-state).

## Layers at a glance
- **Substrate (pipelinea) crates**
  - `sim_core`: Bevy runtime plumbing; mode sets (`ModeSet`), config (`SimConfig`), `SimPlugin`, `SimRuntimePlugin`, hooks for controls/autopilot/recorder metadata, recorder types.
  - `vision_core`: Detector/capture/overlay interfaces (`Frame`, `DetectionResult`, `Detector`, `Recorder`), overlay helpers, `CaptureLimit`.
  - `vision_runtime`: Bevy plugins for capture (camera/readback) and inference (detector handle, overlay state), integrates with `SimRunMode`.
  - `data_contracts`: Capture/manifest schemas (serde) + validation helpers.
  - `models`: Detector models (TinyDet, BigDet) + configs.
  - `inference`: Burn-backed detector factory (loads checkpoints, heuristic fallback).
  - `training`: Burn training/eval CLI + dataset loaders (warehouse/manifests) using `data_contracts` and `models`.
  - `capture_utils`: Default recorder sink (`JsonRecorder`), prune/filter helpers, overlay generation.
  - `colon_sim_tools`: CLI utilities (overlay/prune, warehouse_etl/export/cmd, datagen scheduler, tui, single_infer) built on shared helpers.
- **App crates**
  - `apps/colon_sim`: Reference world/entities, HUD, controls/autopilot hooks, recorder world-state updater; bins `sim_view` / `inference_view` live here.
  - `apps/hello_substrate`: Minimal demo app showing how to plug a custom plugin into `sim_core` without colon-specific systems.
- **Root crate (`colon_sim`)**
  - Orchestration/CLI glue only: `src/cli/*`, `run_app` wires `SimConfig` + core plugins + app hooks; no domain systems.
  - Bins point to the colon_sim app by default (`sim_view`, `inference_view`).

## Runtime flow
1) `AppArgs` parsed in a bin → set `RunMode` (Sim/Datagen/Inference) → `run_app`.
2) `run_app` builds a Bevy app via `sim_core::build_app` with `SimConfig` (mode, headless, capture roots, prune flags, max frames).
3) Add core plugins: `SimPlugin`, `SimRuntimePlugin`, capture/inference plugins from `vision_runtime`.
4) App crate adds its systems via hooks/plugins (controls/autopilot, HUD, recorder world-state updates).
5) Recorder runs in the substrate with a default `JsonRecorder`; apps can inject custom sinks.
6) Tools consume the same schemas/helpers (`data_contracts`, `capture_utils`, `vision_core`) for ETL/export/single inference.

## Principles we want to preserve
- Solve real flows: capture → ETL → train → infer; keep “why” obvious in docs.
- Hide complexity: sensible defaults and happy-path commands; avoid leaking Bevy/Burn internals.
- Small surfaces: SimHooks + recorder metadata/world-state are the primary extension points.
- No gratuitous abstractions: add traits only to remove duplication.
- Lean by default: gate heavy deps (burn_wgpu, tui/scheduler/gpu_*) and keep fast NdArray tests by default.
- Pragmatic tone: clear docs/examples; no “tech priesthood” required.

## Extending / building your own app
- Create an app crate under `apps/your_app` with your systems and plugins.
- Implement hooks from `sim_core` (controls/autopilot) and recorder metadata/world-state updates.
- Add bins under your app crate (e.g., `bin/sim_view.rs`, `bin/inference_view.rs`) that parse args and call `run_app` or a thin wrapper.
- Keep the root crate glue-only; avoid pulling domain systems into core crates.

## Migration note
- See `MIGRATION.md` for a summary of the refactor: root as orchestrator, app crates under `apps/`, tools moved to `colon_sim_tools`, recorder default sink in substrate, and tests/docs refreshed.
