# Examples (vision_runtime)
Quick read: Minimal examples you can adapt safely.

## 1) Minimal capture plugin usage
```rust,ignore
use bevy::prelude::*;
use sim_core::ModeSet;
use vision_runtime::CapturePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CapturePlugin)
        .add_systems(Update, |state: Res<vision_runtime::FrontCameraState>| {
            info!("Front camera active: {}, frames: {}", state.active, state.frame_counter);
        })
        .run();
}
```

## 2) Hook up a heuristic detector and run inference
```rust,ignore
use bevy::prelude::*;
use sim_core::ModeSet;
use vision_runtime::{CapturePlugin, DetectorHandle, DetectorKind, InferencePlugin, InferenceThresholds};
use vision_core::interfaces::{DetectionResult, Detector, Frame};

// Simple detector for demo purposes
struct Heuristic;
impl Detector for Heuristic {
    fn detect(&mut self, frame: &Frame) -> DetectionResult {
        DetectionResult { frame_id: frame.id, positive: true, confidence: 0.9, boxes: vec![], scores: vec![] }
    }
}

fn main() {
    App::new()
        .insert_resource(sim_core::SimRunMode::Inference)
        .insert_resource(DetectorHandle { detector: Box::new(Heuristic), kind: DetectorKind::Heuristic })
        .insert_resource(InferenceThresholds { obj_thresh: 0.5, iou_thresh: 0.5 })
        .add_plugins(DefaultPlugins)
        .add_plugins((CapturePlugin, InferencePlugin))
        .run();
}
```

## 3) Poll inference results (overlay state)
```rust,ignore
use bevy::prelude::*;
use sim_core::ModeSet;
use vision_runtime::{CapturePlugin, InferencePlugin, DetectionOverlayState};

fn log_overlay(overlay: Res<DetectionOverlayState>) {
    if let Some(ms) = overlay.inference_ms {
        info!("Last inference took {:.2} ms, boxes={}", ms, overlay.boxes.len());
    }
}

fn main() {
    App::new()
        .insert_resource(sim_core::SimRunMode::Inference)
        .add_plugins(DefaultPlugins)
        .add_plugins((CapturePlugin, InferencePlugin))
        .add_systems(Update, log_overlay.in_set(ModeSet::Inference))
        .run();
}
```

## Links
- Source: `crates/vision_runtime/src/lib.rs`
