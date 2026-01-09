# inference

**Why**: The runtime detector factory.
**How it fits**: Loads checkpoints and runs detectors in the sim.
**Learn more**: Use the pages below; docs.rs: https://docs.rs/cortenforge-inference.

## Known stubs
- `InferencePlugin` is a stub that seeds state; real runtime scheduling lives in `vision_runtime`.
- `InferenceFactory` falls back to a heuristic detector if weights are missing or fail to load.
