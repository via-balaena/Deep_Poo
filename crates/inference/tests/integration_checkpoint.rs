use inference::factory::{InferenceFactory, InferenceThresholds};
use std::env;
use std::path::PathBuf;
use vision_core::interfaces::Frame;

#[test]
fn load_checkpoint_when_provided() {
    let ckpt = match env::var("INFERENCE_CKPT") {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            eprintln!("INFERENCE_CKPT not set; skipping checkpoint integration test.");
            return;
        }
    };
    assert!(ckpt.exists(), "checkpoint path {:?} does not exist", ckpt);
    let factory = InferenceFactory;
    let mut detector = factory.build(InferenceThresholds::default(), Some(&ckpt));
    let result = detector.detect(&Frame {
        id: 0,
        timestamp: 0.0,
        rgba: None,
        size: (1, 1),
        path: None,
    });
    // We only assert it didn't panic; scores/boxes may be empty depending on the checkpoint.
    assert_eq!(result.frame_id, 0);
}
