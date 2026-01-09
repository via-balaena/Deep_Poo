# vision_core: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide a detector/capture data model and overlay math that are engine-agnostic, forming the core vision interfaces for the stack.

## Scope
- Data types: frames, frame records, labels, detection results.
- Traits/interfaces: Detector/Recorder abstractions.
- Overlay utilities (e.g., draw_rect) and capture limits.
- Interfaces/overlay are runtime-agnostic; capture resources use Bevy types for integration.

## Non-goals
- No Bevy plugins or runtime scheduling (handled by vision_runtime).
- No detector implementations or model loading (handled by inference/models).
- No recorder sinks (handled by capture_utils/apps).

## Who should use it
- Runtime/plugins (vision_runtime) needing the shared vision interfaces and overlay math.
- Tools and sinks that operate on frame/label data structures.
- Contributors defining detectors/recorders or working on overlay logic.

## Links
- Source: `vision_core/src/lib.rs`
- Module: `vision_core/src/interfaces.rs`
- Module: `vision_core/src/overlay.rs`
- Module: `vision_core/src/capture.rs`
- Docs.rs: https://docs.rs/cortenforge-vision-core
