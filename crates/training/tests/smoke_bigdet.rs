use burn::backend::{ndarray::NdArray, Autodiff};
use burn::module::Module;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings};
use data_contracts::capture::{CaptureMetadata, DetectionLabel};
use std::fs;
use std::path::PathBuf;
use training::dataset::{collate, DatasetPathConfig};
use training::{ConvolutionalDetector, ConvolutionalDetectorConfig};

type ADBackend = Autodiff<NdArray<f32>>;

fn synthetic_dataset(tmp: &tempfile::TempDir) -> anyhow::Result<Vec<training::RunSample>> {
    let labels_dir = tmp.path().join("labels");
    fs::create_dir_all(&labels_dir)?;
    let meta = CaptureMetadata {
        frame_id: 1,
        sim_time: 0.0,
        unix_time: 0.0,
        image: "frame_00001.png".into(),
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
    let json = serde_json::to_vec(&meta)?;
    fs::write(labels_dir.join("frame_00001.json"), json)?;

    // Tiny 2x2 red image.
    let img = image::RgbImage::from_fn(2, 2, |_x, _y| image::Rgb([255, 0, 0]));
    let img_path = tmp.path().join("frame_00001.png");
    img.save(&img_path)?;

    let cfg = DatasetPathConfig {
        root: PathBuf::from(tmp.path()),
        labels_subdir: "labels".into(),
        images_subdir: ".".into(),
    };
    cfg.load()
}

#[test]
fn smoke_train_step_bigdet() {
    let temp = tempfile::tempdir().unwrap();
    let samples = synthetic_dataset(&temp).unwrap();
    assert_eq!(samples.len(), 1);

    let batch = collate::<ADBackend>(&samples, 4).unwrap();
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();

    let mut model = ConvolutionalDetector::<ADBackend>::new(
        ConvolutionalDetectorConfig {
            max_boxes: 4,
            input_dim: Some(4 + 8),
            ..Default::default()
        },
        &device,
    );
    let mut optim = AdamConfig::new().init();

    let boxes = batch.boxes.clone();
    let first_box = boxes
        .clone()
        .slice([0..boxes.dims()[0], 0..1, 0..4])
        .reshape([boxes.dims()[0], 4]);
    let features = batch.features.clone();
    let input = burn::tensor::Tensor::cat(vec![first_box, features], 1);

    let (pred_boxes, pred_scores) = model.forward_multibox(input);
    let (obj_targets, box_targets, box_weights) = training::util::build_greedy_targets(
        pred_boxes.clone(),
        batch.boxes.clone(),
        batch.box_mask.clone(),
    );

    // Simple loss as in training loop.
    let eps = 1e-6;
    let pred_scores_clamped = pred_scores.clamp(eps, 1.0 - eps);
    let obj_targets_inv =
        burn::tensor::Tensor::<ADBackend, 2>::ones(obj_targets.dims(), &obj_targets.device())
            - obj_targets.clone();
    let obj_loss = -((obj_targets.clone() * pred_scores_clamped.clone().log())
        + (obj_targets_inv
            * (burn::tensor::Tensor::<ADBackend, 2>::ones(
                pred_scores_clamped.dims(),
                &pred_scores_clamped.device(),
            ) - pred_scores_clamped)
                .log()))
    .sum()
    .div_scalar((obj_targets.dims()[0] * obj_targets.dims()[1]) as f32);

    let box_err = (pred_boxes - box_targets.clone()).abs() * box_weights.clone();
    let matched = box_weights.clone().sum().div_scalar(4.0);
    let matched_scalar = matched
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default()
        .first()
        .copied()
        .unwrap_or(0.0);
    let box_loss = if matched_scalar > 0.0 {
        box_err.sum().div_scalar(matched_scalar)
    } else {
        let zeros = vec![0.0f32; 1];
        burn::tensor::Tensor::<ADBackend, 1>::from_data(
            burn::tensor::TensorData::new(zeros, [1]),
            &box_weights.device(),
        )
    };

    let loss = box_loss + obj_loss;
    let loss_val: f32 = loss
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default()
        .into_iter()
        .next()
        .unwrap_or(0.0);
    assert!(loss_val.is_finite());

    let grads = GradientsParams::from_grads(loss.backward(), &model);
    model = optim.step(1e-3, model, grads);

    // Save/load sanity
    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    let ckpt = temp.path().join("bigdet_test.bin");
    model
        .clone()
        .save_file(&ckpt, &recorder)
        .expect("save checkpoint");
    let _loaded = ConvolutionalDetector::<ADBackend>::new(
        ConvolutionalDetectorConfig {
            max_boxes: 4,
            input_dim: Some(4 + 8),
            ..Default::default()
        },
        &device,
    )
    .load_file(&ckpt, &recorder, &device)
    .expect("load checkpoint");
}
