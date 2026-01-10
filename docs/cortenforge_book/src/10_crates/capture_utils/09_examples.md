# Examples (capture_utils)
Quick read: Minimal examples you can adapt safely.

## 1) Write labels with JsonRecorder
```rust,ignore
use capture_utils::JsonRecorder;
use vision_core::prelude::{Frame, FrameRecord};

fn main() -> std::io::Result<()> {
    let mut recorder = JsonRecorder::new("assets/datasets/captures/run_00001");
    let frame = Frame {
        id: 1,
        timestamp: 0.1,
        rgba: None,
        size: (640, 480),
        path: Some("images/frame_00001.png".into()),
    };
    let record = FrameRecord {
        frame,
        labels: &[],
        camera_active: true,
        polyp_seed: 42,
    };
    recorder.record(&record)
}
```

## 2) Generate overlays for a run
```rust,ignore
use std::path::Path;

fn main() -> anyhow::Result<()> {
    capture_utils::generate_overlays(Path::new("assets/datasets/captures/run_00001"))
}
```

## 3) Copy a run into a filtered root and report counts
```rust,ignore
use std::path::Path;

fn main() -> std::io::Result<()> {
    let (kept, skipped) = capture_utils::prune_run(
        Path::new("assets/datasets/captures/run_00001"),
        Path::new("assets/datasets/captures_filtered"),
    )?;
    println!("kept {kept} labels, skipped {skipped}");
    Ok(())
}
```

## Links
- Source: `crates/capture_utils/src/lib.rs`
