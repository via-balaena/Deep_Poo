use burn::backend::Autodiff;
use burn::module::Module;
use burn::nn::loss::{MseLoss, Reduction};
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings, RecorderError};
use burn::tensor::{Tensor, TensorData};
use burn_dataset::WarehouseLoaders;
use std::path::Path;

use crate::{
    ConvolutionalDetector, ConvolutionalDetectorConfig, DatasetPathConfig, LinearDetector,
    LinearDetectorConfig, TrainBackend,
};
use clap::{Parser, ValueEnum};
use std::fs;

pub fn load_linear_detector_from_checkpoint<P: AsRef<Path>>(
    path: P,
    device: &<TrainBackend as burn::tensor::backend::Backend>::Device,
) -> Result<LinearDetector<TrainBackend>, RecorderError> {
    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    LinearDetector::<TrainBackend>::new(LinearDetectorConfig::default(), device).load_file(
        path.as_ref(),
        &recorder,
        device,
    )
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ModelKind {
    Tiny,
    Big,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum BackendKind {
    NdArray,
    Wgpu,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum TrainingInputSource {
    Warehouse,
    CaptureLogs,
}

#[derive(Parser, Debug)]
#[command(
    name = "train",
    about = "Train LinearDetector/ConvolutionalDetector (warehouse-first)"
)]
pub struct TrainArgs {
    /// Model to train.
    #[arg(long, value_enum, default_value_t = ModelKind::Tiny)]
    pub model: ModelKind,
    /// Backend to use (ndarray or wgpu if enabled).
    #[arg(long, value_enum, default_value_t = BackendKind::NdArray)]
    pub backend: BackendKind,
    /// Maximum boxes per image (pads/truncates to this for training).
    #[arg(long, default_value_t = 64)]
    pub max_boxes: usize,
    /// Loss weight for box regression.
    #[arg(long, default_value_t = 1.0)]
    pub lambda_box: f32,
    /// Loss weight for objectness.
    #[arg(long, default_value_t = 1.0)]
    pub lambda_obj: f32,
    /// Training input source (warehouse by default).
    #[arg(long, value_enum, default_value_t = TrainingInputSource::Warehouse)]
    pub input_source: TrainingInputSource,
    /// Warehouse manifest path (used with --input-source warehouse).
    #[arg(long, default_value = "assets/warehouse/manifest.json")]
    pub warehouse_manifest: String,
    /// Capture-log dataset root containing labels/ and images/.
    #[arg(long, default_value = "assets/datasets/captures_filtered")]
    pub dataset_root: String,
    /// Labels subdirectory relative to dataset root (capture-logs only).
    #[arg(long, default_value = "labels")]
    pub labels_subdir: String,
    /// Images subdirectory relative to dataset root (capture-logs only).
    #[arg(long, default_value = ".")]
    pub images_subdir: String,
    /// Number of epochs.
    #[arg(long, default_value_t = 1)]
    pub epochs: usize,
    /// Batch size.
    #[arg(long, default_value_t = 1)]
    pub batch_size: usize,
    /// Learning rate.
    #[arg(long, default_value_t = 1e-3)]
    pub lr: f32,
    /// Objectness threshold (for future eval).
    #[arg(long, default_value_t = 0.3)]
    pub infer_obj_thresh: f32,
    /// IoU threshold (for future eval).
    #[arg(long, default_value_t = 0.5)]
    pub infer_iou_thresh: f32,
    /// Checkpoint output path (defaults by model if not provided).
    #[arg(long)]
    pub checkpoint_out: Option<String>,
}

pub fn run_train(args: TrainArgs) -> anyhow::Result<()> {
    validate_backend_choice(args.backend)?;

    let ckpt_path = args
        .checkpoint_out
        .clone()
        .unwrap_or_else(|| match args.model {
            ModelKind::Tiny => "checkpoints/linear_detector.bin".to_string(),
            ModelKind::Big => "checkpoints/convolutional_detector.bin".to_string(),
        });

    if let Some(parent) = Path::new(&ckpt_path).parent() {
        fs::create_dir_all(parent)?;
    }

    match args.input_source {
        TrainingInputSource::Warehouse => {
            let manifest_path = Path::new(&args.warehouse_manifest);
            let loaders = WarehouseLoaders::from_manifest_path(manifest_path, 0.0, None, false)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "failed to load warehouse manifest at {}: {e}",
                        manifest_path.display()
                    )
                })?;
            if loaders.train_len() == 0 {
                anyhow::bail!(
                    "warehouse manifest {} contains no training shards",
                    manifest_path.display()
                );
            }
            match args.model {
                ModelKind::Tiny => train_linear_detector_warehouse(&args, &loaders, &ckpt_path)?,
                ModelKind::Big => {
                    train_convolutional_detector_warehouse(&args, &loaders, &ckpt_path)?
                }
            }
        }
        TrainingInputSource::CaptureLogs => {
            println!("training from capture logs (legacy path); prefer warehouse manifests");
            let cfg = DatasetPathConfig {
                root: args.dataset_root.clone().into(),
                labels_subdir: args.labels_subdir.clone(),
                images_subdir: args.images_subdir.clone(),
            };
            let samples = cfg.load()?;
            if samples.is_empty() {
                println!("No samples found under {}", cfg.root.display());
                return Ok(());
            }
            match args.model {
                ModelKind::Tiny => train_linear_detector(&args, &samples, &ckpt_path)?,
                ModelKind::Big => train_convolutional_detector(&args, &samples, &ckpt_path)?,
            }
        }
    }

    println!("Saved checkpoint to {}", ckpt_path);
    Ok(())
}

