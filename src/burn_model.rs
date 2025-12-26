//! Minimal anchor-free detection head for Burn integration.
//!
//! Shapes (normalized coords):
//! - Input images: `[B, 3, H, W]`
//! - Objectness logits: `[B, 1, H, W]`
//! - Box offsets: `[B, 4, H, W]` (x_min, y_min, x_max, y_max in 0..1)

#![allow(unused)] // Gated behind `burn_runtime`.

use anyhow::Result;
use burn::module::{Ignored, Module};
use burn::nn::PaddingConfig2d;
use burn::nn::conv::{Conv2d, Conv2dConfig};
use burn::nn::loss::Reduction;
use burn::record::{BinFileRecorder, FullPrecisionSettings};
use burn::tensor::{Tensor, activation::sigmoid, backend::Backend};
use std::path::Path;

/// Generic bounding box model contract (backend-agnostic).
pub trait BoundingBoxModel<B: Backend> {
    fn forward(&self, images: Tensor<B, 4>) -> ModelOutputs<B>;
    fn compute_loss(
        &self,
        outputs: &ModelOutputs<B>,
        targets: &ModelTargets<B>,
        device: &B::Device,
    ) -> Tensor<B, 1>;
    fn postprocess(&self, outputs: &ModelOutputs<B>, obj_thresh: f32) -> Vec<Vec<Detection>>;
}

#[derive(Debug)]
pub struct ModelOutputs<B: Backend> {
    pub obj_logits: Tensor<B, 4>,
    pub box_logits: Tensor<B, 4>,
}

#[derive(Debug)]
pub struct ModelTargets<B: Backend> {
    pub obj: Tensor<B, 4>,
    pub boxes: Tensor<B, 4>,
    pub box_mask: Tensor<B, 4>,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub score: f32,
    pub bbox: [f32; 4],
}

/// Configuration for the detection head.
#[derive(Debug, Clone, Copy)]
pub struct TinyDetConfig {
    pub max_boxes: usize,
}

impl Default for TinyDetConfig {
    fn default() -> Self {
        Self { max_boxes: 16 }
    }
}

/// Configuration for a larger detector.
#[derive(Debug, Clone, Copy)]
pub struct BigDetConfig {
    pub max_boxes: usize,
}

impl Default for BigDetConfig {
    fn default() -> Self {
        Self { max_boxes: 16 }
    }
}

#[derive(Module, Debug)]
pub struct TinyDet<B: Backend> {
    stem: Conv2d<B>,
    head_obj: Conv2d<B>,
    head_box: Conv2d<B>,
    pub config: Ignored<TinyDetConfig>,
}

