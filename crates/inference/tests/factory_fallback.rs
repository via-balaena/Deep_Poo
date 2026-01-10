use inference::prelude::{InferenceFactory, InferenceThresholds};
use vision_core::prelude::Frame;

#[test]
fn factory_uses_heuristic_without_weights() {
    // No weights path provided -> should fall back to heuristic detector without panic.
    let factory = InferenceFactory;
    let mut detector = factory.build(
        InferenceThresholds {
            obj_thresh: 0.3,
            iou_thresh: 0.5,
        },
        None,
    );

    // Blank 1x1 frame with no RGBA data.
    let frame = Frame {
        id: 0,
        timestamp: 0.0,
        rgba: None,
        size: (1, 1),
        path: None,
    };

    let result = detector.detect(&frame);
    // Heuristic returns empty boxes/scores; just assert it runs and echoes the frame id.
    assert_eq!(result.frame_id, 0);
    assert!(result.boxes.is_empty());
    assert!(result.scores.is_empty());
}