type ADBackend = Autodiff<TrainBackend>;

fn train_linear_detector(
    args: &TrainArgs,
    samples: &[crate::RunSample],
    ckpt_path: &str,
) -> anyhow::Result<()> {
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = LinearDetector::<ADBackend>::new(LinearDetectorConfig::default(), &device);
    let mut optim = AdamConfig::new().init();

    let batch_size = args.batch_size.max(1);
    let data = samples.to_vec();
    for epoch in 0..args.epochs {
        let mut losses = Vec::new();
        for batch in data.chunks(batch_size) {
            let batch = crate::collate::<ADBackend>(batch, args.max_boxes)?;
            // Feature: take the first box (or zeros) as the input vector.
            let boxes = batch.boxes.clone();
            let first_box = boxes
                .clone()
                .slice([0..boxes.dims()[0], 0..1, 0..4])
                .reshape([boxes.dims()[0], 4]);

            // Target: 1.0 if any box present, else 0.0.
            let mask = batch.box_mask.clone();
            let has_box = mask.clone().sum_dim(1).reshape([mask.dims()[0], 1]);

            let preds = model.forward(first_box);
            let mse = MseLoss::new();
            let loss = mse.forward(preds, has_box, Reduction::Mean);
            let loss_detached = loss.clone().detach();
            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = optim.step(args.lr as f64, model, grads);

            let loss_val: f32 = loss_detached
                .into_data()
                .to_vec::<f32>()
                .unwrap_or_default()
                .into_iter()
                .next()
                .unwrap_or(0.0);
            losses.push(loss_val);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4}");
    }

    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    model
        .clone()
        .save_file(Path::new(ckpt_path), &recorder)
        .map_err(|e| anyhow::anyhow!("failed to save checkpoint: {e}"))?;

    Ok(())
}

