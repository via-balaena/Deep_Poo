//! Shared data contracts for runs, manifests, and capture metadata.

pub mod capture;
pub mod manifest;
pub mod preprocess;

pub use capture::{CaptureMetadata, DetectionLabel, ValidationError};
pub use manifest::{RunManifest, RunManifestSchemaVersion};
pub use preprocess::{ImageStats, ImageStatsError};
