# ML/Data Prep Refactor Plan

## 1) Config & CLI
- Add a small CLI (e.g., `clap`) for run mode (`sim`/`datagen`), seed, output root, frames-to-capture, headless toggle.
- Centralize defaults in a config file (e.g., `config/datagen.toml`) and log seed/settings at startup.

## 2) Data Layout & Manifest ✅
- Standardize run structure: `run_<timestamp>/images`, `labels`, `overlays`.
- Add `run_manifest.json` capturing seed, settings, counts, schema version, and capture metadata.
- Document schema in `docs/data_schema.md`.

## 3) Runtime vs Tooling Split
- Isolate recording/overlay code into a `vision/` or `tools/` module/feature gate.
- Keep overlay/training prep binaries under `tools/`/`scripts/`, separate from the main sim.

## 4) Headless Datagen Entry Point ✅
- `datagen` binary now forces headless mode, auto-starts autopilot+probe POV, and runs a full data capture without key presses.
- Captures honor an optional `--max-frames` cap, write `run_manifest.json`, auto-generate overlays, and exit on completion/cecum.
- Offscreen rendering sticks to the capture camera; no HUD window is shown when headless.

## 5) Vision Stack Interfaces ✅
- Added `src/vision_interfaces.rs` with `FrameSource`, `Detector`, `Recorder` traits and shared `Frame`/`DetectionResult` structs.
- Included optional `BurnDetectorFactory` behind a `burn_runtime` feature gate to keep the Burn runtime optional/decoupled.
- These interfaces will back the runtime swap between GPU capture, file replay, and future ML detectors.

## 6) Seeding & Determinism ✅
- Centralized seed resolution (`src/seed.rs`) from CLI/`POLYP_SEED`/time and stored in `SeedState`; the same seed feeds `PolypSpawnMeta`, `PolypRandom`, and manifests.
- Added a deterministic RNG check (`tests/seed_consistency.rs`) to assert identical sequences for the same seed.
- Manifest continues to log the seed; use `--seed` for reproducible layouts.

## 7) Testing & Validation ✅
- Added seed determinism test (`tests/seed_consistency.rs`) to ensure identical RNG sequences per seed.
- Added overlay/schema test (`tests/overlay_schema.rs`) to validate label JSON shape and overlay generation output.
- Added run-dir/manifest test inside `vision` to assert `init_run_dirs` writes required folders and manifest fields.
- Manual datagen smoke is still advised when changing rendering/capture, but core schema/manifest checks are now covered by tests.

## 8) Vision Interface Integration ✅
- Wired the detector and recorder paths through `vision_interfaces`: detection now uses a `DetectorHandle` boxed trait, and recording goes through a `Recorder` adapter (`DiskRecorder`) using the shared `Frame`/`Label`/`FrameRecord` types.
- Added `Label`/`FrameRecord` to the interfaces to carry capture + metadata cleanly for future Burn integration.

## 9) Seed E2E Check ✅
- Added an end-to-end seed determinism test (`tests/seed_e2e_positions.rs`) that spawns polyps twice with the same seed and asserts identical layouts.

## 10) Headless Datagen Smoke Test ✅
- Added `tests/datagen_smoke.rs` to run a 20-frame headless datagen and assert run dir + overlays + manifest creation. Guarded by `RUN_DATAGEN_SMOKE=1` to opt-in on GPU-capable machines.

## 11) Detector Feature Flag
- Decide whether to keep the heuristic detector default or gate Burn integration behind a feature flag for easy swapping.

## 12) Inline Datagen Harness
- Reduce the headless smoke dependency on `cargo run`; add an inline harness to avoid spawning Cargo during CI.

## 13) Visible Smoke
- Add a quick visible (non-headless) smoke to catch rendering regressions.
