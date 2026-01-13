use clap::Parser;
use training::dataset::{collate, DatasetPathConfig};
use training::util::{
    load_convolutional_detector_from_checkpoint, load_linear_detector_from_checkpoint, BackendKind,
    ModelKind,
};
use training::{
    ConvolutionalDetector, ConvolutionalDetectorConfig, LinearDetector, LinearDetectorConfig,
    TrainBackend,
};

#[derive(Parser, Debug)]
#[command(
    name = "eval",
    about = "Evaluate LinearDetector/ConvolutionalDetector checkpoint on a dataset (precision/recall by IoU)"
)]
struct Args {
    /// Model to evaluate.
    #[arg(long, value_enum, default_value_t = ModelKind::Tiny)]
    model: ModelKind,
    /// Backend to use (ndarray or wgpu if enabled).
    #[arg(long, value_enum, default_value_t = BackendKind::NdArray)]
    backend: BackendKind,
    /// Dataset root containing labels/ and images/ (uses data_contracts schemas).
    #[arg(long, default_value = "assets/datasets/captures_filtered")]
    dataset_root: String,
    /// Labels subdirectory relative to dataset root.
    #[arg(long, default_value = "labels")]
    labels_subdir: String,
    /// Images subdirectory relative to dataset root.
    #[arg(long, default_value = ".")]
    images_subdir: String,
    /// Maximum boxes per image (pads/truncates to this for eval batch collation).
    #[arg(long, default_value_t = 64)]
    max_boxes: usize,
    /// Checkpoint path to load.
    #[arg(long)]
    checkpoint: Option<String>,
    /// IoU threshold for true positive.
    #[arg(long, default_value_t = 0.5)]
    iou_thresh: f32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    training::util::validate_backend_choice(args.backend)?;

    let cfg = DatasetPathConfig {
        root: args.dataset_root.into(),
        labels_subdir: args.labels_subdir,
        images_subdir: args.images_subdir,
    };
    let samples = cfg.load()?;
    if samples.is_empty() {
        println!("No samples found under {}", cfg.root.display());
        return Ok(());
    }

    let device = <TrainBackend as burn::tensor::backend::Backend>::Device::default();
    let ckpt = args.checkpoint.clone();

    // Build collate batches
    let batch_size = 8usize;
    let mut total_tp = 0f32;
    let mut total_fp = 0f32;
    let mut total_fn = 0f32;

