# cortenforge (umbrella): Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
- Add dependency and select features instead of pinning individual crates:
  ```toml
  [dependencies]
  cortenforge = { version = "0.3.0", features = ["sim-core", "vision-core", "vision-runtime", "training", "inference", "models", "capture-utils"] }
  ```
- Import from prelude or modules as needed; underlying crates are re-exported.

## Execution flow
- Consumer opts into feature sets (e.g., `sim-core`, `vision-runtime`, `training`, `inference`, `models`).
- The umbrella exposes the member crates; lifecycle is managed by those crates. This crate is a facade only.

## Notes
- Keep features aligned with member crates; burn-core 0.19.1 avoids the prior publish break.
