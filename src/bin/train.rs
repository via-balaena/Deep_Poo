#![cfg(feature = "burn_runtime")]

use anyhow::Result;
use burn_ndarray::NdArray;
use colon_sim::burn_model::{TinyDet, TinyDetConfig, assign_targets_to_grid};
use colon_sim::tools::burn_dataset::{BatchIter, DatasetConfig, split_runs};
use std::path::Path;

type Backend = NdArray<f32>;

fn main() -> Result<()> {
    let device = <Backend as burn::tensor::backend::Backend>::Device::default();
    let cfg = DatasetConfig {
        target_size: Some((128, 128)),
        flip_horizontal_prob: 0.5,
        max_boxes: 8,
        ..Default::default()
    };

    let root = Path::new("assets/datasets/captures");
    let indices =
        colon_sim::tools::burn_dataset::index_runs(root).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let (train_idx, _) = split_runs(indices, 0.2);
    let mut train =
        BatchIter::from_indices(train_idx, cfg).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let model = TinyDet::<Backend>::new(TinyDetConfig::default(), &device);

    if let Some(batch) = train
        .next_batch::<Backend>(1, &device)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
    {
        let (obj_logits, box_logits) = model.forward(batch.images.clone());
        let (t_obj, t_boxes, t_mask) =
            build_targets(&batch, obj_logits.dims()[2], obj_logits.dims()[3], &device)?;
        let loss = model.loss(
            obj_logits,
            box_logits.clone(),
            t_obj.clone(),
            t_boxes.clone(),
            t_mask,
            &device,
        );
        let loss_val = loss.to_data().to_vec::<f32>().unwrap_or_default();
        let mean_iou = mean_iou_host(&box_logits, &t_boxes, &t_obj);
        println!(
            "dummy training step, loss={:?}, mean_iou={:?}",
            loss_val.first(),
            mean_iou
        );
    } else {
        println!("No training batches found under {:?}", root);
    }

    Ok(())
}

fn build_targets(
    batch: &colon_sim::tools::burn_dataset::BurnBatch<Backend>,
    grid_h: usize,
    grid_w: usize,
    device: &<Backend as burn::tensor::backend::Backend>::Device,
) -> Result<(
    burn::tensor::Tensor<Backend, 4>,
    burn::tensor::Tensor<Backend, 4>,
    burn::tensor::Tensor<Backend, 4>,
)> {
    // Supports batch_size = 1 for now.
    let boxes_vec = batch
        .boxes
        .to_data()
        .to_vec::<f32>()
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let box_mask_vec = batch
        .box_mask
        .to_data()
        .to_vec::<f32>()
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let max_boxes = batch.boxes.dims()[1];
    let mut valid_boxes = Vec::new();
    for i in 0..max_boxes {
        let base = i * 4;
        if box_mask_vec[i] > 0.0 {
            valid_boxes.push([
                boxes_vec[base],
                boxes_vec[base + 1],
                boxes_vec[base + 2],
                boxes_vec[base + 3],
            ]);
        }
    }

    let (obj, tgt, mask) = assign_targets_to_grid(&valid_boxes, grid_h, grid_w);
    let obj_t = burn::tensor::Tensor::<Backend, 4>::from_floats(obj.as_slice(), device)
        .reshape([1, 1, grid_h, grid_w]);
    let boxes_t = burn::tensor::Tensor::<Backend, 4>::from_floats(tgt.as_slice(), device)
        .reshape([1, 4, grid_h, grid_w]);
    let mask_t = burn::tensor::Tensor::<Backend, 4>::from_floats(mask.as_slice(), device)
        .reshape([1, 4, grid_h, grid_w]);
    Ok((obj_t, boxes_t, mask_t))
}

fn mean_iou_host(
    pred_boxes: &burn::tensor::Tensor<Backend, 4>,
    target_boxes: &burn::tensor::Tensor<Backend, 4>,
    target_obj: &burn::tensor::Tensor<Backend, 4>,
) -> f32 {
    fn fast_sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    let pb = match pred_boxes.to_data().to_vec::<f32>() {
        Ok(v) => v,
        Err(_) => return 0.0,
    };
    let tb = match target_boxes.to_data().to_vec::<f32>() {
        Ok(v) => v,
        Err(_) => return 0.0,
    };
    let tobj = match target_obj.to_data().to_vec::<f32>() {
        Ok(v) => v,
        Err(_) => return 0.0,
    };
    let dims = pred_boxes.dims();
    if dims.len() != 4 {
        return 0.0;
    }
    let (b, _c, h, w) = (dims[0], dims[1], dims[2], dims[3]);
    let hw = h * w;
    let mut sum = 0.0f32;
    let mut count = 0.0f32;
    for bi in 0..b {
        for yi in 0..h {
            for xi in 0..w {
                let idx = bi * hw + yi * w + xi;
                if tobj[idx] <= 0.5 {
                    continue;
                }
                let base = bi * 4 * hw + yi * w + xi;
                let mut pb_vals = [
                    fast_sigmoid(pb[base]),
                    fast_sigmoid(pb[base + hw]),
                    fast_sigmoid(pb[base + 2 * hw]),
                    fast_sigmoid(pb[base + 3 * hw]),
                ];
                pb_vals[0] = pb_vals[0].clamp(0.0, 1.0);
                pb_vals[1] = pb_vals[1].clamp(0.0, 1.0);
                pb_vals[2] = pb_vals[2].clamp(pb_vals[0], 1.0);
                pb_vals[3] = pb_vals[3].clamp(pb_vals[1], 1.0);

                let tb_vals = [
                    tb[base].clamp(0.0, 1.0),
                    tb[base + hw].clamp(0.0, 1.0),
                    tb[base + 2 * hw].clamp(0.0, 1.0),
                    tb[base + 3 * hw].clamp(0.0, 1.0),
                ];

                let inter_x0 = pb_vals[0].max(tb_vals[0]);
                let inter_y0 = pb_vals[1].max(tb_vals[1]);
                let inter_x1 = pb_vals[2].min(tb_vals[2]);
                let inter_y1 = pb_vals[3].min(tb_vals[3]);
                let inter_w = (inter_x1 - inter_x0).max(0.0);
                let inter_h = (inter_y1 - inter_y0).max(0.0);
                let inter = inter_w * inter_h;

                let area_p =
                    (pb_vals[2] - pb_vals[0]).max(0.0) * (pb_vals[3] - pb_vals[1]).max(0.0);
                let area_t =
                    (tb_vals[2] - tb_vals[0]).max(0.0) * (tb_vals[3] - tb_vals[1]).max(0.0);
                let union = (area_p + area_t - inter).max(1e-6);
                let iou = inter / union;
                sum += iou;
                count += 1.0;
            }
        }
    }
    if count == 0.0 { 0.0 } else { sum / count }
}
