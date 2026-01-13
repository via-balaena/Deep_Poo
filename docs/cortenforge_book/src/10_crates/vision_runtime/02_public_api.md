# vision_runtime: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| PrimaryCameraFrame | struct | Captured frame resource |
| PrimaryCameraState | struct | Tracks primary camera state |
| PrimaryCameraFrameBuffer | struct | Buffer for captured frames |
| ModelLoadState | struct | Tracks whether a model is loaded |
| DetectionOverlayState | struct | Overlay UI state for detections |
| DetectorKind | enum | Detector type (Burn/Heuristic) |
| InferenceThresholdsResource | struct | Bevy wrapper for inference thresholds |
| RuntimeDetectionResult | struct | Runtime detection results |
| AsyncInferenceState | struct | Tracks async inference task state |
| DetectorHandle | struct | Resource holding the active detector |
| CapturePlugin | struct | Bevy plugin to set up capture pipeline |
| InferencePlugin | struct | Bevy plugin to set up inference pipeline |
| setup_front_capture | fn | Configure capture target/readback |
| track_front_camera_state | fn | Track camera state resource |
| capture_front_camera_frame | fn | Capture a frame into buffer |
| on_front_capture_readback | fn | Handle GPU readback for capture |
| schedule_burn_inference | fn | Schedule detector task on frames |
| threshold_hotkeys | fn | Handle hotkeys to adjust thresholds |
| recorder_draw_rect | fn | Draw rect overlay into recorder output |
| poll_inference_task | fn | Poll inference task completion |
| Modules (pub mod) | module | prelude |

## Links
- Source: `crates/vision_runtime/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge-vision-runtime
