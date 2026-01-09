# Dev Loop
**Why**: Keep feedback fast while you iterate.
**How it fits**: Pair these watchers with the main build commands.
**Learn more**: See [Build & Run](build_and_run.md).

## Watchers
Quick commands to keep code and docs continuously rebuilt.

| Target | Command |
| --- | --- |
| Code | `cargo watch -x fmt -x 'clippy --workspace --all-targets -D warnings' -x 'test --workspace'` |
| Docs | `cargo watch -w docs/cortenforge_book -s 'mdbook build docs/cortenforge_book'` |

## Doc rebuild strategy
How to keep docs builds consistent and fast while iterating.

| Topic | Guidance |
| --- | --- |
| Makefile targets | `make docs` (build), `make docs-watch` (serve/rebuild), `make docs-test` (doctests). |
| Snippet hygiene | Mark non-runnable snippets with `ignore`/`no_run` to keep `mdbook test` green. |

## Keeping mdBook in sync
Key checkpoints to keep documentation aligned with code changes.

| Trigger | Action |
| --- | --- |
| APIs change | Update crate pages (overview/API/examples) and dependency graph if edges shift. |
| Features change | Update `feature_flags.md` and per-crate examples/snippets. |
| Releases | Refresh version strings, confirm burn-core note (0.14.1, no patch), and changelog. |
| Before merge | Run `mdbook test` and `mdbook build`. |
