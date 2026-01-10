# Reproducibility
**Why**: Ensure builds and releases are consistent across machines.
**How it fits**: Locks versions and documents expectations for stable outputs.
**Learn more**: See [Build & Run](build_and_run.md).

## Lockfile policy
How we keep local builds and publishes deterministic.
- Use the workspace `Cargo.lock` for local builds/tests; enforce with `--locked` before publish.
    - Note: published crates resolve without a lockfile; burn-core 0.19.1 avoids the prior bincode publish break (bincode remains at 2.0.1 until 3.x is real).

## MSRV
Minimum Rust version and how to keep it consistent.
- Target Rust 1.75+ across crates (umbrella uses 2024 edition).
    - Keep MSRV aligned in docs/metadata; bump intentionally.

## CI expectations
Baseline checks to keep CI and local runs aligned.

| Check | Command |
| --- | --- |
| fmt | `cargo fmt -- --check` |
| clippy | `cargo clippy --workspace --all-targets --all-features -D warnings` |
| tests | `cargo test --workspace --locked` |
| optional | `cargo deny check`, `cargo hakari generate && cargo hakari manage-deps` |
| docs | `mdbook build docs/cortenforge_book`; `mdbook test` for doctests. |

## Deterministic builds
Habits that keep builds repeatable across machines.
1) Use `--locked` to pin deps; avoid adding `path` patches except for local dev needs.
2) Document feature sets used in builds (NdArray default; GPU/WGPU opt-in).
3) Avoid network fetches in tests; keep fixtures small and included.
