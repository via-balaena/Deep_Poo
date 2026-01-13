use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use inference::InferenceThresholds;
use vision_core::interfaces::{self, DetectionResult, Detector, Frame};
use vision_runtime::prelude::{
    AsyncInferenceState, CapturePlugin, DetectionOverlayState, DetectorHandle, DetectorKind,
    InferencePlugin, InferenceThresholdsResource,
};

struct DummyDetector;
impl Detector for DummyDetector {
    fn detect(&mut self, frame: &Frame) -> DetectionResult {
        DetectionResult {
            frame_id: frame.id,
            positive: true,
            confidence: 0.9,
            boxes: vec![[0.1, 0.1, 0.2, 0.2]],
            scores: vec![0.9],
        }
    }
}

// Smoke test: ensure the plugins add resources/systems without panicking and that the overlay
// updates when the detector kind is heuristic.
#[test]
fn inference_plugin_smoke_updates_overlay() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(DetectorHandle {
            detector: Box::new(DummyDetector),
            kind: DetectorKind::Heuristic,
        })
        .insert_resource(InferenceThresholdsResource(InferenceThresholds {
            obj_thresh: 0.3,
            iou_thresh: 0.5,
        }))
        .add_plugins(CapturePlugin)
        .add_plugins(InferencePlugin)
        .insert_resource(Assets::<Image>::default())
        .insert_resource(sim_core::SimRunMode::Inference)
        .insert_resource(ButtonInput::<KeyCode>::default());

    app.update();
    // Simulate a completed task by enqueuing a ready task and calling poll system.
    {
        let mut jobs = app
            .world_mut()
            .get_resource_mut::<AsyncInferenceState>()
            .unwrap();
        let task = AsyncComputeTaskPool::get().spawn(async {
            (
                Box::new(DummyDetector) as Box<dyn interfaces::Detector + Send + Sync>,
                DetectorKind::Heuristic,
                DetectionResult {
                    frame_id: 1,
                    positive: true,
                    confidence: 0.8,
                    boxes: vec![[0.1, 0.1, 0.2, 0.2]],
                    scores: vec![0.8],
                },
                1.0f32,
                (64, 64),
            )
        });
        jobs.pending = Some(task);
    }
    // Poll a few times to allow the async task to complete.
    for _ in 0..3 {
        app.world_mut()
            .run_system_once(vision_runtime::poll_inference_task)
            .unwrap();
        app.update();
    }
    let overlay = app.world().get_resource::<DetectionOverlayState>().unwrap();
    assert_eq!(overlay.boxes.len(), 1);
}
