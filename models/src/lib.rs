use burn::module::Module;
use burn::nn;
use burn::tensor::activation::{relu, sigmoid};
use burn::tensor::Tensor;

#[derive(Debug, Clone)]
pub struct TinyDetConfig {
    pub hidden: usize,
}

impl Default for TinyDetConfig {
    fn default() -> Self {
        Self { hidden: 64 }
    }
}

#[derive(Debug, Module)]
pub struct TinyDet<B: burn::tensor::backend::Backend> {
    linear1: nn::Linear<B>,
    linear2: nn::Linear<B>,
}

impl<B: burn::tensor::backend::Backend> TinyDet<B> {
    pub fn new(cfg: TinyDetConfig, device: &B::Device) -> Self {
        let linear1 = nn::LinearConfig::new(4, cfg.hidden).init(device);
        let linear2 = nn::LinearConfig::new(cfg.hidden, 1).init(device);
        Self { linear1, linear2 }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = relu(x);
        self.linear2.forward(x)
    }
}

#[derive(Debug, Clone)]
pub struct BigDetConfig {
    pub hidden: usize,
    pub depth: usize,
    pub max_boxes: usize,
    pub input_dim: Option<usize>,
}

impl Default for BigDetConfig {
    fn default() -> Self {
        Self {
            hidden: 128,
            depth: 2,
            max_boxes: 64,
            input_dim: None,
        }
    }
}

#[derive(Debug, Module)]
pub struct BigDet<B: burn::tensor::backend::Backend> {
    stem: nn::Linear<B>,
    blocks: Vec<nn::Linear<B>>,
    box_head: nn::Linear<B>,
    score_head: nn::Linear<B>,
    max_boxes: usize,
    input_dim: usize,
}

impl<B: burn::tensor::backend::Backend> BigDet<B> {
    pub fn new(cfg: BigDetConfig, device: &B::Device) -> Self {
        let input_dim = cfg.input_dim.unwrap_or(4);
        let stem = nn::LinearConfig::new(input_dim, cfg.hidden).init(device);
        let mut blocks = Vec::new();
        for _ in 0..cfg.depth {
            blocks.push(nn::LinearConfig::new(cfg.hidden, cfg.hidden).init(device));
        }
        let box_head = nn::LinearConfig::new(cfg.hidden, cfg.max_boxes.max(1) * 4).init(device);
        let score_head = nn::LinearConfig::new(cfg.hidden, cfg.max_boxes.max(1)).init(device);
        Self {
            stem,
            blocks,
            box_head,
            score_head,
            max_boxes: cfg.max_boxes.max(1),
            input_dim,
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = relu(self.stem.forward(input));
        for block in &self.blocks {
            x = relu(block.forward(x));
        }
        self.score_head.forward(x)
    }

    /// Multibox forward: returns (boxes, scores) with shape [B, max_boxes, 4] and [B, max_boxes].
    /// Boxes/scores are passed through sigmoid to keep them in a stable range.
    pub fn forward_multibox(&self, input: Tensor<B, 2>) -> (Tensor<B, 3>, Tensor<B, 2>) {
        let mut x = relu(self.stem.forward(input));
        for block in &self.blocks {
            x = relu(block.forward(x));
        }
        let boxes_flat = sigmoid(self.box_head.forward(x.clone()));
        let scores = sigmoid(self.score_head.forward(x));
        let batch = boxes_flat.dims()[0];
        let boxes = boxes_flat.reshape([batch, self.max_boxes, 4]);

        // Reorder/clamp to enforce x0 <= x1, y0 <= y1 within [0,1] using arithmetic.
        let x0 = boxes.clone().slice([0..batch, 0..self.max_boxes, 0..1]);
        let y0 = boxes.clone().slice([0..batch, 0..self.max_boxes, 1..2]);
        let x1 = boxes.clone().slice([0..batch, 0..self.max_boxes, 2..3]);
        let y1 = boxes.clone().slice([0..batch, 0..self.max_boxes, 3..4]);

        let dx = x0.clone() - x1.clone();
        let dy = y0.clone() - y1.clone();
        let half = 0.5;

        let x_min = (x0.clone() + x1.clone() - dx.clone().abs()) * half;
        let x_max = (x0 + x1 + dx.abs()) * half;
        let y_min = (y0.clone() + y1.clone() - dy.clone().abs()) * half;
        let y_max = (y0 + y1 + dy.abs()) * half;

        let x_min = x_min.clamp(0.0, 1.0);
        let x_max = x_max.clamp(0.0, 1.0);
        let y_min = y_min.clamp(0.0, 1.0);
        let y_max = y_max.clamp(0.0, 1.0);

        let boxes_ordered = burn::tensor::Tensor::cat(vec![x_min, y_min, x_max, y_max], 2);

        (boxes_ordered, scores)
    }
}

pub mod prelude {
    pub use super::{BigDet, BigDetConfig, TinyDet, TinyDetConfig};
}
