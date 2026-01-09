# Book Redundancy Audit

Goal: zero redundancy across the dissection book (workspace + crates), the flows chapter, and the contributor book while preserving depth via links (docs.rs, source).

## Guiding principles
- One primary home per topic; all other mentions become short links.
- Dissection book explains architecture and crate internals; contributor book explains how to build, test, release, and operate.
- Deep API reference lives in docs.rs and source links, not in book prose.
- Prefer a thin index over repeated explanations; reduce cognitive load by routing readers quickly.

## Legacy context (from the old two-book setup)
The dissection book held architecture and crate internals; the contributor book held build/test/release guidance.
The new plan collapses both into one calm book, with docs.rs and source links used for depth instead of duplicated prose.

## Consolidated book plan (current target)
Single book: `docs/cortenforge_book` with a calm, low‑load progression.

### Decisions (locked)
- One book only (successor to dissection + contributor); other books are deleted after this one is complete.
- Building Apps is the primary onboarding path; crates link to docs.rs for deep API detail.
- Workspace chapter stays as the “big picture” map (short, link‑heavy, low cognitive load).
- Glossary lives in the CortenForge book only (no duplicate elsewhere).

### Final structure (current)
| Chapter | Purpose | Notes |
| --- | --- | --- |
| 0) Workspace | Big picture + contracts + flows; link out for depth. | Keep every page short; use “Why / How it fits / Learn more.” |
| 1) Building Apps | Story‑based, step‑by‑step onboarding. | AstroForge Surveyor, 9 steps + epilogue. |
| 2) Crate Deep Dives | Source of truth per crate. | Each page starts with “Quick read”; link docs.rs in README. |
| 3) Reference (short) | Links, contracts, changelog, open questions. | Avoid long checklists; link to ops docs if needed. |

### Migration checklist
1) Finish cortenforge_book content and tone pass (complete).
2) Point all internal links to the new book (SUMMARY + cross‑links).
3) Delete or archive dissection + contributor books once the new one is complete.
4) Update any README/entrypoints to link to the new book only.

### Gaps to fill (if any)
- Confirm which “Reference” pages are still needed (linking_style, docsrs_alignment, integration_contracts, changelog, open_questions).
- Decide if any ops/build content should live outside the book (repo docs vs book).

### Status
- Done: Building Apps chapter (9 steps + epilogue) and calm framing across Workspace + Crate intros.
- Done: Crate pages now include quick‑read lines and docs.rs links.
- Done: Link migration (all entrypoints point at `cortenforge_book`).
- Done: Legacy contributor/dissection books removed from the repo.

## Cortenforge book outline (removed)
Superseded by “Final structure (current)” above.

## Guided project (proposed)
Name: AstroForge Surveyor.
Premise: A scout ship surveys asteroid fields, tags promising rocks, then schedules mining runs.

Progression (maps to crates):
1) Sim + capture: `sim_core` + `vision_runtime` + `capture_utils` to generate and record runs.
2) Schema + dataset: `data_contracts` + `burn_dataset` to validate and load captures.
3) Train + model: `models` + `training` to produce checkpoints.
4) Inference loop: `inference` + `vision_core` in a runtime pipeline.
5) Tooling: `cortenforge-tools` + `cli_support` for orchestration and TUI/scheduler options.
6) Umbrella: `cortenforge` as the "bundle" a real app depends on.

Notes:
- Each step should link directly to the relevant crate chapter and docs.rs for API detail.
- Keep each page to a single flow with a minimal diagram and a short "try this" snippet.
