# inference crate

[![crates.io](https://img.shields.io/crates/v/cortenforge-inference.svg)](https://crates.io/crates/cortenforge-inference) [![docs.rs](https://docs.rs/cortenforge-inference/badge.svg)](https://docs.rs/cortenforge-inference) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

- Purpose: provide the Burn-backed detector factory and inference plugin used by Bevy apps (sim_view/inference_view).
- Backend: defaults to `backend-ndarray`; enable `--features backend-wgpu` for WGPU. Needs `burn` features enabled in the root build if you want GPU.
- Model: loads `TinyDet` (default) or `BigDet` from the shared `models` crate via `BinFileRecorder` (full precision). Pass a weights path to the factory to load a checkpoint; otherwise it falls back to a heuristic detector.
- Use: app orchestrators insert the detector built by `inference::InferenceFactory` when mode==Inference. Ensure the checkpoint exists and matches the model config.
- Smoke: unit test ensures fallback when no weights are provided. Add an integration test pointing at a real checkpoint once available.

## License
Apache-2.0 (see `LICENSE` in the repo root).
