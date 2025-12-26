#![cfg(feature = "burn_runtime")]

use burn::tensor::{Tensor, backend::Backend};

/// Decode grid logits into per-image detections (score, [x0,y0,x1,y1] normalized).
pub fn decode_grid_preds<B: Backend>(
    obj_logits: &Tensor<B, 4>,
    box_logits: &Tensor<B, 4>,
    obj_thresh: f32,
) -> Option<Vec<Vec<(f32, [f32; 4])>>> {
    let obj = obj_logits.to_data().to_vec::<f32>().ok()?;
    let boxes = box_logits.to_data().to_vec::<f32>().ok()?;
    let dims = obj_logits.dims();
    if dims.len() != 4 {
        return None;
    }
    let (b, _c, h, w) = (dims[0], dims[1], dims[2], dims[3]);
    let hw = h * w;

    let mut per_img = Vec::with_capacity(b);
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
        preds.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        per_img.push(preds);
    }
    Some(per_img)
}

/// Collect GT boxes per image from batch tensors.
pub fn collect_gt_boxes<B: Backend>(
    gt_boxes: &Tensor<B, 3>,
    gt_mask: &Tensor<B, 2>,
) -> Option<Vec<Vec<[f32; 4]>>> {
    let boxes = gt_boxes.to_data().to_vec::<f32>().ok()?;
    let mask = gt_mask.to_data().to_vec::<f32>().ok()?;
    let dims = gt_boxes.dims();
    if dims.len() != 3 {
        return None;
    }
    let (b, max_boxes, _) = (dims[0], dims[1], dims[2]);
    let mut per_img = Vec::with_capacity(b);
    for bi in 0..b {
        let mut gt = Vec::new();
        for gi in 0..max_boxes {
            let m = mask[bi * max_boxes + gi];
            if m <= 0.0 {
                continue;
            }
            let base = (bi * max_boxes + gi) * 4;
            gt.push([
                boxes[base].clamp(0.0, 1.0),
                boxes[base + 1].clamp(0.0, 1.0),
                boxes[base + 2].clamp(0.0, 1.0),
                boxes[base + 3].clamp(0.0, 1.0),
            ]);
        }
        per_img.push(gt);
    }
    Some(per_img)
}
