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

### 2026-01-09 (v0.1.4)
- Changed: moved core crates under `crates/` and updated workspace members/patches to match.
- Changed: unified Bevy version usage and refreshed source-link paths in the book.
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
