# capture_utils: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
- Use default recorder sink:
  ```rust,ignore
  use vision_core::prelude::{Frame, FrameRecord};
  let mut recorder = JsonRecorder::new(run_dir);
  let frame = Frame {
      id: 1,
      timestamp: 0.0,
      rgba: None,
      size: (640, 480),
      path: Some("images/frame_00001.png".into()),
  };
  let record = FrameRecord {
      frame,
      labels: &[],
      camera_active: true,
      polyp_seed: 1,
  };
  recorder.record(&record)?;
  ```
- Overlay/prune helpers in tools:
  ```rust,ignore
  generate_overlays(run_dir)?;
  let (_kept, _dropped) = prune_run(input_run, output_root)?;
  ```

## Execution flow
- Recorder sinks consume labels/frames produced by runtime/tools and write JSON manifests.
- Overlay/prune functions operate on capture directories (frames/labels) to produce overlays or filtered runs.

## Notes
- Stateless helpers; lifecycle driven by callers (runtime/tools).

## Links
- Source: `capture_utils/src/lib.rs`
