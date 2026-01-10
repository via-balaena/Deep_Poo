pub mod dataset;
pub mod util;

pub use dataset::{collate, DatasetConfig, RunSample};
pub use models::{BigDet, BigDetConfig, TinyDet, TinyDetConfig};
pub use util::{run_train, TrainArgs};
/// Backend alias for training/eval (NdArray by default; WGPU if enabled).
#[cfg(feature = "backend-wgpu")]
pub type TrainBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type TrainBackend = burn_ndarray::NdArray<f32>;
