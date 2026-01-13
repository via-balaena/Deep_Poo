use data_contracts::capture::{CaptureMetadata, DetectionLabel, LabelSource, ValidationError};

#[test]
fn invalid_bbox_norm_rejected() {
    let meta = CaptureMetadata {
        frame_id: 0,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "images/frame.png".into(),
        image_present: true,
        camera_active: true,
        label_seed: 42,
        labels: vec![DetectionLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: None,
            bbox_norm: Some([0.8, 0.2, 0.1, 0.9]),
            source: None,
            source_confidence: None,
        }],
    };
    let err = meta.validate().unwrap_err();
    matches!(err, ValidationError::InvalidBboxNorm(_));
}

#[test]
fn valid_bbox_passes() {
    let meta = CaptureMetadata {
        frame_id: 0,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "images/frame.png".into(),
        image_present: true,
        camera_active: true,
        label_seed: 42,
        labels: vec![DetectionLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: Some([0.0, 0.0, 10.0, 10.0]),
            bbox_norm: Some([0.1, 0.1, 0.2, 0.2]),
            source: None,
            source_confidence: None,
        }],
    };
    assert!(meta.validate().is_ok());
}

#[test]
fn provenance_roundtrip_validates() {
    let meta = CaptureMetadata {
        frame_id: 0,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "images/frame.png".into(),
        image_present: true,
        camera_active: true,
        label_seed: 7,
        labels: vec![DetectionLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: Some([0.0, 0.0, 10.0, 10.0]),
            bbox_norm: Some([0.1, 0.1, 0.2, 0.2]),
            source: Some(LabelSource::Model),
            source_confidence: Some(0.75),
        }],
    };
    let json = serde_json::to_vec(&meta).unwrap();
    let decoded: CaptureMetadata = serde_json::from_slice(&json).unwrap();
    assert!(decoded.validate().is_ok());
}
