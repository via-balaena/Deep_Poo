# sim_core: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide Bevy-based scaffolding for CortenForge apps: building the runtime, configuring mode sets (Common/SimDatagen/Inference), and wiring controls/autopilot and recorder plumbing without embedding detector or domain logic.

## Scope
- App builder entrypoints (`build_app`), Bevy plugins (`SimPlugin`, `SimRuntimePlugin`).
- Runtime mode sets and hooks (`SimHooks`, `ControlsHook`, `AutopilotHook`).
- Recorder types/resources (config/state/motion/world/meta providers) and default sink integration points.

## Non-goals
- No detector wiring or vision runtime; those live in `vision_runtime`/`inference`.
- No domain/world systems; apps supply their own Bevy plugins/systems.
- No app-specific tools or schemas.

## Who should use it
- App authors wiring the CortenForge substrate into their own Bevy app (e.g., colon_sim repo).
- Contributors extending runtime hooks or recorder scaffolding while keeping the core domain-agnostic.

## Links
- Source: `sim_core/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge-sim-core
