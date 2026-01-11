# sim_core

[![crates.io](https://img.shields.io/crates/v/cortenforge-sim-core.svg)](https://crates.io/crates/cortenforge-sim-core) [![docs.rs](https://docs.rs/cortenforge-sim-core/badge.svg)](https://docs.rs/cortenforge-sim-core) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

Bevy runtime scaffolding, hooks, and recorder types shared by sim/datagen/inference apps. It owns:
- ModeSet + SimRunMode: system sets for common vs sim/datagen vs inference paths.
- SimConfig: mode, headless, capture output/prune settings, max_frames, optional capture interval.
- Plugins: SimPlugin (mode sets), SimRuntimePlugin (default runtime resources); app crates wire their own systems (e.g., their `AppSystemsPlugin`).
- Modules: camera/controls, autopilot_types/recorder_types, probe_types.

How to use (in your app repo)
1) Build the base app via `sim_core::build_app(SimConfig { .. })`.
2) Add plugins: `SimPlugin`, `SimRuntimePlugin`, plus your app systems plugin and any app-specific bootstrap (e.g., environment).
3) If you need inference, add detector resources/systems in an inference-only branch (ModeSet::Inference).
4) Bins set `SimRunMode` through CLI (sim/datagen/inference) and pass headless/output/prune/max_frames via `SimConfig`.

Adding systems
- Common systems: add to ModeSet::Common.
- Sim/datagen-only: add to ModeSet::SimDatagen.
- Inference-only: add to ModeSet::Inference.
Use `SimRuntimePlugin` to keep registration in one place; avoid detector wiring here to keep the core crate lean. Recorder metadata/sink/world-state live here (`recorder_meta`); apps provide world-state updates and can inject custom sinks.

> Deprecated: the old `sim_core` crate name was renamed to `cortenforge-sim-core`. Please depend on the new crate name.

## License
Apache-2.0 (see `LICENSE` in the repo root).
