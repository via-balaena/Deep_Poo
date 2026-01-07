# toy_vision_runtime: Layout & Stubs

File tree:
```text
toy_vision_runtime/
├─ Cargo.toml
└─ src/
   ├─ lib.rs
   ├─ capture.rs
   ├─ inference.rs
   └─ overlay.rs
```

## Cargo.toml
```toml
[package]
name = "toy_vision_runtime"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.13", default-features = false, features = ["bevy_winit", "multi-threaded"] }
futures-lite = "1"
```

## src/lib.rs
```rust,ignore
pub mod capture;
pub mod inference;
pub mod overlay;

pub use capture::{CaptureState, Frame, spawn_capture};
pub use inference::{
    Detector, DetectorHandle, DetectorKind, InferencePlugin, InferenceState, Thresholds,
};
pub use overlay::OverlayState;

pub struct ToyVisionRuntimePlugin;

impl bevy::prelude::Plugin for ToyVisionRuntimePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CaptureState>()
            .init_resource::<InferenceState>()
            .init_resource::<OverlayState>()
            .init_resource::<DetectorHandle>()
            .add_systems(bevy::prelude::Update, (spawn_capture, inference::schedule, inference::poll));
    }
}
```

## src/capture.rs
```rust,ignore
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Frame {
    pub id: u64,
    pub timestamp: f64,
}

#[derive(Resource, Default)]
pub struct CaptureState {
    pub latest: Option<Frame>,
    pub counter: u64,
}

pub fn spawn_capture(mut state: ResMut<CaptureState>, time: Res<Time>) {
    state.counter = state.counter.wrapping_add(1);
    state.latest = Some(Frame {
        id: state.counter,
        timestamp: time.elapsed_seconds_f64(),
    });
}
```

## src/inference.rs
```rust,ignore
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future::poll_once;
use std::time::Instant;

use crate::capture::Frame;
use crate::overlay::OverlayState;

pub trait Detector: Send + Sync {
    fn detect(&mut self, frame: &Frame) -> Detection;
}

#[derive(Clone, Debug)]
pub struct Detection {
    pub frame_id: u64,
    pub confidence: f32,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct Thresholds {
    pub obj: f32,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self { obj: 0.5 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetectorKind {
    Heuristic,
    MockBurn,
}

#[derive(Resource, Default)]
pub struct DetectorHandle {
    pub detector: Box<dyn Detector>,
    pub kind: DetectorKind,
}

#[derive(Resource)]
pub struct InferenceState {
    pub pending: Option<Task<(Box<dyn Detector>, DetectorKind, Detection, f32)>>, // detector, kind, result, ms
}

impl Default for InferenceState {
    fn default() -> Self {
        Self { pending: None }
    }
}

struct HeuristicDetector;
impl Detector for HeuristicDetector {
    fn detect(&mut self, frame: &Frame) -> Detection {
        Detection {
            frame_id: frame.id,
            confidence: 0.8,
        }
    }
}

pub fn schedule(
    mut state: ResMut<InferenceState>,
    mut handle: ResMut<DetectorHandle>,
    capture: Res<crate::capture::CaptureState>,
) {
    if state.pending.is_some() {
        return;
    }
    let Some(frame) = capture.latest.clone() else { return; };
    let mut detector = std::mem::replace(&mut handle.detector, Box::new(HeuristicDetector));
    let kind = handle.kind;
    let task = AsyncComputeTaskPool::get().spawn(async move {
        let t0 = Instant::now();
        let result = detector.detect(&frame);
        let ms = t0.elapsed().as_secs_f32() * 1000.0;
        (detector, kind, result, ms)
    });
    state.pending = Some(task);
}

pub fn poll(
    mut state: ResMut<InferenceState>,
    mut handle: ResMut<DetectorHandle>,
    mut overlay: ResMut<OverlayState>,
) {
    let Some(mut task) = state.pending.take() else { return; };
    if let Some((detector, kind, result, ms)) = futures_lite::future::block_on(poll_once(&mut task)) {
        handle.detector = detector;
        handle.kind = kind;
        overlay.last_confidence = Some(result.confidence);
        overlay.last_frame = Some(result.frame_id);
        overlay.last_ms = Some(ms);
    } else {
        state.pending = Some(task);
    }
}
```

## src/overlay.rs
```rust,ignore
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct OverlayState {
    pub last_frame: Option<u64>,
    pub last_confidence: Option<f32>,
    pub last_ms: Option<f32>,
}
```
