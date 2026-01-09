# plugin (inference)

## Responsibility
- Wire inference state into Bevy as a plugin hook point.
- Provide a stub system (`inference_stub`) that seeds `InferenceState` with a default `DetectionResult` while real scheduling/polling is TODO.

## Key items
- `InferenceState` (Resource): holds the last `DetectionResult` (optional).
- `inference_stub`: populates `InferenceState` with a dummy negative detection if empty.
- `InferencePlugin`: registers state, configures `ModeSet::Inference`, and runs the stub system in the `Update` schedule.

## Invariants / Gotchas
- No real inference is performed yet; this is a placeholder to keep the pipeline wired.
- Systems are gated under `ModeSet::Inference`; ensure sim_core adds that mode to the schedule.
- Downstream users should replace the stub with actual detector invocation once Burn integration is finalized.

## Cross-module deps
- Uses `sim_core::ModeSet` to slot into the shared schedule.
- Shares `vision_core::interfaces::DetectionResult` so the state matches the detector output type.
- Intended to pair with `InferenceFactory` once model-backed detection is ready.
