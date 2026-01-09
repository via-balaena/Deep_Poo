# camera (sim_core)

## Responsibility
Defines camera components/resources and systems for fly camera control and POV toggling; sets up 3D + UI cameras.

## Key types
- `Flycam` (Component): yaw/pitch/speed/mouse_sensitivity for free-fly camera.
- `ProbePovCamera` (Component): marker for probe POV camera.
- `PovState` (Resource): toggles probe vs free camera.
- `UiOverlayCamera` (Component): marker for UI overlay camera.

## Important functions/systems
- `setup_camera`: spawns a 3D camera with Flycam and a UI overlay camera.
- `camera_controller`: handles mouse/keyboard to move/rotate Flycam; right-mouse to capture motion; supports WASD/arrow + space/shift.
- `pov_toggle_system`: toggles active camera between ProbePovCamera and Flycam via `KeyC`.

## Invariants / Gotchas
- `camera_controller` clears mouse motion when RMB not held; ensures pitch clamped to avoid flipping.
- `pov_toggle_system` assumes exactly one of each camera type; ensure queries are populated.

## Cross-module deps
- Uses ModeSet indirectly via app systems; otherwise self-contained in sim_core; consumed by app runtime wiring.
