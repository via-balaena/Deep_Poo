# CI

What runs in CI, how to mirror it locally, and expectations for contributors. Adjust specifics to your actual pipeline, but keep the split between fast defaults and opt-in heavy jobs.

## Pipeline shape (recommended)
- Fast lane (default, required):
  - `cargo fmt -- --check`
  - `cargo clippy --workspace --all-targets --all-features -D warnings` (or with minimal features if clippy is too heavy; keep consistent)
  - `cargo check --workspace`
  - `cargo test --workspace` (NdArray/backends only, no GPU requirements)
  - mdBook lint/build: `mdbook test docs/user_book` and `mdbook test docs/contributor_book` (or at least `mdbook build`)
- Opt-in/heavy lane (manual or nightly):
  - `cargo test --workspace --all-features` (scheduler/tui/gpu_nvidia/burn_wgpu)
  - GPU/WGPU smoke (if runners have GPUs): minimal inference/capture smoke with WGPU backend.
  - Larger dataset integration (if applicable): behind a feature flag and separate job.

## Local reproduction
- Fast pass: `cargo fmt -- --check && cargo clippy --workspace --all-targets -D warnings && cargo test --workspace`
- Docs: `mdbook build docs/user_book && mdbook build docs/contributor_book`
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
