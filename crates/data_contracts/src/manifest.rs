use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RunManifestSchemaVersion {
    V1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunManifest {
    pub schema_version: RunManifestSchemaVersion,
    pub seed: Option<u64>,
    pub output_root: PathBuf,
    pub run_dir: PathBuf,
    pub started_at_unix: f64,
    pub max_frames: Option<u32>,
}

impl RunManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.started_at_unix.is_nan() || self.started_at_unix < 0.0 {
            return Err("started_at_unix must be non-negative".into());
        }
        if let Some(max) = self.max_frames {
            if max == 0 {
                return Err("max_frames cannot be zero".into());
            }
        }
        Ok(())
    }
}
