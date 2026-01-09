# Maintenance Routine (Weekly)
**Why**: A short rhythm keeps the book aligned with the codebase.
**How it fits**: Do this after refactors or before a release.
**Learn more**: See [Quality Gates](quality_gates.md).

Keep the CortenForge book aligned with the codebase and releases.

## Weekly checklist
Routine checks to keep docs aligned week to week.
| Area | Action |
| --- | --- |
| Build status | Run `mdbook build docs/cortenforge_book` to ensure no warnings/errors. |
| Feature flags | Review `feature_flags.md` for new/removed flags or defaults. |
| Flows & contracts | Skim `canonical_flows.md` and `integration_contracts.md` after major refactors (capture/train/inference). |
| Crate chapters | Spot-check crates that changed; ensure public API/traits/error/perf/ownership pages still match code. |
| Open questions | Update statuses, add new unknowns as they appear. |
| Changelog | Add entries for notable changes (new crates, breaking changes, refactors). |
| Links | Verify `linking_style` usage and docs.rs links if versions bumped. |

## Release prep (when tagging/publishing)
Checklist to run only when cutting a release.
1) Confirm `[patch]`/vendor notes are current; update `overview.md` and crate pages.
2) Update versions/links in `docsrs_alignment.md` if docs.rs URLs change.
3) Run `mdbook build` and fix warnings before publish.
4) Deploy: ensure the GitHub Actions `mdbook` workflow runs clean for the CortenForge book.

## Quick commands
Common shortcuts for recurring maintenance tasks.
| Purpose | Command |
| --- | --- |
| Crate list, module tree, pub items. | `tools/extract_for_book.sh` |
| Build check. | `mdbook build docs/cortenforge_book` |
| Shortcut to build. | `tools/book_tasks.sh build` |
