# Documentation index

Single source of truth:
- **CortenForge book**: `docs/cortenforge_book` (workspace overview, guided app building, crate deep dives).
- App-facing flows live in the app repository (reference app: https://github.com/via-balaena/Deep-Poo).

Build/read:
```bash
mdbook build docs/cortenforge_book
```

Repository map (docs-relevant bits):
- `sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`: substrate crates.
- `models`, `training`, `inference`: detectors, training/eval, detector factory.
- `tools`: CLI utilities (overlay/prune/etl/cmd/scheduler/tui/single_infer) consumed by apps.
- `docs/cortenforge_book/`: mdBook.

Quick doc pointers:
- Start here: `docs/cortenforge_book/src/README.md`.
