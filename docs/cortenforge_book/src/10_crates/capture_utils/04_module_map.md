# capture_utils: Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Single-module crate containing:
  - `JsonRecorder`: Default JSON recorder sink implementing `vision_core::Recorder` for labels.
  - `generate_overlays`: Helper to render overlays from label JSONs in a run dir.
  - `prune_run`: Helper to copy a run into a filtered root and report kept/skipped counts.
  - Local helper `draw_rect` for overlay rendering.

Cross-module dependencies:
- Uses `data_contracts` for schemas and `vision_core` interfaces for recorder integration.
- Uses standard image/fs utilities.
- Consumed by runtime/apps/tools.

## Links
- Source: `capture_utils/src/lib.rs`
