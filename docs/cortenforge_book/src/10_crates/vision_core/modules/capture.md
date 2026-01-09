# capture (vision_core)

## Responsibility
Defines capture-related resources/components for front camera handling and GPU readback.

## Key items
- `CaptureLimit` (Resource): Optional max_frames limit.
- `FrontCamera` (Component): Marker for front camera.
- `FrontCaptureCamera` (Component): Marker for capture camera.
- `FrontCaptureTarget` (Resource): Render target handle, size, entity for capture.
- `FrontCaptureReadback` (Resource): Latest readback bytes.

## Invariants / Gotchas
- Resources/components are markers/state; actual capture logic lives in vision_runtime.
- Ensure `FrontCaptureTarget` is populated before use; readback may be None until a frame is captured.

## Cross-module deps
- Used by vision_runtime capture systems; shared data model comes from vision_core interfaces.
