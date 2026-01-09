# Design Review (cortenforge)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Minimal facade; reduces dependency boilerplate for consumers who want a single entry point.
- Feature-gated re-exports prevent pulling in unneeded crates.

## Risks / gaps
- Facade can hide fine-grained versioning; consumers may prefer direct deps for clarity.
- Needs maintenance when member crates change; easy to drift if not kept in sync.

## Refactor ideas
- Keep facade optional in docs; encourage direct crate deps for advanced users.
- Add a CI check to ensure feature list matches workspace members.

## Links
- Source: `crates/cortenforge/src/lib.rs`
