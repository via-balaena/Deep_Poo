# vision_runtime: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide Bevy plugins for capture and inference built on `vision_core`, wiring detectors into the runtime loop for live capture/inference scenarios.

## Scope
- Capture pipeline: render target/readback setup, frame capture to `FrameRecord`, mode gating.
- Inference pipeline: detector handle management (Burn or heuristic), scheduling on captured frames, overlay state updates.
- Bevy integration: resources/plugins/systems to plug into sim_core-built apps.

## Non-goals
- No detector implementations (handled by inference/models).
- No recorder sinks (handled by capture_utils/apps), though it emits frame records.
- No app/world/domain systems; apps supply those.

## Who should use it
- App authors wiring capture/inference into Bevy apps built on sim_core.
- Contributors extending capture/inference plugins or overlay state handling.

## Links
- Source: `vision_runtime/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge-vision-runtime
