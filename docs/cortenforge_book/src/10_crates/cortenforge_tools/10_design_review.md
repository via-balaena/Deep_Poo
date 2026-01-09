# Design Review (cortenforge-tools)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Thin orchestration/wrapper layer; leverages other crates rather than duplicating logic.
- Helpful service helpers and warehouse command builders reduce boilerplate in bins.
- Feature flags gate heavier bins/features to keep default footprint small.

## Risks / gaps
- Mix of shared and app-specific tooling; unclear boundary increases risk of leaking app concerns into the library repo.
- Error handling is best-effort; some silent skips (e.g., missing manifests) could hide issues.
- No clear publishing story; keeping `publish = false` is intentional but needs ongoing discipline.

## Refactor ideas
- Execute the split/trim plan: move app-specific bins to the app repo, keep only shared helpers here.
- Add clearer logging/metrics for service operations to surface failures.
- Document and enforce ownership boundaries (what stays here vs. moves) to avoid future drift.
