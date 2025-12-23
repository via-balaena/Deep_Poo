#[cfg(feature = "burn_runtime")]
mod real {
    use anyhow::Result;
    use burn::backend::Autodiff;
    #[cfg(feature = "burn_wgpu")]
    use burn_wgpu::Wgpu;
    #[cfg(not(feature = "burn_wgpu"))]
    use burn::backend::ndarray::NdArray;
    use burn::module::Module;
    use burn::record::{BinFileRecorder, FullPrecisionSettings};
    use clap::Parser;
    use colon_sim::burn_model::{nms, TinyDet, TinyDetConfig};
    use colon_sim::tools::burn_dataset::{BatchIter, BurnBatch, DatasetConfig, index_runs};
    use serde::Serialize;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[derive(Parser, Debug)]
    #[command(name = "eval", about = "Evaluate TinyDet checkpoint on a dataset")]
    struct EvalArgs {
        /// Dataset root containing runs to evaluate.
        #[arg(long, default_value = "assets/datasets/captures_filtered")]
        input_root: String,
        /// Checkpoint path to load (model only).
        #[arg(long)]
        checkpoint: String,
        /// Batch size (still limited by target assignment).
        #[arg(long, default_value_t = 1)]
        batch_size: usize,
        /// Objectness threshold for matching.
        #[arg(long, default_value_t = 0.3)]
        val_obj_thresh: f32,
        /// Base IoU threshold for NMS/matching.
        #[arg(long, default_value_t = 0.5)]
        val_iou_thresh: f32,
        /// Optional comma-separated list of IoU thresholds to sweep (e.g., "0.5,0.75").
        #[arg(long)]
        val_iou_sweep: Option<String>,
        /// Optional metrics output path (JSONL).
        #[arg(long)]
        metrics_out: Option<String>,
    }

    #[cfg(feature = "burn_wgpu")]
    type Backend = Wgpu<f32>;
    #[cfg(not(feature = "burn_wgpu"))]
    type Backend = NdArray<f32>;
    type ADBackend = Autodiff<Backend>;

    pub fn main_impl() -> Result<()> {
        let args = EvalArgs::parse();
        let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
        let mut model = TinyDet::<ADBackend>::new(TinyDetConfig::default(), &device);
        let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
        let path = Path::new(&args.checkpoint);
        model = model
            .load_file(path, &recorder, &device)
            .map_err(|e| anyhow::anyhow!("Failed to load checkpoint {}: {:?}", path.display(), e))?;
        println!("Loaded checkpoint {}", path.display());

        let cfg = DatasetConfig {
            target_size: Some((128, 128)),
            flip_horizontal_prob: 0.0,
            shuffle: false,
            ..Default::default()
        };
        let root = Path::new(&args.input_root);
        let indices = index_runs(root).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let mut val = BatchIter::from_indices(indices, cfg).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let mut iou_thresholds: Vec<f32> = vec![args.val_iou_thresh];
        if let Some(extra) = &args.val_iou_sweep {
            for part in extra.split(',') {
                if let Ok(v) = part.trim().parse::<f32>() {
                    iou_thresholds.push(v);
                }
            }
        }
        iou_thresholds.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        iou_thresholds.dedup();

        #[derive(Serialize)]
        struct Metrics {
            iou: f32,
            mean_iou: f32,
            precision: f32,
            recall: f32,
            map: f32,
            tp: usize,
            fp: usize,
            fn_: usize,
            batches: usize,
            seed: Option<u64>,
        }

        struct Accum {
            iou: f32,
            val_sum: f32,
            batches: usize,
            tp: usize,
            fp: usize,
            fn_: usize,
            matched: usize,
            pr_curve: Vec<(f32, usize, usize, usize)>,
        }

        let mut accums: Vec<Accum> = iou_thresholds
            .iter()
            .map(|iou| Accum {
                iou: *iou,
                val_sum: 0.0,
                batches: 0,
                tp: 0,
                fp: 0,
                fn_: 0,
                matched: 0,
                pr_curve: (1..=19)
                    .map(|i| (i as f32 * 0.05, 0usize, 0usize, 0usize))
                    .collect(),
            })
            .collect();

        while let Some(batch) = val
            .next_batch::<ADBackend>(args.batch_size, &device)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
        {
            let (obj, boxes) = model.forward(batch.images.clone());
            for acc in accums.iter_mut() {
                let (iou_sum, matched_count, batch_tp, batch_fp, batch_fn) =
                    val_metrics_nms(&obj, &boxes, &batch, args.val_obj_thresh, acc.iou);
                acc.val_sum += iou_sum;
                acc.matched += matched_count;
                acc.tp += batch_tp;
                acc.fp += batch_fp;
                acc.fn_ += batch_fn;
                for entry in acc.pr_curve.iter_mut() {
                    let th = entry.0;
                    let (btp, bfp, bfn) = val_pr_threshold(&obj, &boxes, &batch, th, acc.iou);
                    entry.1 += btp;
                    entry.2 += bfp;
                    entry.3 += bfn;
                }
                acc.batches += 1;
            }
        }

        let mut results = Vec::new();
        for acc in accums.iter() {
            if acc.batches == 0 {
                continue;
            }
            let mean_iou = if acc.matched > 0 {
                acc.val_sum / acc.matched as f32
            } else {
                0.0
            };
            let precision = if acc.tp + acc.fp > 0 {
                acc.tp as f32 / (acc.tp + acc.fp) as f32
            } else {
                0.0
            };
            let recall = if acc.tp + acc.fn_ > 0 {
                acc.tp as f32 / (acc.tp + acc.fn_) as f32
            } else {
                0.0
            };
            let mut pr_points: Vec<(f32, usize, usize, usize)> = acc.pr_curve.clone();
            pr_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let map = compute_map(&pr_points);
            println!(
                "eval@IoU{:.2}: mean IoU {:.4}, precision {:.3}, recall {:.3}, mAP {:.3} (tp/fp/fn {}/{}/{}, batches {})",
                acc.iou, mean_iou, precision, recall, map, acc.tp, acc.fp, acc.fn_, acc.batches
            );
            results.push(Metrics {
                iou: acc.iou,
                mean_iou,
                precision,
                recall,
                map,
                tp: acc.tp,
                fp: acc.fp,
                fn_: acc.fn_,
                batches: acc.batches,
                seed: None,
            });
        }

        if let Some(path) = &args.metrics_out {
            if let Some(parent) = Path::new(path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
                for m in results.iter() {
                    let _ = writeln!(f, "{}", serde_json::to_string(m).unwrap_or_default());
                }
                println!("Wrote eval metrics to {}", path);
            }
        }

        Ok(())
    }

