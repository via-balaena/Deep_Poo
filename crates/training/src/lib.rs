#![recursion_limit = "256"]

pub mod dataset;
pub mod util;

pub use dataset::{collate, collate_from_burn_batch, CollatedBatch, DatasetConfig, RunSample};
pub use models::{
    ConvolutionalDetector, ConvolutionalDetectorConfig, LinearDetector, LinearDetectorConfig,
};
pub use util::{run_train, TrainArgs};
/// Backend alias for training/eval (NdArray by default; WGPU if enabled).
#[cfg(feature = "backend-wgpu")]
pub type TrainBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type TrainBackend = burn_ndarray::NdArray<f32>;
