# recorder_types (sim_core)

## Responsibility
Configuration and state types for the recorder pipeline.

## Key types
- `RecorderConfig` (Resource): output_root, capture_interval (Timer), resolution, prune settings.
- `RecorderState` (Resource): runtime state (enabled, session_dir, frame_idx, toggles, paused, overlays/prune flags, init/manifest state).
- `AutoRecordTimer` (Resource): timer for auto-recording.
- `RecorderMotion` (Resource): last head_z, cumulative_forward, started flag.

## Invariants / Gotchas
- Defaults point to `assets/datasets/captures`; adjust for your app environment.
- `RecorderState` has many flags; ensure they are updated consistently in recorder systems.
- `capture_interval` default 0.33s; adjust for desired frame rate.

## Cross-module deps
- Inserted by `SimRuntimePlugin`; consumed by recorder systems (app/runtime) and sinks.
