# inference

## Overview
Detector factory and inference plugin for runtime use.

## Usage
Build `InferenceFactory` with a checkpoint and inject into your app; docs.rs: https://docs.rs/cortenforge-inference; source: https://github.com/via-balaena/CortenForge/tree/main/crates/inference.

## Pitfalls
Heuristic fallback is used if weights are missing; see Known stubs.

## Known stubs
- `InferencePlugin` is a stub that seeds state; real runtime scheduling lives in `vision_runtime`.
- `InferenceFactory` falls back to a heuristic detector if weights are missing or fail to load.
