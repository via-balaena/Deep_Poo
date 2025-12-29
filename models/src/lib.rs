use burn::module::Module;
use burn::nn;
use burn::tensor::activation::relu;
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
}

impl Default for BigDetConfig {
    fn default() -> Self {
        Self { hidden: 128, depth: 2 }
    }
}

#[derive(Debug, Module)]
pub struct BigDet<B: burn::tensor::backend::Backend> {
    stem: nn::Linear<B>,
    blocks: Vec<nn::Linear<B>>,
    head: nn::Linear<B>,
}

impl<B: burn::tensor::backend::Backend> BigDet<B> {
    pub fn new(cfg: BigDetConfig, device: &B::Device) -> Self {
        let stem = nn::LinearConfig::new(4, cfg.hidden).init(device);
        let mut blocks = Vec::new();
        for _ in 0..cfg.depth {
            blocks.push(nn::LinearConfig::new(cfg.hidden, cfg.hidden).init(device));
        }
        let head = nn::LinearConfig::new(cfg.hidden, 1).init(device);
        Self { stem, blocks, head }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = relu(self.stem.forward(input));
        for block in &self.blocks {
            x = relu(block.forward(x));
        }
        self.head.forward(x)
    }
}

pub mod prelude {
    pub use super::{BigDet, BigDetConfig, TinyDet, TinyDetConfig};
}
