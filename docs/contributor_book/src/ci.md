# CI

What runs in CI, how to mirror it locally, and expectations for contributors. Keep a fast default lane and an opt-in heavy lane.

## Pipeline shape (recommended)
- Fast lane (default, required):
  - `cargo fmt -- --check`
  - `cargo clippy --workspace --all-targets --all-features -D warnings` (or minimal features if needed; keep consistent)
  - `cargo check --workspace`
  - `cargo test --workspace` (NdArray/backends only, no GPU requirements)
  - mdBook lint/build: `mdbook test docs/contributor_book` (or at least `mdbook build`)
  - Optional dep checks: `cargo deny check`, `cargo hakari generate && cargo hakari manage-deps`
- Opt-in/heavy lane (manual or nightly):
  - `cargo test --workspace --all-features` (scheduler/tui/gpu_nvidia/burn_wgpu)
  - GPU/WGPU smoke (if runners have GPUs): minimal inference/capture smoke with WGPU backend.
  - Larger dataset integration (if applicable): behind a feature flag and separate job.

## Local reproduction
- Fast pass: `cargo fmt -- --check && cargo clippy --workspace --all-targets -D warnings && cargo test --workspace`
- Docs: `mdbook build docs/contributor_book`
- Full sweep (optional): `cargo test --workspace --all-features`

## Expectations for contributors
- Keep default tests fast and backend-light; gate heavy/GPU paths behind features.
- Ensure fmt/clippy pass with the same flags CI uses.
- Document any new feature flags or env vars needed for tests.
- Avoid adding flaky or long-running tests to the default lane; move them to the opt-in lane.

## Troubleshooting
- CI-only clippy/fmt failures: match the CI command flags locally.
- Feature-related test failures: verify feature flags and backend availability; default to NdArray.
- WGPU/GPU issues: ensure the test is feature-gated; skip on non-GPU runners; provide a manual reproduction command.
