//! Burn-based training and evaluation for CortenForge detection models.
//!
//! This crate provides:
//! - Dataset loading and collation (`collate`, `collate_from_burn_batch`).
//! - Training loop utilities (`run_train`, `TrainArgs`).
//! - Model checkpoint loading/saving helpers.
//!
//! Supports both `LinearClassifier` and `MultiboxModel` from the `models` crate.
//!
//! ## Backend Selection
//! - `backend-wgpu`: Uses WGPU for GPU-accelerated training.
//! - Default: Falls back to NdArray CPU backend.

#![recursion_limit = "256"]

pub mod dataset;
pub mod util;

pub use dataset::{collate, collate_from_burn_batch, CollatedBatch, DatasetPathConfig, RunSample};
pub use models::{LinearClassifier, LinearClassifierConfig, MultiboxModel, MultiboxModelConfig};
pub use util::{run_train, TrainArgs};
/// Backend alias for training/eval (NdArray by default; WGPU if enabled).
#[cfg(feature = "backend-wgpu")]
pub type TrainBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type TrainBackend = burn_ndarray::NdArray<f32>;
