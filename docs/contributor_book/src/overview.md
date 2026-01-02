# Overview

Welcome to the contributor guide for CortenForge. This book explains how the substrate and example apps fit together, what design choices were made, and how to extend or modify the system without getting lost.

## What this is
- A modular simulation substrate (CortenForge) plus example apps (`colon_sim` reference, `hello_substrate` minimal).
- Shared crates for runtime orchestration, capture/inference, ETL, training, and tooling.
- A map for contributors: where code lives, how pieces talk, and how to add or change behavior safely.

## Who should read this
- New contributors ramping up on architecture and conventions.
- Engineers adding features (runtime hooks, vision/capture, recorder sinks, tools).
- Domain authors building a new app on the substrate or adapting the reference app.

## How to use this book
- Start with **Introduction** for scope and expectations.
- Read **Architecture** for the substrate vs. app split and the runtime/data flow.
- Jump to the chapter that matches your work:
  - **Core crates** if you’re inside shared runtime/vision/recorder code.
  - **Hooks / extension points** when wiring new behavior into the sim loop or recorder.
  - **Apps** for patterns and the reference app tour.
  - **Tools crate** for CLI utilities and adding new commands.
  - **Testing** / **CI** for validation and pipelines.
  - **Roadmap** for upcoming changes and migration notes.

## Scope
- In scope: architecture, crate responsibilities, extension points, app wiring, tools, testing/CI, and migration guidance.
- Out of scope: end-user gameplay instructions (see user book), hardware/patent licensing specifics (see `COMMERCIAL_LICENSE.md`), and exhaustive API docs (read the code; this book points you there).

## Repo map (at a glance)
- `src/`: CLI + orchestrator (`run_app`), no domain systems.
- `sim_core/`, `vision_core/`, `vision_runtime/`, `data_contracts/`, `capture_utils/`, `models/`, `training/`, `inference/`, `tools/`: substrate crates.
- `apps/colon_sim/`, `apps/hello_substrate/`: example apps.
- `docs/user_book/`, `docs/contributor_book/`: mdBooks (run `mdbook build docs/contributor_book`).

## Conventions
- Keep core crates domain-agnostic and detector-free; apps supply domain systems and sinks.
- Favor small, composable surfaces (SimHooks, recorder meta/world state, vision hooks).
- Prefer defaults and clear wiring over deep abstraction; gate heavy deps behind features.
