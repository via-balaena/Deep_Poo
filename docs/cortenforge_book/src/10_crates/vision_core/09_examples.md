# Examples (vision_core)
Quick read: Minimal examples you can adapt safely.

## 1) Implement a simple detector
```rust,ignore
use vision_core::interfaces::{Detector, Frame, DetectionResult};

struct ThresholdDetector {
    threshold: f32,
}

impl Detector for ThresholdDetector {
    fn detect(&mut self, frame: &Frame) -> DetectionResult {
        // Toy heuristic: positive if frame id is even
        let positive = frame.id % 2 == 0;
        DetectionResult {
            frame_id: frame.id,
            positive,
            confidence: if positive { self.threshold } else { 1.0 - self.threshold },
            boxes: Vec::new(),
            scores: Vec::new(),
        }
    }
}
```

## 2) Implement a recorder
```rust,ignore
use std::fs::File;
use std::io::Write;
use vision_core::interfaces::{FrameRecord, Recorder};

struct FileRecorder {
    out: File,
}

impl Recorder for FileRecorder {
    fn record(&mut self, record: &FrameRecord) -> std::io::Result<()> {
        writeln!(
            self.out,
            "frame={} labels={} active={}",
            record.frame.id,
            record.labels.len(),
            record.camera_active
        )
    }
}
```

## 3) Implement a frame source
```rust,ignore
use vision_core::interfaces::{Frame, FrameSource};

struct StaticFrameSource {
    frames: Vec<Frame>,
    idx: usize,
}

impl FrameSource for StaticFrameSource {
    fn next_frame(&mut self) -> Option<Frame> {
        if self.idx >= self.frames.len() {
            return None;
        }
        let f = self.frames[self.idx].clone();
        self.idx += 1;
        Some(f)
    }
}
```

## Links
- Source: `vision_core/src/interfaces.rs`
