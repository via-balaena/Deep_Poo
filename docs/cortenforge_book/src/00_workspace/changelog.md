# Changelog Guide (mdBook)
**Why**: Capture the changes that affect readers and maintainers.
**How it fits**: Update this alongside real code changes.
**Learn more**: See [Maintenance Routine](maintenance_routine.md).

How to track workspace changes and keep the book current.

## What to record
Quick scan of change categories that should land in the changelog.
| Change type | What to capture |
| --- | --- |
| New crates | Name, path, purpose, publish status, feature flags. Add crate chapter and update `SUMMARY.md`. |
| Breaking changes | APIs removed/renamed, schema changes (`data_contracts`), feature flag defaults, backend changes. |
| Refactors | Module moves, file relocations, flow changes (capture, training, inference), dependency bumps that alter behavior. |
| Deletions/migrations | Crates removed or moved to other repos (e.g., `cortenforge-tools` split); note where content moved. |
| Release milestones | Versions published, tags, lockfile updates if they change interfaces. |

## Update process
Steps to keep the book synchronized after adding a changelog entry.
1) Add an entry under the current date/version with a short summary and impacted crates.
2) Update related pages:
   - Crate chapters (overviews, module maps, APIs, lifecycles).
   - `canonical_flows.md` and `integration_contracts.md` if interfaces/flows changed.
   - `SUMMARY.md` when adding/removing chapters.
   - README / RELEASE notes if public-facing changes.
3) Run `mdbook build` to ensure navigation/linking is valid.

## Template entry
Copy/paste scaffold for new release notes.
```text
## YYYY-MM-DD (v{x}.{x}.{x})
- Changed: <describe change and crates affected>
- Impact: <breaking?/compatible?>; <any migration steps>
- Actions taken: <pages updated, diagrams refreshed>
```

### 2026-01-10 (v0.2.1)
- Changed: added `gpu_probe` bin with stable JSON schema and `--format=json` flag.
- Impact: compatible; `gpu_macos_helper` alias removed in v0.3+ (use `gpu_probe`).
- Actions taken: updated tools crate docs to prefer `gpu_probe` and note alias/deprecation.

### 2026-01-10 (v0.3.1)
- Changed: removed `gpu_macos_helper` alias bin; `datagen_scheduler` now calls `gpu_probe` only.
- Impact: compatible for users already on `gpu_probe`; breaking for callers still invoking `gpu_macos_helper`.
- Actions taken: updated tools crate docs and changelog to drop the alias.

### 2026-01-10 (v0.3.0)
- Changed: removed legacy feature aliases (`burn_runtime`, `gpu_nvidia`) and standardized on `burn-runtime`/`gpu-nvidia`.
- Impact: breaking; update feature flags in downstream crates and docs to match new names.
- Actions taken: updated manifests, docs, and release notes to reference normalized flags only.

### 2026-01-10 (v0.2.0)
- Changed: bumped workspace dependencies to latest stable (Burn 0.19.1, bincode 2.0.1, Arrow/Parquet 57.1.0, sysinfo 0.37.2, ratatui 0.30.0).
- Impact: potentially breaking (Burn API changes); bincode 3.x deferred because crates.io 3.0.0 is a stub.
- Actions taken: updated docs to reflect new dependency versions and compatibility notes.

### 2026-01-09 (v0.1.5)
- Changed: moved the `cortenforge` umbrella crate to the repo root (`Cargo.toml` + `src/`).
- Changed: updated workspace members and source-link paths to point at root `src/lib.rs`.
- Impact: compatible; no API surface changes expected beyond layout/navigation.
- Actions taken: updated book paths and README layout notes.

### 2026-01-09 (v0.1.4)
- Changed: moved core crates under `crates/` and updated workspace members/patches to match.
- Changed: unified Bevy version usage and refreshed source-link paths in the book.
- Changed: pinned CI toolchain, split heavy tests to scheduled runs, and refreshed cargo-deny config.
- Impact: compatible; no API surface changes expected beyond the path/layout refactor.
- Actions taken: updated crate layout docs and ignored generated mdBook output.

### 2026-01-09 (v0.1.3)
- Changed: publish `cortenforge-tools` and add it as an optional feature in the `cortenforge` umbrella.

### 2026-01-09 (v0.1.2)
- Changed: consolidated docs into `cortenforge_book` and removed legacy books.
- Changed: added a calm “Building Apps” path and simplified the crate overview/deep‑dive split.
- Changed: renamed and refactored `cortenforge-tools` with config support and clearer defaults.
- Changed: bumped workspace crate versions to `0.1.2`.

### 2026-01-07 (v0.1.1)
Example entry showing the expected level of detail.
- Changed: aligned workspace crates to `0.1.1`; refreshed docs/version references; removed burn-core vendor patch and updated to 0.14.1.
- Docs: completed quality-gates sweep across crate chapters; added source/docs.rs links and aligned examples with current APIs.
- Impact: compatible; no API breaks expected beyond version alignment.
- Actions taken: updated crate metadata/release checklist; staged publish order and notes.

## Tips
Short guidance for keeping entries useful and readable over time.
- Keep entries concise; link to PRs or commits for detail.
- Prefer grouping related changes per release rather than per-commit noise.
- When unsure, err on the side of documenting breaking behavior and feature flag changes.