impl<B: Backend> TinyDet<B> {
    pub fn new(config: TinyDetConfig, device: &B::Device) -> Self {
        let stem = Conv2dConfig::new([3, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let head_obj = Conv2dConfig::new([32, 1], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);
        let head_box = Conv2dConfig::new([32, 4], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);

        Self {
            stem,
            head_obj,
            head_box,
            config: Ignored(config),
        }
    }

    /// Forward pass returning (objectness_logits, box_offsets).
    pub fn forward(&self, input: Tensor<B, 4>) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let x = burn::tensor::activation::relu(self.stem.forward(input));
        let obj_logits = self.head_obj.forward(x.clone());
        let box_logits = self.head_box.forward(x);
        (obj_logits, box_logits)
    }

    /// Compute detection loss (objectness BCE + Huber boxes, masked).
    pub fn loss(
        &self,
        obj_logits: Tensor<B, 4>,
        box_preds: Tensor<B, 4>,
        target_obj: Tensor<B, 4>,
        target_boxes: Tensor<B, 4>,
        box_mask: Tensor<B, 4>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        // Objectness: BCE with logits.
        let obj_shape = obj_logits.dims();
        let ones_obj = Tensor::<B, 4>::ones(obj_shape.clone(), device);
        let prob = sigmoid(obj_logits.clone())
            .clamp_min(1e-6)
            .clamp_max(1.0 - 1e-6);
        let one_minus = (ones_obj.clone() - prob.clone()).clamp_min(1e-6);
        let obj_loss = (target_obj.clone() * prob.clone().log()
            + (ones_obj.clone() - target_obj.clone()) * one_minus.log())
        .neg()
        .mean();

        // Box regression: Huber masked by presence.
        let diff = (box_preds.clone() - target_boxes.clone()).abs();
        let ones_boxes = Tensor::<B, 4>::ones(diff.dims().clone(), device);
        let delta = ones_boxes.clone();
        let mask_small = diff.clone().lower(delta.clone());
        let diff_sq = diff.clone() * diff.clone();
        let small = diff_sq.mul_scalar(0.5);
        let large = diff.sub_scalar(0.5);
        let mask_float = mask_small.clone().float();
        let per_elem = small * mask_float.clone() + large * (ones_boxes.clone() - mask_float);

        let masked = per_elem * box_mask.clone();
        let eps_scalar = Tensor::<B, 1>::from_floats([1e-6], device);
        let denom = box_mask.clone().sum() + eps_scalar;
        let box_loss = masked.sum() / denom;

        let ciou_loss = self.ciou_loss(box_preds, target_boxes, box_mask, device);

        obj_loss + box_loss + ciou_loss
    }

    fn ciou_loss(
        &self,
        pred: Tensor<B, 4>,
        target: Tensor<B, 4>,
        mask: Tensor<B, 4>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        let p = sigmoid(pred).clamp(0.0, 1.0);
        let t = target.clamp(0.0, 1.0);
        let obj_mask = mask.narrow(1, 0, 1);

        let p = p
            .clone()
            .reshape([p.dims()[0], 4, p.dims()[2] * p.dims()[3]]);
        let t = t
            .clone()
            .reshape([t.dims()[0], 4, t.dims()[2] * t.dims()[3]]);
        let m = obj_mask.clone().reshape([
            obj_mask.dims()[0],
            1,
            obj_mask.dims()[2] * obj_mask.dims()[3],
        ]);

        let px0 = p.clone().narrow(1, 0, 1);
        let py0 = p.clone().narrow(1, 1, 1);
        let px1 = p.clone().narrow(1, 2, 1);
        let py1 = p.clone().narrow(1, 3, 1);

        let tx0 = t.clone().narrow(1, 0, 1);
        let ty0 = t.clone().narrow(1, 1, 1);
        let tx1 = t.clone().narrow(1, 2, 1);
        let ty1 = t.clone().narrow(1, 3, 1);

        let inter_x0 = px0.clone().max_pair(tx0.clone());
        let inter_y0 = py0.clone().max_pair(ty0.clone());
        let inter_x1 = px1.clone().min_pair(tx1.clone());
        let inter_y1 = py1.clone().min_pair(ty1.clone());

        let inter_w = (inter_x1 - inter_x0).clamp_min(0.0);
        let inter_h = (inter_y1 - inter_y0).clamp_min(0.0);
        let inter = inter_w.clone() * inter_h.clone();

        let area_p =
            (px1.clone() - px0.clone()).clamp_min(0.0) * (py1.clone() - py0.clone()).clamp_min(0.0);
        let area_t =
            (tx1.clone() - tx0.clone()).clamp_min(0.0) * (ty1.clone() - ty0.clone()).clamp_min(0.0);
        let union = (area_p + area_t - inter.clone()).clamp_min(1e-6);
        let iou = inter / union;

        let cx_p = (px0.clone() + px1.clone()).mul_scalar(0.5);
        let cy_p = (py0.clone() + py1.clone()).mul_scalar(0.5);
        let cx_t = (tx0.clone() + tx1.clone()).mul_scalar(0.5);
        let cy_t = (ty0.clone() + ty1.clone()).mul_scalar(0.5);

        let rho2 = (cx_p - cx_t).powf_scalar(2.0) + (cy_p - cy_t).powf_scalar(2.0);

        let enc_x0 = px0.min_pair(tx0);
        let enc_y0 = py0.min_pair(ty0);
        let enc_x1 = px1.max_pair(tx1);
        let enc_y1 = py1.max_pair(ty1);
        let c2 = (enc_x1 - enc_x0).clamp_min(1e-6).powf_scalar(2.0)
            + (enc_y1 - enc_y0).clamp_min(1e-6).powf_scalar(2.0);

        let diou = iou - rho2 / c2;
        let ciou_loss = diou.mul_scalar(-1.0).add_scalar(1.0) * m.clone();

        let denom = m.sum().clamp_min(1e-6);
        ciou_loss.sum() / denom
    }
}

impl<B: Backend> BoundingBoxModel<B> for TinyDet<B> {
    fn forward(&self, images: Tensor<B, 4>) -> ModelOutputs<B> {
        let (obj_logits, box_logits) = self.forward(images);
        ModelOutputs {
            obj_logits,
            box_logits,
        }
    }

