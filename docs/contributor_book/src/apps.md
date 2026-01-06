# Troubleshooting

Common pitfalls and fixes when building/publishing with the substrate.

## Burn-core / bincode publish failure
- Symptom: `cargo publish --dry-run -p burn-core` (or any dependent crate) fails with `cannot find function decode_borrowed_from_slice` in `bincode::serde`.
- Cause: `burn-core 0.14.0` pulls `bincode 2.0.1` when there is no lockfile; that API was removed.
- Fix (temporary): vendor/patch `burn-core 0.14.0`; drop the patch once Burn publishes a fixed release (or upstream pins bincode exact).
- Repro to share upstream: `git checkout v0.14.0 && rm Cargo.lock && cargo publish --dry-run -p burn-core`.

## Cargo.lock masking issues
- Libraries don’t ship lockfiles; `cargo publish` re-resolves deps. If a build works only with `Cargo.lock`, reproduce without it to mirror publish behavior.

## GPU/WGPU issues
- Symptom: WGPU init failures or GPU-only deps failing CI.
- Fix: gate GPU paths behind features (`backend-wgpu`, `gpu_nvidia`), default to NdArray; skip GPU tests on non-GPU runners. Provide a manual repro command with the feature flags.

## CLI/tool failures
- Verify schemas: ensure outputs still match `data_contracts`.
- Run minimal smoke: `overlay_labels`, `prune_empty`, `warehouse_etl` on small fixtures.
- Keep feature flags minimal in tests; only enable `tui`/`scheduler`/`gpu_nvidia` when needed.

## Recorder issues
- Missing meta/world state: ensure app inserts `RecorderMetaProvider` and updates `RecorderWorldState`.
- Custom sinks: verify they implement `RecorderSink` and preserve schema compatibility for ETL/training.

## Documentation build
- If docs fail: `mdbook build docs/contributor_book`; install `mdbook-mermaid` (`cargo install mdbook-mermaid`) for Mermaid diagrams.

## Debugging core paths (quick pointers)
- Recorder failures: log meta/world state values; validate against `data_contracts`; ensure sinks are registered.
- Runtime hangs: check mode gating (`SimRunMode`/`ModeSet`), system ordering, and that plugins are added.
- Dataset issues: run `warehouse_etl` on a tiny capture; validate manifests with `data_contracts`; inspect schema versions.
