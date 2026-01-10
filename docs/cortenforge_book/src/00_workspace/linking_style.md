# Linking Style
**Why**: Stable links make the book trustworthy over time.
**How it fits**: Use these rules whenever you reference code or docs.
**Learn more**: See [docs.rs Alignment](docsrs_alignment.md).

Standardize source links to keep references consistent and stable.

## Format
How to format links so they stay stable and readable.
- Use repo-relative paths with line anchors: ``crate/path/to/file.rs:L123``.
- Examples:
  - `crates/sim_core/src/hooks.rs:L10`
  - `crates/vision_runtime/src/lib.rs:L75`
  - `docs/cortenforge_book/src/00_workspace/overview.md:L1` (for book refs)
- Avoid range anchors; link to the first relevant line.

## When to link
Where links add clarity without duplicating docs.
1) Point to source when explaining specific functions/types or design decisions.
2) Prefer linking to code over docs.rs when tying to this repo’s version.
3) For public API reference, optionally include a docs.rs link alongside the source link.

## Maintenance
How to keep links accurate as code evolves.
1) When code moves, update links in the same PR as the code change.
2) Keep links in crate pages and flow docs current after refactors.
3) If line numbers are likely to drift, consider linking to the symbol in docs.rs as a secondary reference.
