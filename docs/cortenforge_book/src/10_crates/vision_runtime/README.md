# vision_runtime

## Overview
Bevy capture/inference plugins built on `vision_core`.

## Usage
Add `CapturePlugin` and `InferencePlugin` with `SimRunMode`; docs.rs: https://docs.rs/cortenforge-vision-runtime; source: https://github.com/via-balaena/CortenForge/tree/main/crates/vision_runtime.

## Pitfalls
Heuristic fallback is used if weights are missing; see Known stubs.

## Known stubs
- Inference can fall back to heuristic detectors when checkpoints are missing or invalid.