    fn val_pr_threshold(
        obj_logits: &burn::tensor::Tensor<ADBackend, 4>,
        box_logits: &burn::tensor::Tensor<ADBackend, 4>,
        batch: &BurnBatch<ADBackend>,
        obj_thresh: f32,
        iou_thresh: f32,
    ) -> (usize, usize, usize) {
        let (_, _, tp, fp, fn_) =
            val_metrics_nms(obj_logits, box_logits, batch, obj_thresh, iou_thresh);
        (tp, fp, fn_)
    }

    fn compute_map(points: &[(f32, usize, usize, usize)]) -> f32 {
        if points.is_empty() {
            return 0.0;
        }
        let mut pr: Vec<(f32, f32)> = points
            .iter()
            .map(|(_th, tp, fp, fn_)| {
                let tp = *tp as f32;
                let fp = *fp as f32;
                let fn_ = *fn_ as f32;
                let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
                let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
                (recall, precision)
            })
            .collect();
        pr.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut ap = 0.0f32;
        let mut prev_r = 0.0f32;
        for (r, p) in pr {
            let delta = (r - prev_r).max(0.0);
            ap += p * delta;
            prev_r = r;
        }
        ap
    }

    fn val_metrics_nms(
        obj_logits: &burn::tensor::Tensor<ADBackend, 4>,
        box_logits: &burn::tensor::Tensor<ADBackend, 4>,
        batch: &BurnBatch<ADBackend>,
        obj_thresh: f32,
        iou_thresh: f32,
    ) -> (f32, usize, usize, usize, usize) {
        let obj = match obj_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return (0.0, 0, 0, 0, 0),
        };
        let boxes = match box_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return (0.0, 0, 0, 0, 0),
        };
        let dims = obj_logits.dims();
        if dims.len() != 4 {
            return (0.0, 0, 0, 0, 0);
        }
        let (b, _c, h, w) = (dims[0], dims[1], dims[2], dims[3]);
        let hw = h * w;

        let gt_boxes = match batch.boxes.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return (0.0, 0, 0, 0, 0),
        };
        let gt_mask = match batch.box_mask.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return (0.0, 0, 0, 0, 0),
        };
        let max_boxes = batch.boxes.dims()[1];

        let mut all_iou = 0.0f32;
        let mut all_matched = 0usize;
        let mut tp = 0usize;
        let mut fp = 0usize;
        let mut fn_total = 0usize;

        for bi in 0..b {
            let mut preds = Vec::new();
            for yi in 0..h {
                for xi in 0..w {
                    let idx = bi * hw + yi * w + xi;
                    let score = 1.0 / (1.0 + (-obj[idx]).exp());
                    if score < obj_thresh {
                        continue;
                    }
                    let base = bi * 4 * hw + yi * w + xi;
                    let mut pb = [
                        1.0 / (1.0 + (-boxes[base]).exp()),
                        1.0 / (1.0 + (-boxes[base + hw]).exp()),
                        1.0 / (1.0 + (-boxes[base + 2 * hw]).exp()),
                        1.0 / (1.0 + (-boxes[base + 3 * hw]).exp()),
                    ];
                    pb[0] = pb[0].clamp(0.0, 1.0);
                    pb[1] = pb[1].clamp(0.0, 1.0);
                    pb[2] = pb[2].clamp(pb[0], 1.0);
                    pb[3] = pb[3].clamp(pb[1], 1.0);
                    preds.push((score, pb));
                }
            }
            let mut gt = Vec::new();
            for gi in 0..max_boxes {
                let m = gt_mask[bi * max_boxes + gi];
                if m <= 0.0 {
                    continue;
                }
                let base = (bi * max_boxes + gi) * 4;
                gt.push([
                    gt_boxes[base].clamp(0.0, 1.0),
                    gt_boxes[base + 1].clamp(0.0, 1.0),
                    gt_boxes[base + 2].clamp(0.0, 1.0),
                    gt_boxes[base + 3].clamp(0.0, 1.0),
                ]);
            }

            if preds.is_empty() {
                if !gt.is_empty() {
                    fn_total += gt.len();
                }
                continue;
            }
            preds.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let mut boxes_only: Vec<[f32; 4]> = preds.iter().map(|p| p.1).collect();
            let scores_only: Vec<f32> = preds.iter().map(|p| p.0).collect();
            let keep = nms(&boxes_only, &scores_only, iou_thresh);
            boxes_only = keep.iter().map(|&i| boxes_only[i]).collect();

            if gt.is_empty() || boxes_only.is_empty() {
                if gt.is_empty() {
                    fp += boxes_only.len();
                } else {
                    fn_total += gt.len();
                }
                continue;
            }

            fn iou_pair(a: &[f32; 4], b: &[f32; 4]) -> f32 {
                let x0 = a[0].max(b[0]);
                let y0 = a[1].max(b[1]);
                let x1 = a[2].min(b[2]);
                let y1 = a[3].min(b[3]);
                let inter = (x1 - x0).max(0.0) * (y1 - y0).max(0.0);
                let area_a = (a[2] - a[0]).max(0.0) * (a[3] - a[1]).max(0.0);
                let area_b = (b[2] - b[0]).max(0.0) * (b[3] - b[1]).max(0.0);
                let union = area_a + area_b - inter + 1e-6;
                inter / union
            }

            let mut matched_gt = vec![false; gt.len()];
            for pb in boxes_only {
                let mut best = -1isize;
                let mut best_iou = 0.0f32;
                for (gidx, gb) in gt.iter().enumerate() {
                    if matched_gt[gidx] {
                        continue;
                    }
                    let i = iou_pair(gb, &pb);
                    if i > best_iou {
                        best_iou = i;
                        best = gidx as isize;
                    }
                }
                if best >= 0 && best_iou >= iou_thresh {
                    matched_gt[best as usize] = true;
                    all_iou += best_iou;
                    all_matched += 1;
                }
            }

            let matched_count = matched_gt.iter().filter(|m| **m).count();
            tp += matched_count;
            fp += preds.len().saturating_sub(matched_count);
            fn_total += gt.len().saturating_sub(matched_count);
        }

        if all_matched == 0 {
            (0.0, all_matched, tp, fp, fn_total)
        } else {
            (all_iou / all_matched as f32, all_matched, tp, fp, fn_total)
        }
    }

    #[allow(dead_code)]
    pub fn main() -> Result<()> {
        main_impl()
    }
}

#[cfg(feature = "burn_runtime")]
fn main() -> anyhow::Result<()> {
    real::main_impl()
}

#[cfg(not(feature = "burn_runtime"))]
fn main() {
    eprintln!("Enable --features burn_runtime to run the evaluator.");
}