fn train_linear_detector_warehouse(
    args: &TrainArgs,
    loaders: &WarehouseLoaders,
    ckpt_path: &str,
) -> anyhow::Result<()> {
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = LinearDetector::<ADBackend>::new(LinearDetectorConfig::default(), &device);
    let mut optim = AdamConfig::new().init();

    let batch_size = args.batch_size.max(1);
    for epoch in 0..args.epochs {
        let mut losses = Vec::new();
        let mut iter = loaders.train_iter();
        loop {
            let batch = match iter.next_batch::<ADBackend>(batch_size, &device)? {
                Some(batch) => batch,
                None => break,
            };
            let batch = crate::collate_from_burn_batch::<ADBackend>(batch, args.max_boxes)?;

            // Feature: take the first box (or zeros) as the input vector.
            let boxes = batch.boxes.clone();
            let first_box = boxes
                .clone()
                .slice([0..boxes.dims()[0], 0..1, 0..4])
                .reshape([boxes.dims()[0], 4]);

            // Target: 1.0 if any box present, else 0.0.
            let mask = batch.box_mask.clone();
            let has_box = mask.clone().sum_dim(1).reshape([mask.dims()[0], 1]);

            let preds = model.forward(first_box);
            let mse = MseLoss::new();
            let loss = mse.forward(preds, has_box, Reduction::Mean);
            let loss_detached = loss.clone().detach();
            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = optim.step(args.lr as f64, model, grads);

            let loss_val: f32 = loss_detached
                .into_data()
                .to_vec::<f32>()
                .unwrap_or_default()
                .into_iter()
                .next()
                .unwrap_or(0.0);
            losses.push(loss_val);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4}");
    }

    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    model
        .clone()
        .save_file(Path::new(ckpt_path), &recorder)
        .map_err(|e| anyhow::anyhow!("failed to save checkpoint: {e}"))?;

    Ok(())
}

fn train_convolutional_detector(
    args: &TrainArgs,
    samples: &[crate::RunSample],
    ckpt_path: &str,
) -> anyhow::Result<()> {
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = ConvolutionalDetector::<ADBackend>::new(
        ConvolutionalDetectorConfig {
            input_dim: Some(4 + 8), // first box (4) + features (8)
            max_boxes: args.max_boxes,
            ..Default::default()
        },
        &device,
    );
    let mut optim = AdamConfig::new().init();

    let batch_size = args.batch_size.max(1);
    let data = samples.to_vec();
    for epoch in 0..args.epochs {
        let mut losses = Vec::new();
        for batch in data.chunks(batch_size) {
            let batch = crate::collate::<ADBackend>(batch, args.max_boxes)?;
            // Features: first box (or zeros) + pooled image features.
            let boxes = batch.boxes.clone();
            let first_box = boxes
                .clone()
                .slice([0..boxes.dims()[0], 0..1, 0..4])
                .reshape([boxes.dims()[0], 4]);
            let features = batch.features.clone();
            let input = burn::tensor::Tensor::cat(vec![first_box, features], 1);

            let (pred_boxes, pred_scores) = model.forward_multibox(input);

            // Targets
            let gt_boxes = batch.boxes.clone();
            let gt_mask = batch.box_mask.clone();

            // Greedy matching per GT: for each GT box, pick best pred by IoU.
            let (obj_targets, box_targets, box_weights) =
                build_greedy_targets(pred_boxes.clone(), gt_boxes.clone(), gt_mask.clone());
            // Greedy IoU matcher is deterministic/cheap; swap to Hungarian if finer matching is needed later.

            // Objectness loss (BCE) with targets; unassigned preds stay at 0.0.
            let eps = 1e-6;
            let pred_scores_clamped = pred_scores.clamp(eps, 1.0 - eps);
            let obj_targets_inv =
                Tensor::<ADBackend, 2>::ones(obj_targets.dims(), &obj_targets.device())
                    - obj_targets.clone();
            let obj_loss = -((obj_targets.clone() * pred_scores_clamped.clone().log())
                + (obj_targets_inv
                    * (Tensor::<ADBackend, 2>::ones(
                        pred_scores_clamped.dims(),
                        &pred_scores_clamped.device(),
                    ) - pred_scores_clamped)
                        .log()))
            .sum()
            .div_scalar((obj_targets.dims()[0] * obj_targets.dims()[1]) as f32);

            // Box regression loss on matched preds only.
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
                // Return a zero scalar in the same tensor rank as div output (rank 1).
                let zeros = vec![0.0f32; 1];
                Tensor::<ADBackend, 1>::from_data(
                    TensorData::new(zeros, [1]),
                    &box_weights.device(),
                )
            };

            let loss = box_loss * args.lambda_box + obj_loss * args.lambda_obj;
            let loss_detached = loss.clone().detach();
            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = optim.step(args.lr as f64, model, grads);

            let loss_val: f32 = loss_detached
                .into_data()
                .to_vec::<f32>()
                .unwrap_or_default()
                .into_iter()
                .next()
                .unwrap_or(0.0);
            losses.push(loss_val);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4}");
    }

    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    model
        .clone()
        .save_file(Path::new(ckpt_path), &recorder)
        .map_err(|e| anyhow::anyhow!("failed to save checkpoint: {e}"))?;

    Ok(())
}

