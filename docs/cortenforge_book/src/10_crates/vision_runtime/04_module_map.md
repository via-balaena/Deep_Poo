# vision_runtime: Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Capture and inference pipeline types/functions and Bevy plugins.
  - Plugins: `CapturePlugin`, `InferencePlugin`.
  - Resources: FrontCamera*, DetectorHandle, thresholds.
  - Systems: capture/readback/inference scheduling, overlay state, hotkeys, polling.
- `prelude`: Re-export of commonly used types from lib.rs.

Cross-module dependencies:
- single-module crate.
- relies on vision_core types, inference detectors, and sim_core/Bevy app context.

## Links
- Source: `vision_runtime/src/lib.rs`
