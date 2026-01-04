# Runtime & pipelines

Core building blocks of the substrate: shared crates in motion, end-to-end flow, and where to extend.

- **Runtime loop**: `sim_core` builds the Bevy app with mode sets (Common/SimDatagen/Inference) and installs runtime systems (`SimRuntimePlugin`). Apps add their own plugins for controls/world/autopilot and recorder meta/world state.
- **Capture**: `vision_runtime::CapturePlugin` configures render targets/readback; emits `FrameRecord`s gated by run mode. Defaults write to disk via `capture_utils::JsonRecorder`.
- **Inference**: `vision_runtime::InferencePlugin` schedules a detector (Burn backend or heuristic) on captured frames; updates overlay state.
- **Recorder**: `sim_core` recorder types carry meta + world state; sinks live in `capture_utils`. Apps can swap sinks and supply metadata.
- **ETL/Training**: tools (`colon_sim_tools`) transform captures → warehouse shards; `training` consumes shards to produce checkpoints; `models` define TinyDet/BigDet.
- **Inference (offline/online)**: `inference` factory loads checkpoints and returns a `Detector`; runtime uses it live, tools use it for single-image inference.

Extension points (brief):
- Sim hooks (controls/autopilot), recorder meta/world providers, vision capture/inference hooks, custom sinks, detector factory inputs, and tool CLI additions. See `hooks.md` for details.
