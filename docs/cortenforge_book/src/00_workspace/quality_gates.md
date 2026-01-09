# Quality Gates
**Why**: Keep every page clear, short, and useful.
**How it fits**: Use this as a checklist when adding or updating pages.
**Learn more**: See [Book Style Guide](style_guide.md).

Rules to keep the book concise, useful, and consistent.

## Per-page rules
| Rule | Description |
| --- | --- |
| State the problem/purpose | Explain why the page exists up front. |
| Key types/actors | Name the important structs/traits/resources involved. |
| Invariants | List constraints/assumptions; note feature gates if relevant. |
| Example | Include at least one runnable/minimal snippet or command. |
| Gotchas | Capture pitfalls, edge cases, feature flags, and perf risks. |
| Links | Add source links (`crate/path.rs:L123`) and/or docs.rs links for referenced items. |
| Scope | Avoid duplicating exhaustive API docs; defer to docs.rs for signatures. |

## Crate chapter checklist
For each crate (`sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`, `models`, `training`, `inference`, `cli_support`, `burn_dataset`, `cortenforge-tools`, `cortenforge`):

**Use this checklist after significant crate changes (API, feature flags, or module layout) and again before tagging a release.**

- [ ] `01_overview.md`: Purpose/scope/non-goals clear; links to docs.rs/source.
- [ ] `02_public_api.md`: Table of pub items present; aligns with current code.
- [ ] `03_lifecycle.md`: Construction/usage narrative accurate.
- [ ] `04_module_map.md`: Modules listed with responsibilities.
- [ ] `05_traits_and_generics.md`: Extensibility/constraints captured.
- [ ] `06_error_model.md`: Error surfaces and handling noted.
- [ ] `07_ownership_and_concurrency.md`: Send/Sync/async/borrowing captured.
- [ ] `08_performance_notes.md`: Hot paths/alloc/cloning highlighted.
- [ ] `09_examples.md`: 2–3 minimal examples compile in principle.
- [ ] `10_design_review.md`: Strengths/risks/refactors noted.

## Progress (publish order)
Track which crate pages were verified for the latest release version.
| Crate | Status | Version | Notes |
| --- | --- | --- | --- |
| data_contracts | complete | 0.1.1 | 01–10 verified |
| models | complete | 0.1.1 | 01–10 verified |
| burn_dataset | complete | 0.1.1 | 01–10 verified |
| cli_support | complete | 0.1.1 | 01–10 verified |
| vision_core | complete | 0.1.1 | 01–10 verified |
| sim_core | complete | 0.1.1 | 01–10 verified |
| capture_utils | complete | 0.1.1 | 01–10 verified |
| inference | complete | 0.1.1 | 01–10 verified |
| vision_runtime | complete | 0.1.1 | 01–10 verified |
| training | complete | 0.1.1 | 01–10 verified |
| cortenforge | complete | 0.1.1 | 01–10 verified |

## Cross-workspace pages
Keep shared “glue” docs in sync so cross-cutting flows and conventions stay accurate.
| Page | Maintenance focus |
| --- | --- |
| `canonical_flows.md` | Flows stay current with crate changes; diagrams updated. |
| `integration_contracts.md` | Assumptions updated when interfaces/schemas change. |
| `docsrs_alignment.md` & `linking_style.md` | Reflect current linking conventions. |

## Update cadence
When to re-run checks so the book stays accurate between releases.
1) On each release/tag: run through the crate checklist and flows; update line links if code moved.
2) When adding a crate: scaffold full set of pages + add to `SUMMARY.md` and checklists.
3) When removing/migrating a crate (e.g., `cortenforge-tools` split): update flows, integration contracts, and remove chapters as appropriate.
