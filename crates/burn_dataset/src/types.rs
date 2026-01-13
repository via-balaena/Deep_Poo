//! Core types, error definitions, and data structures for burn_dataset.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub type DatasetResult<T> = Result<T, BurnDatasetError>;

#[derive(Debug, Error)]
pub enum BurnDatasetError {
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("json parse error at {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("label validation failed at {path}: {msg}")]
    Validation { path: PathBuf, msg: String },
    #[error("image missing for label {path}")]
    MissingImage { path: PathBuf },
    #[error("image file missing for label {path}: {image}")]
    MissingImageFile { path: PathBuf, image: PathBuf },
    #[error("image decode error at {path}: {source}")]
    Image {
        path: PathBuf,
        #[source]
        source: image::ImageError,
    },
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct DatasetSample {
    pub frame_id: u64,
    /// Image in CHW layout, normalized to [0, 1].
    pub image_chw: Vec<f32>,
    pub width: u32,
    pub height: u32,
    /// Normalized bounding boxes: [x_min, y_min, x_max, y_max] in 0..1.
    pub boxes: Vec<[f32; 4]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeMode {
    /// Stretch to fill the target dimensions (may distort boxes).
    Force,
    /// Preserve aspect ratio; pad to target with zeros.
    Letterbox,
}

#[derive(Debug, Clone)]
pub struct SampleIndex {
    pub run_dir: PathBuf,
    pub label_path: PathBuf,
}

// Internal types for JSON deserialization
#[derive(Deserialize)]
pub(crate) struct LabelEntry {
    pub(crate) frame_id: u64,
    pub(crate) image: String,
    pub(crate) image_present: bool,
    pub(crate) polyp_labels: Vec<PolypLabel>,
}

#[derive(Deserialize)]
pub(crate) struct PolypLabel {
    pub(crate) bbox_px: Option<[f32; 4]>,
    pub(crate) bbox_norm: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheableTransformConfig {
    pub target_size: Option<(u32, u32)>,
    pub resize_mode: ResizeMode,
    pub max_boxes: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunSummary {
    pub run_dir: PathBuf,
    pub total: usize,
    pub non_empty: usize,
    pub empty: usize,
    pub missing_image: usize,
    pub missing_file: usize,
    pub invalid: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatasetSummary {
    pub runs: Vec<RunSummary>,
    pub totals: RunSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationOutcome {
    Pass,
    Warn,
    Fail,
}

impl ValidationOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidationOutcome::Pass => "pass",
            ValidationOutcome::Warn => "warn",
            ValidationOutcome::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationThresholds {
    pub max_invalid: Option<usize>,
    pub max_missing: Option<usize>,
    pub max_empty: Option<usize>,
    pub max_invalid_ratio: Option<f32>,
    pub max_missing_ratio: Option<f32>,
    pub max_empty_ratio: Option<f32>,
}

impl ValidationThresholds {
    pub fn from_env() -> Self {
        fn parse_usize(key: &str) -> Option<usize> {
            std::env::var(key).ok()?.parse().ok()
        }
        fn parse_ratio(key: &str) -> Option<f32> {
            std::env::var(key).ok()?.parse().ok()
        }
        ValidationThresholds {
            max_invalid: parse_usize("BURN_DATASET_MAX_INVALID"),
            max_missing: parse_usize("BURN_DATASET_MAX_MISSING"),
            max_empty: parse_usize("BURN_DATASET_MAX_EMPTY"),
            max_invalid_ratio: parse_ratio("BURN_DATASET_MAX_INVALID_RATIO"),
            max_missing_ratio: parse_ratio("BURN_DATASET_MAX_MISSING_RATIO"),
            max_empty_ratio: parse_ratio("BURN_DATASET_MAX_EMPTY_RATIO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub outcome: ValidationOutcome,
    pub reasons: Vec<String>,
    pub summary: DatasetSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardMetadata {
    pub id: String,
    /// Path to the shard file, relative to the warehouse root (UTF-8).
    pub relative_path: String,
    /// Shard format version (for binary header layout).
    pub shard_version: u32,
    pub samples: usize,
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    pub max_boxes: usize,
    /// Hex-encoded SHA256 of the shard contents (optional until populated).
    pub checksum_sha256: Option<String>,
    pub dtype: ShardDType,
    pub endianness: Endianness,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShardDType {
    F32,
    F16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarehouseStoreMode {
    /// All shards loaded into memory upfront.
    InMemory,
    /// Memory-mapped shards (low RAM, fast random access on most systems).
    Mmap,
    /// Background-streamed loading with N-shard buffer.
    Streaming { prefetch: usize },
}

impl WarehouseStoreMode {
    pub fn default_streaming() -> Self {
        WarehouseStoreMode::Streaming { prefetch: 2 }
    }
    pub fn is_streaming(&self) -> bool {
        matches!(self, WarehouseStoreMode::Streaming { .. })
    }
    pub fn from_env() -> Self {
        match std::env::var("WAREHOUSE_STORE_MODE").as_deref() {
            Ok("inmemory") => WarehouseStoreMode::InMemory,
            Ok("mmap") => WarehouseStoreMode::Mmap,
            Ok("streaming") => Self::default_streaming(),
            _ => WarehouseStoreMode::Mmap, // default
        }
    }
    pub fn prefetch_from_env() -> usize {
        std::env::var("WAREHOUSE_PREFETCH")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2)
    }
}
