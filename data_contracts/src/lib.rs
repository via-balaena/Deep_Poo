//! Shared data contracts for runs, manifests, and capture metadata.

pub mod capture;
pub mod manifest;

pub use capture::{CaptureMetadata, PolypLabel, ValidationError};
pub use manifest::{RunManifest, RunManifestSchemaVersion};
