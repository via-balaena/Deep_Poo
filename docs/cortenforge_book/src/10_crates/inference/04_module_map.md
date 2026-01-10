# inference: Module Map
Quick read: What each module owns and why it exists.

- `factory`: InferenceFactory, InferenceThresholds, logic to load checkpoints and build detectors (Burn or heuristic), backend/model selection.
- `plugin`: InferencePlugin and InferenceState for Bevy integration.
- `prelude`: Convenience re-exports.
- `lib.rs`: Re-exports factory/plugin/prelude and backend/model aliases.

Cross-module dependencies:
- factory depends on models and burn.
- plugin integrates with sim_core mode sets.
- consumers (runtime/tools) use factory and/or plugin.

## Links
- Source: `crates/inference/src/lib.rs`
- Module: `crates/inference/src/factory.rs`
- Module: `crates/inference/src/plugin.rs`