    match args.model {
        ModelKind::Tiny => {
            let model = match ckpt {
                Some(ref p) => {
                    load_linear_detector_from_checkpoint(p, &device).unwrap_or_else(|e| {
                        println!("Failed to load checkpoint {p}; using fresh model ({e})");
                        LinearDetector::<TrainBackend>::new(
                            LinearDetectorConfig::default(),
                            &device,
                        )
                    })
                }
                None => {
                    println!("No checkpoint provided; using fresh LinearDetector");
                    LinearDetector::<TrainBackend>::new(LinearDetectorConfig::default(), &device)
                }
            };
            for chunk in samples.chunks(batch_size) {
                let batch = collate::<TrainBackend>(chunk, args.max_boxes)?;
                let boxes = batch.boxes.clone();
                let first_box = boxes
                    .clone()
                    .slice([0..boxes.dims()[0], 0..1, 0..4])
                    .reshape([boxes.dims()[0], 4]);

                let mask = batch.box_mask.clone();
                let has_box = mask.clone().sum_dim(1).reshape([mask.dims()[0], 1]);

                let preds = model.forward(first_box);
                // Treat preds > 0.5 as positive.
                let preds_vec: Vec<f32> = preds.into_data().to_vec::<f32>().unwrap_or_default();
                let has_box_vec: Vec<f32> = has_box.into_data().to_vec::<f32>().unwrap_or_default();
                for (p, t) in preds_vec.into_iter().zip(has_box_vec.into_iter()) {
                    let pred_pos = p > 0.5;
                    let gt_pos = t > 0.5;
                    match (pred_pos, gt_pos) {
                        (true, true) => total_tp += 1.0,
                        (true, false) => total_fp += 1.0,
                        (false, true) => total_fn += 1.0,
                        (false, false) => {}
                    }
                }
            }
        }
        ModelKind::Big => {
            let model = match ckpt {
                Some(ref p) => {
                    load_convolutional_detector_from_checkpoint(p, &device, args.max_boxes)
                        .unwrap_or_else(|e| {
                            println!("Failed to load checkpoint {p}; using fresh model ({e})");
                            ConvolutionalDetector::<TrainBackend>::new(
                                ConvolutionalDetectorConfig {
                                    input_dim: Some(4 + 8),
                                    max_boxes: args.max_boxes,
                                    ..Default::default()
                                },
                                &device,
                            )
                        })
                }
                None => {
                    println!("No checkpoint provided; using fresh ConvolutionalDetector");
                    ConvolutionalDetector::<TrainBackend>::new(
                        ConvolutionalDetectorConfig {
                            input_dim: Some(4 + 8),
                            max_boxes: args.max_boxes,
                            ..Default::default()
                        },
                        &device,
                    )
                }
            };
            for chunk in samples.chunks(batch_size) {
                let batch = collate::<TrainBackend>(chunk, args.max_boxes)?;
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

                // Simple IoU TP/FP: for each pred with score>0.5, see if it matches a GT.
                let pb = pred_boxes.into_data().to_vec::<f32>().unwrap_or_default();
                let ps = pred_scores.into_data().to_vec::<f32>().unwrap_or_default();
                let gb = gt_boxes.into_data().to_vec::<f32>().unwrap_or_default();
                let gm = gt_mask.into_data().to_vec::<f32>().unwrap_or_default();

                let bsz = chunk.len();
                for b in 0..bsz {
                    // Collect GT boxes for this sample.
                    let mut gt_list = Vec::new();
                    for g in 0..args.max_boxes {
                        if gm[b * args.max_boxes + g] > 0.5 {
                            gt_list.push([
                                gb[(b * args.max_boxes + g) * 4],
                                gb[(b * args.max_boxes + g) * 4 + 1],
                                gb[(b * args.max_boxes + g) * 4 + 2],
                                gb[(b * args.max_boxes + g) * 4 + 3],
                            ]);
                        }
                    }
                    let mut gt_matched = vec![false; gt_list.len()];
                    for p in 0..args.max_boxes {
                        let score = ps[b * args.max_boxes + p];
                        if score <= 0.5 {
                            continue;
                        }
                        let pb_box = [
                            pb[(b * args.max_boxes + p) * 4],
                            pb[(b * args.max_boxes + p) * 4 + 1],
                            pb[(b * args.max_boxes + p) * 4 + 2],
                            pb[(b * args.max_boxes + p) * 4 + 3],
                        ];
                        let mut matched = false;
                        for (i, gb_box) in gt_list.iter().enumerate() {
                            let iou = iou_xyxy(pb_box, *gb_box);
                            if iou >= args.iou_thresh {
                                matched = true;
                                gt_matched[i] = true;
                                break;
                            }
                        }
                        if matched {
                            total_tp += 1.0;
                        } else {
                            total_fp += 1.0;
                        }
                    }
                    for matched in gt_matched {
                        if !matched {
                            total_fn += 1.0;
                        }
                    }
                }
            }
        }
    }

    let precision = if total_tp + total_fp > 0.0 {
        total_tp / (total_tp + total_fp)
    } else {
        0.0
    };
    let recall = if total_tp + total_fn > 0.0 {
        total_tp / (total_tp + total_fn)
    } else {
        0.0
    };

    println!(
        "Eval complete: precision={:.3}, recall={:.3} (tp={}, fp={}, fn={}, iou_thresh={})",
        precision, recall, total_tp, total_fp, total_fn, args.iou_thresh
    );

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
