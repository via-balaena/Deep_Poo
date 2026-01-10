use vision_runtime::DetectionOverlayState;

#[test]
fn overlay_state_defaults_empty() {
    let state = DetectionOverlayState::default();
    assert!(state.boxes.is_empty());
    assert!(state.scores.is_empty());
    assert_eq!(state.size, (0, 0));
    assert!(state.fallback.is_none());
    assert!(state.inference_ms.is_none());
}