    fn compute_loss(
        &self,
        outputs: &ModelOutputs<B>,
        targets: &ModelTargets<B>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        self.loss(
            outputs.obj_logits.clone(),
            outputs.box_logits.clone(),
            targets.obj.clone(),
            targets.boxes.clone(),
            targets.box_mask.clone(),
            device,
        )
    }

    fn postprocess(&self, outputs: &ModelOutputs<B>, obj_thresh: f32) -> Vec<Vec<Detection>> {
        let obj = match outputs.obj_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let boxes = match outputs.box_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let dims = outputs.obj_logits.dims();
        if dims.len() != 4 {
            return Vec::new();
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
                    preds.push(Detection { score, bbox: pb });
                }
            }
            preds.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            per_img.push(preds);
        }
        per_img
    }
}

#[derive(Module, Debug)]
pub struct BigDet<B: Backend> {
    stem1: Conv2d<B>,
    stem2: Conv2d<B>,
    head_obj: Conv2d<B>,
    head_box: Conv2d<B>,
    pub config: Ignored<BigDetConfig>,
}

impl<B: Backend> BigDet<B> {
    pub fn new(config: BigDetConfig, device: &B::Device) -> Self {
        let stem1 = Conv2dConfig::new([3, 48], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let stem2 = Conv2dConfig::new([48, 96], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let head_obj = Conv2dConfig::new([96, 1], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);
        let head_box = Conv2dConfig::new([96, 4], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);

        Self {
            stem1,
            stem2,
            head_obj,
            head_box,
            config: Ignored(config),
        }
    }

    pub fn forward(&self, input: Tensor<B, 4>) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let x = burn::tensor::activation::relu(self.stem1.forward(input));
        let x = burn::tensor::activation::relu(self.stem2.forward(x));
        let obj_logits = self.head_obj.forward(x.clone());
        let box_logits = self.head_box.forward(x);
        (obj_logits, box_logits)
    }

    pub fn loss(
        &self,
        obj_logits: Tensor<B, 4>,
        box_preds: Tensor<B, 4>,
        target_obj: Tensor<B, 4>,
        target_boxes: Tensor<B, 4>,
        box_mask: Tensor<B, 4>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        // Same formulation as TinyDet.
        let obj_shape = obj_logits.dims();
        let ones_obj = Tensor::<B, 4>::ones(obj_shape.clone(), device);
        let prob = sigmoid(obj_logits.clone())
            .clamp_min(1e-6)
            .clamp_max(1.0 - 1e-6);
        let one_minus = (ones_obj.clone() - prob.clone()).clamp_min(1e-6);
        let obj_loss = (target_obj.clone() * prob.clone().log()
            + (ones_obj.clone() - target_obj.clone()) * one_minus.log())
        .neg()
        .mean();

        let diff = (box_preds.clone() - target_boxes.clone()).abs();
        let ones_boxes = Tensor::<B, 4>::ones(diff.dims().clone(), device);
        let delta = ones_boxes.clone();
        let mask_small = diff.clone().lower(delta.clone());
        let diff_sq = diff.clone() * diff.clone();
        let small = diff_sq.mul_scalar(0.5);
        let large = diff.sub_scalar(0.5);
        let mask_float = mask_small.clone().float();
        let per_elem = small * mask_float.clone() + large * (ones_boxes.clone() - mask_float);

        let masked = per_elem * box_mask.clone();
        let eps_scalar = Tensor::<B, 1>::from_floats([1e-6], device);
        let denom = box_mask.clone().sum() + eps_scalar;
        let box_loss = masked.sum() / denom;

        let ciou_loss = self.ciou_loss(box_preds, target_boxes, box_mask, device);

        obj_loss + box_loss + ciou_loss
    }

    fn ciou_loss(
        &self,
        pred: Tensor<B, 4>,
        target: Tensor<B, 4>,
        mask: Tensor<B, 4>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        let p = sigmoid(pred).clamp(0.0, 1.0);
        let t = target.clamp(0.0, 1.0);
        let obj_mask = mask.narrow(1, 0, 1);

        let p = p
            .clone()
            .reshape([p.dims()[0], 4, p.dims()[2] * p.dims()[3]]);
        let t = t
            .clone()
            .reshape([t.dims()[0], 4, t.dims()[2] * t.dims()[3]]);
        let m = obj_mask.clone().reshape([
            obj_mask.dims()[0],
            1,
            obj_mask.dims()[2] * obj_mask.dims()[3],
        ]);

        let px0 = p.clone().narrow(1, 0, 1);
        let py0 = p.clone().narrow(1, 1, 1);
        let px1 = p.clone().narrow(1, 2, 1);
        let py1 = p.clone().narrow(1, 3, 1);

        let tx0 = t.clone().narrow(1, 0, 1);
        let ty0 = t.clone().narrow(1, 1, 1);
        let tx1 = t.clone().narrow(1, 2, 1);
        let ty1 = t.clone().narrow(1, 3, 1);

        let inter_x0 = px0.clone().max_pair(tx0.clone());
        let inter_y0 = py0.clone().max_pair(ty0.clone());
        let inter_x1 = px1.clone().min_pair(tx1.clone());
        let inter_y1 = py1.clone().min_pair(ty1.clone());

        let inter_w = (inter_x1 - inter_x0).clamp_min(0.0);
        let inter_h = (inter_y1 - inter_y0).clamp_min(0.0);
        let inter = inter_w.clone() * inter_h.clone();

        let area_p =
            (px1.clone() - px0.clone()).clamp_min(0.0) * (py1.clone() - py0.clone()).clamp_min(0.0);
        let area_t =
            (tx1.clone() - tx0.clone()).clamp_min(0.0) * (ty1.clone() - ty0.clone()).clamp_min(0.0);
        let union = (area_p + area_t - inter.clone()).clamp_min(1e-6);
        let iou = inter / union;

        let cx_p = (px0.clone() + px1.clone()).mul_scalar(0.5);
        let cy_p = (py0.clone() + py1.clone()).mul_scalar(0.5);
        let cx_t = (tx0.clone() + tx1.clone()).mul_scalar(0.5);
        let cy_t = (ty0.clone() + ty1.clone()).mul_scalar(0.5);

        let rho2 = (cx_p - cx_t).powf_scalar(2.0) + (cy_p - cy_t).powf_scalar(2.0);

        let enc_x0 = px0.min_pair(tx0);
        let enc_y0 = py0.min_pair(ty0);
        let enc_x1 = px1.max_pair(tx1);
        let enc_y1 = py1.max_pair(ty1);
        let c2 = (enc_x1 - enc_x0).clamp_min(1e-6).powf_scalar(2.0)
            + (enc_y1 - enc_y0).clamp_min(1e-6).powf_scalar(2.0);

        let diou = iou - rho2 / c2;
        let ciou_loss = diou.mul_scalar(-1.0).add_scalar(1.0) * m.clone();

        let denom = m.sum().clamp_min(1e-6);
        ciou_loss.sum() / denom
    }
}

impl<B: Backend> BoundingBoxModel<B> for BigDet<B> {
    fn forward(&self, images: Tensor<B, 4>) -> ModelOutputs<B> {
        let (obj_logits, box_logits) = self.forward(images);
        ModelOutputs {
            obj_logits,
            box_logits,
        }
    }

