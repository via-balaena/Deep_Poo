# recorder_meta (sim_core)

## Responsibility
Recorder metadata provider interfaces and resources for world state and sinks.

## Key types
- `RecorderMetadataProvider` (trait): provides metadata (polyp_seed).
- `RecorderMetaProvider` (Resource): boxed provider implementing the trait.
- `BasicRecorderMeta` (struct): simple provider with a seed field.
- `RecorderSink` (Resource): holds optional writer implementing `vision_core::Recorder`.
- `RecorderWorldState` (Resource): app-provided world state (head_z, stop_flag).

## Invariants / Gotchas
- Provider/sink are optional; app should insert concrete implementations.
- Validation is deferred to sink implementations; ensure seeds/meta align with schemas.

## Cross-module deps
- Used by recorder_types/runtime to drive recorder behavior; consumed by runtime/tools sinks.
