# Overview

What this is:
- A modular simulation substrate (CortenForge) distributed as shared crates for runtime orchestration, capture/inference, ETL, training, and tooling.
- Library-only: apps (e.g., `colon_sim`, demos) live in their own repos.
- A map for contributors: where code lives, how pieces talk, and how to change or extend behavior safely.

Versions/features (current target `0.1.1`):
- Crates on crates.io: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-training`, `cortenforge-inference`, `cortenforge-cli-support`, `cortenforge-burn-dataset`, `cortenforge` (umbrella).
- Feature flags:
  - Training/inference: `backend-wgpu` for GPU (default NdArray).
  - Inference: `tinydet` / `bigdet`.
  - Tools: `scheduler`, `tui`, `gpu_nvidia` (tools crate not published by default).
- MSRV: Rust 1.75+.
- Note: `burn-core` is temporarily patched to a vendored 0.14.0 due to a bincode publish break; drop the patch once upstream releases a fixed version.

Who should read this:
- New contributors ramping on architecture and conventions.
- Engineers adding runtime/vision/recorder/tools features.
- App authors wiring the substrate into their own repos (app-side docs are in those repos).

How to use this book:
- Start with **Architecture & Crate Map** for the substrate vs. app split and runtime/data flow.
- Jump to **Crate Deep Dives** for responsibilities, features, and “does/doesn’t.”
- Use **Hooks / Extension Points** when wiring new behavior.
- See **Runtime & Pipelines**, **Tools & CLI**, **Testing/CI**, and **Release & Publishing** for day-to-day work.
- Check **Troubleshooting**, **Roadmap**, and **Migration Notes** for current gaps and history.

Scope:
- In scope: architecture, crate responsibilities, extension points, app wiring, tools, testing/CI, release/publishing, and migration guidance.
- Out of scope: end-user gameplay flows (maintained in app repos), licensing specifics (see `COMMERCIAL_LICENSE.md`), exhaustive API docs (read the code; this book points you there).

Repo map (at a glance):
- `sim_core/`, `vision_core/`, `vision_runtime/`, `data_contracts/`, `capture_utils/`, `models/`, `training/`, `inference/`, `tools/`: substrate crates.
- `crates/*`: supporting libraries (cli_support, burn_dataset, cortenforge umbrella).
- App crates live elsewhere (see app repos).
- `docs/contributor_book/`: mdBook (run `mdbook build docs/contributor_book`).

Conventions:
- Keep core crates domain-agnostic and detector-free; apps supply world/systems.
- Favor small, composable surfaces (SimHooks, recorder meta/world state, vision hooks).
- Prefer defaults and clear wiring over deep abstraction; gate heavy deps behind features.
- Standardize shared third-party deps via root `[workspace.dependencies]`; prefer `workspace = true` in member crates.
