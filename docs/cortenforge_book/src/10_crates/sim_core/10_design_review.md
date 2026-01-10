# Design Review (sim_core)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Very small surface: Bevy scaffolding + hook traits make it easy for an app to bolt on controls/autopilot without touching core.
- Recorder metadata/sink abstractions are simple trait objects with clear responsibilities.
- No hidden side effects; resources are explicit.

## Risks / gaps
- Recorder metadata is fixed to a single seed field; if more metadata is needed, the trait will need to evolve (breaking change).
- Hook registration relies on caller discipline (no duplicate registration, ordering); no guards or idempotence.
- Recorder sink is a single boxed writer; no built-in fan-out or composition.

## Refactor ideas
- Make `RecorderMetadataProvider` extensible (e.g., key/value map or versioned struct) to avoid frequent breaking changes.
- Add optional idempotent registration helpers (e.g., sets to avoid double-adding systems).
- Provide a small fan-out recorder combinator (e.g., `Vec<Box<Recorder>>`) for multi-sink use cases.

## Links
- Source: `crates/sim_core/src/recorder_meta.rs`
