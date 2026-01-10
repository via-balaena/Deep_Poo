# capture_utils: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide recorder sinks and capture helpers (overlay/prune) compatible with CortenForge schemas for use in runtime and tooling.

## Scope
- Default JSON recorder sink for frame labels.
- Overlay/prune helpers used by tools/tests.
- Helpers aligned with `data_contracts` schemas.

## Non-goals
- No recorder meta/world state definitions (from sim_core/app).
- No runtime plugins (vision_runtime) or detector logic.
- No app-specific sinks; keep schema-compatible for ETL/training.

## Who should use it
- Runtime recorder pipelines (sim_core/apps) needing a default sink.
- Tools performing overlay/prune on captures.
- Contributors adding sinks while preserving schema compatibility.

## Links
- Source: `crates/capture_utils/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge-capture-utils
