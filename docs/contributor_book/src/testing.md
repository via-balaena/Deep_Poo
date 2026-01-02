# Testing

Expectations for contributors: fast-by-default tests, heavier paths gated behind features, and clear smoke coverage.

## Defaults
- Primary backend: NdArray for portability and speed.
- WGPU/GPU-heavy tests: feature-gated; do not run by default.
- Keep runtimes small; avoid large assets in unit tests.

## Coverage targets
- Recorder sinks (`capture_utils`): JSON label writes.
- Models: TinyDet/BigDet forward-shape and smoke train (NdArray).
- Inference factory: heuristic fallback smoke; Burn load path if weights are present (feature-gated if GPU).
- Tools: basic CLI smoke for overlay/prune/warehouse_cmd/warehouse_etl/single_infer (minimal args).

## Recommended commands
- Fast pass: `cargo check --workspace`
- Default tests: `cargo test --workspace`
- Opt-in sweep: `cargo test --workspace --all-features` (enables scheduler/tui/gpu_nvidia/burn_wgpu where available)
- Manual smokes: run bins with defaults on small inputs (`sim_view`, `warehouse_etl`, `train`, `inference_view`).

## Adding tests
- Prefer synthetic fixtures; avoid touching large datasets.
- Gate GPU/WGPU paths behind features; keep NdArray as the default.
- Mirror user-facing defaults in CLI tests; minimize required args.
- Place tool tests under `tools/tests/`, app tests under app crates, core tests in their crates.

## CI alignment
- Default CI should mirror the default matrix (NdArray-only).
- Provide an opt-in job/profile for `--all-features` if runners support GPUs.
- Keep flaky/heavy tests out of the default path; document how to run them manually.
