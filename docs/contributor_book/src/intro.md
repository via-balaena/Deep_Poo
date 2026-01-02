# Introduction

This book is for contributors to the CortenForge substrate and its example apps. It explains how the pieces fit, why the architecture looks the way it does, and how to extend the system safely.

What to expect:
- Architecture and data flow: substrate vs. apps, runtime wiring, capture/inference/ETL/training loop.
- Core crates: responsibilities and boundaries.
- Hooks / extension points: where to plug in controls, autopilot, recorder meta/world state, and vision capture/inference hooks.
- Apps: patterns, reference app tour (`colon_sim`), and how to build your own.
- Tooling: CLI utilities, adding new tools, feature flags.
- Testing and CI: what to run, backends, fixtures, and how pipelines are structured.
- Roadmap and migration notes: upcoming changes, version bumps, deprecations.

Housekeeping:
- Build this book: `mdbook build docs/contributor_book`
- Build the user book (end-user flows): `mdbook build docs/user_book`
- Recent refactor highlights: see `MIGRATION.md` (root orchestrator, app crates under `apps/`, tools split, recorder defaults, tests/docs refresh).
