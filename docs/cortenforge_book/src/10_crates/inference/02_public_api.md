# inference: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| InferenceFactory | struct | Loads checkpoints and builds detectors |
| InferenceThresholds | struct | Thresholds for inference |
| InferencePlugin | struct | Bevy plugin to integrate inference state |
| InferenceState | struct | Tracks inference state (plugin) |
| InferenceBackend | type | Backend alias (NdArray or WGPU) |
| `InferenceModel<B>` | type | Model alias (TinyDet/BigDet) for backend B |
| InferenceModelConfig | type | Model config alias |
| Modules (pub mod) | module | factory, plugin, prelude |
| Re-exports | re-export | InferenceFactory, InferenceThresholds, InferencePlugin |

## Links
- Source: `crates/inference/src/lib.rs`
- Module: `crates/inference/src/factory.rs`
- Module: `crates/inference/src/plugin.rs`
- Docs.rs: https://docs.rs/cortenforge-inference