    fn compute_loss(
        &self,
        outputs: &ModelOutputs<B>,
        targets: &ModelTargets<B>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        self.loss(
            outputs.obj_logits.clone(),
            outputs.box_logits.clone(),
            targets.obj.clone(),
            targets.boxes.clone(),
            targets.box_mask.clone(),
            device,
        )
    }

    fn postprocess(&self, outputs: &ModelOutputs<B>, obj_thresh: f32) -> Vec<Vec<Detection>> {
        // Same decode as TinyDet.
        let obj = match outputs.obj_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let boxes = match outputs.box_logits.to_data().to_vec::<f32>() {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let dims = outputs.obj_logits.dims();
        if dims.len() != 4 {
            return Vec::new();
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
                    preds.push(Detection { score, bbox: pb });
                }
            }
            preds.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            per_img.push(preds);
        }
        per_img
    }
}

/// Assign ground-truth boxes to a grid (H, W) by nearest center, producing per-cell targets.
pub fn assign_targets_to_grid(
    boxes: &[[f32; 4]],
    grid_h: usize,
    grid_w: usize,
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut obj = vec![0.0f32; grid_h * grid_w];
    let mut tgt = vec![0.0f32; grid_h * grid_w * 4];
    let mut mask = vec![0.0f32; grid_h * grid_w * 4];
    let mut score = vec![0.0f32; grid_h * grid_w];

    for b in boxes {
        let cx = (b[0] + b[2]) * 0.5;
        let cy = (b[1] + b[3]) * 0.5;
        let gx = (cx * grid_w as f32).clamp(0.0, (grid_w - 1) as f32) as usize;
        let gy = (cy * grid_h as f32).clamp(0.0, (grid_h - 1) as f32) as usize;
        let idx = gy * grid_w + gx;
        // Score by IoU with the cell itself to pick the best-fitting box per cell.
        let cell = [
            gx as f32 / grid_w as f32,
            gy as f32 / grid_h as f32,
            (gx + 1) as f32 / grid_w as f32,
            (gy + 1) as f32 / grid_h as f32,
        ];
        let inter_x0 = b[0].max(cell[0]);
        let inter_y0 = b[1].max(cell[1]);
        let inter_x1 = b[2].min(cell[2]);
        let inter_y1 = b[3].min(cell[3]);
        let inter = (inter_x1 - inter_x0).max(0.0) * (inter_y1 - inter_y0).max(0.0);
        let area_b = (b[2] - b[0]).max(0.0) * (b[3] - b[1]).max(0.0);
        let area_c = (cell[2] - cell[0]) * (cell[3] - cell[1]);
        let iou = inter / (area_b + area_c - inter + 1e-6);

        if iou > score[idx] {
            score[idx] = iou;
            obj[idx] = 1.0;
            let base = idx * 4;
            tgt[base] = b[0];
            tgt[base + 1] = b[1];
            tgt[base + 2] = b[2];
            tgt[base + 3] = b[3];
            mask[base] = 1.0;
            mask[base + 1] = 1.0;
            mask[base + 2] = 1.0;
            mask[base + 3] = 1.0;
        }
    }

    (obj, tgt, mask)
}

/// Simple NMS over boxes `[x0,y0,x1,y1]` with scores.
pub fn nms(boxes: &[[f32; 4]], scores: &[f32], iou_thresh: f32) -> Vec<usize> {
    let mut idxs: Vec<usize> = (0..boxes.len()).collect();
    idxs.sort_unstable_by(|&a, &b| {
        scores[b]
            .partial_cmp(&scores[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut keep = Vec::with_capacity(boxes.len());
    let mut suppressed = vec![false; boxes.len()];
    for (pos, &i) in idxs.iter().enumerate() {
        if suppressed[i] {
            continue;
        }
        keep.push(i);
        for &j in idxs.iter().skip(pos + 1) {
            if suppressed[j] {
                continue;
            }
            if iou(&boxes[i], &boxes[j]) > iou_thresh {
                suppressed[j] = true;
            }
        }
    }
    keep
}

fn iou(a: &[f32; 4], b: &[f32; 4]) -> f32 {
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
