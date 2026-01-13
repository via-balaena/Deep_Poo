//! Dataset loading, splitting, and Burn-compatible batching utilities for CortenForge.
//!
//! This crate provides utilities for:
//! - Loading capture datasets from filesystem
//! - Train/val splitting with stratification
//! - Image augmentation pipelines
//! - Burn-compatible batch iteration
//! - Warehouse manifest and shard storage

// Module declarations
pub mod aug;
pub mod capture;
pub mod splits;
pub mod types;
pub mod validation;

#[cfg(feature = "burn-runtime")]
pub mod batch;
#[cfg(feature = "burn-runtime")]
pub mod warehouse;

// Re-export public API
pub use aug::{DatasetConfig, TransformPipeline, TransformPipelineBuilder};
pub use capture::{index_runs, load_run_dataset, load_sample_for_etl, summarize_runs};
pub use splits::{count_boxes, split_runs, split_runs_stratified};
pub use types::*;
pub use validation::{summarize_root_with_thresholds, summarize_with_thresholds, validate_summary};

#[cfg(feature = "burn-runtime")]
pub use warehouse::{WarehouseLoaders, WarehouseManifest};

#[cfg(feature = "burn-runtime")]
pub use batch::{build_train_val_iters, BatchIter, BurnBatch};
