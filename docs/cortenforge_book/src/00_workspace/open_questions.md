# Open Questions
**Why**: Keep uncertainty visible so it gets resolved.
**How it fits**: Use this after deep dives or during refactors.
**Learn more**: See [Maintenance Routine](maintenance_routine.md).

Use this page to track unknowns and where to investigate. Add/remove entries as they’re answered.

## Current questions
1) (none) — add entries below as needed.

## Template
1) **Question**: <what do we need to know?>
2) **Where to look**: <files/modules/tests to inspect>
3) **Status**: Unanswered / In progress / Answered
4) **Notes**: <findings, hypotheses, links>

### Example:
**Question**: Should `capture_utils::generate_overlays` log skipped files?  
**Where to look**: `crates/capture_utils/src/lib.rs`, overlay generation loop.  
**Status**: Unanswered.  
**Notes**: Currently silent; consider adding logging for missing images.
