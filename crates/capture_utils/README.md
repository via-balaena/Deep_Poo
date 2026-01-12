# capture_utils

[![crates.io](https://img.shields.io/crates/v/cortenforge-capture-utils.svg)](https://crates.io/crates/cortenforge-capture-utils) [![docs.rs](https://docs.rs/cortenforge-capture-utils/badge.svg)](https://docs.rs/cortenforge-capture-utils) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

Recorder sinks and capture helpers for capture runs and overlays:
- `JsonRecorder` writes frame metadata/labels to disk under `run_dir/labels/frame_XXXXX.json` (uses `data_contracts::CaptureMetadata`).
- `generate_overlays` renders boxes onto PNGs in a run directory (`overlays/`).
- `prune_run` copies a run to a filtered destination, skipping unwanted frames.

Usage
- Add `capture_utils` as a dependency and construct `JsonRecorder` for default file-based recording.
- Recorder sinks are pluggable: the substrate recorder installs `JsonRecorder` by default; you can inject your own sink implementing `vision_core::Recorder`.


## License
Apache-2.0 (see `LICENSE` in the repo root).
