# Changelog Guide (mdBook)

How to track workspace changes and keep the book current.

## What to record
- **New crates**: name, path, purpose, publish status, feature flags. Add crate chapter and update SUMMARY.
- **Breaking changes**: APIs removed/renamed, schema changes (`data_contracts`), feature flag defaults, backend changes.
- **Refactors**: module moves, file relocations, flow changes (capture, training, inference), dependency bumps that alter behavior.
- **Deletions/migrations**: crates removed or moved to other repos (e.g., `colon_sim_tools` split); note where content moved.
- **Release milestones**: versions published, tags, lockfile updates if they change interfaces.

## Update process
1) Add an entry under the current date/version with a short summary and impacted crates.
2) Update related pages:
   - Crate chapters (overviews, module maps, APIs, lifecycles).
   - `canonical_flows.md` and `integration_contracts.md` if interfaces/flows changed.
   - `SUMMARY.md` when adding/removing chapters.
   - README / RELEASE notes if public-facing changes.
3) Run `mdbook build` to ensure navigation/linking is valid.

## Template entry
```text
## YYYY-MM-DD (v{x}.{x}.{x})
- Changed: <describe change and crates affected>
- Impact: <breaking?/compatible?>; <any migration steps>
- Actions taken: <pages updated, diagrams refreshed>
```

## 2026-01-07 (v0.1.1)
- Changed: aligned workspace crates to `0.1.1`; refreshed docs/version references; removed burn-core vendor patch and updated to 0.14.1.
- Docs: completed quality-gates sweep across crate chapters; added source/docs.rs links and aligned examples with current APIs.
- Impact: compatible; no API breaks expected beyond version alignment.
- Actions taken: updated crate metadata/release checklist; staged publish order and notes.

## Tips
- Keep entries concise; link to PRs or commits for detail.
- Prefer grouping related changes per release rather than per-commit noise.
- When unsure, err on the side of documenting breaking behavior and feature flag changes.