fn train_convolutional_detector_warehouse(
    args: &TrainArgs,
    loaders: &WarehouseLoaders,
    ckpt_path: &str,
) -> anyhow::Result<()> {
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = ConvolutionalDetector::<ADBackend>::new(
        ConvolutionalDetectorConfig {
            input_dim: Some(4 + 8), // first box (4) + features (8)
            max_boxes: args.max_boxes,
            ..Default::default()
        },
        &device,
    );
    let mut optim = AdamConfig::new().init();

    let batch_size = args.batch_size.max(1);
    for epoch in 0..args.epochs {
        let mut losses = Vec::new();
        let mut iter = loaders.train_iter();
        loop {
            let batch = match iter.next_batch::<ADBackend>(batch_size, &device)? {
                Some(batch) => batch,
                None => break,
            };
            let batch = crate::collate_from_burn_batch::<ADBackend>(batch, args.max_boxes)?;

            let boxes = batch.boxes.clone();
            let first_box = boxes
                .clone()
                .slice([0..boxes.dims()[0], 0..1, 0..4])
                .reshape([boxes.dims()[0], 4]);
            let features = batch.features.clone();
            let input = burn::tensor::Tensor::cat(vec![first_box, features], 1);

            let (pred_boxes, pred_scores) = model.forward_multibox(input);

            let gt_boxes = batch.boxes.clone();
            let gt_mask = batch.box_mask.clone();

            let (obj_targets, box_targets, box_weights) =
                build_greedy_targets(pred_boxes.clone(), gt_boxes.clone(), gt_mask.clone());

            let eps = 1e-6;
            let pred_scores_clamped = pred_scores.clamp(eps, 1.0 - eps);
            let obj_targets_inv =
                Tensor::<ADBackend, 2>::ones(obj_targets.dims(), &obj_targets.device())
                    - obj_targets.clone();
            let obj_loss = -((obj_targets.clone() * pred_scores_clamped.clone().log())
                + (obj_targets_inv
                    * (Tensor::<ADBackend, 2>::ones(
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
                Tensor::<ADBackend, 1>::from_data(
                    TensorData::new(zeros, [1]),
                    &box_weights.device(),
                )
            };

            let loss = box_loss * args.lambda_box + obj_loss * args.lambda_obj;
            let loss_detached = loss.clone().detach();
            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = optim.step(args.lr as f64, model, grads);

            let loss_val: f32 = loss_detached
                .into_data()
                .to_vec::<f32>()
                .unwrap_or_default()
                .into_iter()
                .next()
                .unwrap_or(0.0);
            losses.push(loss_val);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4}");
    }

    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    model
        .clone()
        .save_file(Path::new(ckpt_path), &recorder)
        .map_err(|e| anyhow::anyhow!("failed to save checkpoint: {e}"))?;

    Ok(())
}

pub fn validate_backend_choice(kind: BackendKind) -> anyhow::Result<()> {
    let built_wgpu = cfg!(feature = "backend-wgpu");
    match (kind, built_wgpu) {
        (BackendKind::Wgpu, false) => {
            anyhow::bail!("backend-wgpu feature not enabled; rebuild with --features backend-wgpu or choose ndarray backend")
        }
        (BackendKind::NdArray, true) => {
            println!("note: built with backend-wgpu; training will still use the WGPU backend despite --backend ndarray");
        }
        _ => {}
    }
    Ok(())
}

fn iou_xyxy(a: [f32; 4], b: [f32; 4]) -> f32 {
    let ax0 = a[0].min(a[2]);
    let ay0 = a[1].min(a[3]);
    let ax1 = a[0].max(a[2]);
    let ay1 = a[1].max(a[3]);
    let bx0 = b[0].min(b[2]);
    let by0 = b[1].min(b[3]);
    let bx1 = b[0].max(b[2]);
    let by1 = b[1].max(b[3]);

    let inter_x0 = ax0.max(bx0);
    let inter_y0 = ay0.max(by0);
    let inter_x1 = ax1.min(bx1);
    let inter_y1 = ay1.min(by1);

    let inter_w = (inter_x1 - inter_x0).max(0.0);
    let inter_h = (inter_y1 - inter_y0).max(0.0);
    let inter_area = inter_w * inter_h;

    let area_a = (ax1 - ax0).max(0.0) * (ay1 - ay0).max(0.0);
    let area_b = (bx1 - bx0).max(0.0) * (by1 - by0).max(0.0);
    let denom = area_a + area_b - inter_area;
    if denom <= 0.0 {
        0.0
    } else {
        inter_area / denom
    }
}
pub fn load_convolutional_detector_from_checkpoint<P: AsRef<Path>>(
    path: P,
    device: &<TrainBackend as burn::tensor::backend::Backend>::Device,
    max_boxes: usize,
) -> Result<ConvolutionalDetector<TrainBackend>, RecorderError> {
    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    ConvolutionalDetector::<TrainBackend>::new(
        ConvolutionalDetectorConfig {
            max_boxes,
            input_dim: Some(4 + 8),
            ..Default::default()
        },
        device,
    )
    .load_file(path.as_ref(), &recorder, device)
}

pub fn build_greedy_targets<B: burn::tensor::backend::Backend>(
    pred_boxes: Tensor<B, 3>,
    gt_boxes: Tensor<B, 3>,
    gt_mask: Tensor<B, 2>,
) -> (Tensor<B, 2>, Tensor<B, 3>, Tensor<B, 3>) {
    let batch = pred_boxes.dims()[0];
    let max_pred = pred_boxes.dims()[1];
    let max_gt = gt_boxes.dims()[1];

    let gt_mask_vec = gt_mask
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default();
    let gt_boxes_vec = gt_boxes
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default();
    let pred_boxes_vec = pred_boxes
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default();

    let mut obj_targets = vec![0.0f32; batch * max_pred];
    let mut box_targets = vec![0.0f32; batch * max_pred * 4];
    let mut box_weights = vec![0.0f32; batch * max_pred * 4];

    for b in 0..batch {
        for g in 0..max_gt {
            let mask_idx = b * max_gt + g;
            if gt_mask_vec.get(mask_idx).copied().unwrap_or(0.0) < 0.5 {
                continue;
            }
            let gb = [
                gt_boxes_vec[(b * max_gt + g) * 4],
                gt_boxes_vec[(b * max_gt + g) * 4 + 1],
                gt_boxes_vec[(b * max_gt + g) * 4 + 2],
                gt_boxes_vec[(b * max_gt + g) * 4 + 3],
            ];

            let mut best_iou = -1.0f32;
            let mut best_p = 0usize;
            for p in 0..max_pred {
                let pb = [
                    pred_boxes_vec[(b * max_pred + p) * 4],
                    pred_boxes_vec[(b * max_pred + p) * 4 + 1],
                    pred_boxes_vec[(b * max_pred + p) * 4 + 2],
                    pred_boxes_vec[(b * max_pred + p) * 4 + 3],
                ];
                let iou = iou_xyxy(pb, gb);
                if iou > best_iou {
                    best_iou = iou;
                    best_p = p;
                }
            }

            let obj_idx = b * max_pred + best_p;
            obj_targets[obj_idx] = 1.0;
            let bt_base = (b * max_pred + best_p) * 4;
            box_targets[bt_base..bt_base + 4].copy_from_slice(&gb);
            box_weights[bt_base..bt_base + 4].copy_from_slice(&[1.0, 1.0, 1.0, 1.0]);
        }
    }

    let device = &B::Device::default();
    let obj_targets =
        Tensor::<B, 2>::from_data(TensorData::new(obj_targets, [batch, max_pred]), device);
    let box_targets =
        Tensor::<B, 3>::from_data(TensorData::new(box_targets, [batch, max_pred, 4]), device);
    let box_weights =
        Tensor::<B, 3>::from_data(TensorData::new(box_weights, [batch, max_pred, 4]), device);

    (obj_targets, box_targets, box_weights)
}
