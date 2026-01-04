# Documentation index

Contributor-focused mdBook (user book retired):
- **Contributor guide**: `docs/contributor_book` (architecture, crates, hooks, testing, migration).
- User-facing flows now live in the app repository (`colon_sim` reference app: https://github.com/via-balaena/Deep-Poo).

Build/read:
```bash
mdbook build docs/contributor_book
```

Repository map (docs-relevant bits):
- `sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`: substrate crates.
- `models`, `training`, `inference`: detectors, training/eval, detector factory.
- `tools`: CLI utilities (overlay/prune/etl/cmd/scheduler/tui/single_infer) consumed by apps.
- `docs/contributor_book/`: mdBook.

Quick doc pointers:
- Architecture/crate internals/hooks/testing: `docs/contributor_book/src/*.md`.
