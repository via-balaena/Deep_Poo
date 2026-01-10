use data_contracts::capture::{CaptureMetadata, PolypLabel, ValidationError};

#[test]
fn invalid_bbox_norm_rejected() {
    let meta = CaptureMetadata {
        frame_id: 0,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "images/frame.png".into(),
        image_present: true,
        camera_active: true,
        polyp_seed: 42,
        polyp_labels: vec![PolypLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: None,
            bbox_norm: Some([0.8, 0.2, 0.1, 0.9]),
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
        polyp_seed: 42,
        polyp_labels: vec![PolypLabel {
            center_world: [0.0, 0.0, 0.0],
            bbox_px: Some([0.0, 0.0, 10.0, 10.0]),
            bbox_norm: Some([0.1, 0.1, 0.2, 0.2]),
        }],
    };
    assert!(meta.validate().is_ok());
}
