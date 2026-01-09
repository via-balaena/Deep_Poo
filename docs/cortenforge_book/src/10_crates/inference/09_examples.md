# Examples (inference)
Quick read: Minimal examples you can adapt safely.

## 1) Heuristic detector fallback
```rust,ignore
use inference::{InferenceFactory, InferenceThresholds};

fn main() {
    let factory = InferenceFactory;
    let mut detector = factory.build(InferenceThresholds::default(), None);
    let frame = vision_core::interfaces::Frame {
        id: 1,
        timestamp: 0.0,
        rgba: None,
        size: (640, 480),
        path: None,
    };
    let result = detector.detect(&frame);
    println!("conf={:.2} positive={}", result.confidence, result.positive);
}
```

## 2) Load Burn checkpoint (if available)
```rust,ignore
use std::path::Path;
use inference::{InferenceFactory, InferenceThresholds};

fn main() {
    let factory = InferenceFactory;
    let weights = Path::new("artifacts/checkpoints/tinydet.bin");
    let mut detector = factory.build(InferenceThresholds { obj_thresh: 0.5, iou_thresh: 0.5 }, Some(weights));
    let frame = vision_core::interfaces::Frame {
        id: 2,
        timestamp: 0.0,
        rgba: None,
        size: (640, 480),
        path: None,
    };
    let result = detector.detect(&frame);
    println!("boxes={}, scores={}", result.boxes.len(), result.scores.len());
}
```

## 3) Adjust thresholds at construction
```rust,ignore
use inference::{InferenceFactory, InferenceThresholds};

fn main() {
    let factory = InferenceFactory;
    let mut detector = factory.build(
        InferenceThresholds { obj_thresh: 0.8, iou_thresh: 0.6 },
        None,
    );
    // detector uses obj_thresh internally for confidence check
    let _ = detector;
}
```

## Links
- Source: `inference/src/factory.rs`
