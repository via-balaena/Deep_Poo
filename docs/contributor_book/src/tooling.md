# Contributing & code style

Expectations for contributions: style, linting, safety, docs, and review basics.

## Style & linting
- Rustfmt/clippy clean: required. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -D warnings`.
- Prefer explicit over clever; keep functions short and modules cohesive.
- Error handling: use structured errors; avoid `unwrap()`/`expect()` outside tests unless it’s a deliberate abort with context.
- Logging: minimal, meaningful logs; avoid noisy debug spam in core crates.

## Docs & comments
- Add doc-comments for public items that aren’t self-explanatory; keep them concise.
- Prefer examples in doc-comments where helpful; mark `no_run`/`ignore` as needed.
- Use `<details>` blocks in the book for long explanations; keep jargon minimal.

## Safety & features
- Gate heavy deps and GPU paths behind features; keep NdArray/defaults lean.
- Maintain schema compatibility (data_contracts) when changing recorder/pipeline paths.
- Keep core crates detector- and domain-agnostic; app logic lives in app repos.

## Tests
- Mirror the Testing chapter: fast-by-default, feature-gated heavy paths.
- Add smoke tests for new CLI/tooling; prefer synthetic fixtures.

## PR basics
- Include repro/commands for reviewers (fmt/clippy/test).
- Update docs when changing behavior (book/README as appropriate).
- Keep commits scoped; avoid drive-by refactors in the same PR.
