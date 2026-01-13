use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LabelSource {
    SimAuto,
    Human,
    Model,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionLabel {
    pub center_world: [f32; 3],
    pub bbox_px: Option<[f32; 4]>,
    pub bbox_norm: Option<[f32; 4]>,
    pub source: Option<LabelSource>,
    pub source_confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMetadata {
    pub frame_id: u64,
    pub sim_time: f64,
    pub unix_time: f64,
    pub image: String,
    pub image_present: bool,
    pub camera_active: bool,
    pub label_seed: u64,
    pub labels: Vec<DetectionLabel>,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("bbox_px invalid order or negative: {0:?}")]
    InvalidBboxPx([f32; 4]),
    #[error("bbox_norm out of range: {0:?}")]
    InvalidBboxNorm([f32; 4]),
    #[error("source_confidence out of range: {0:?}")]
    InvalidSourceConfidence(f32),
    #[error("missing image path for present frame")]
    MissingImage,
}

impl DetectionLabel {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(px) = self.bbox_px {
            if px[0].is_nan()
                || px[1].is_nan()
                || px[2].is_nan()
                || px[3].is_nan()
                || px[0] > px[2]
                || px[1] > px[3]
            {
                return Err(ValidationError::InvalidBboxPx(px));
            }
        }
        if let Some(norm) = self.bbox_norm {
            let in_range = norm.iter().all(|v| !v.is_nan() && *v >= 0.0 && *v <= 1.0);
            if !in_range || norm[0] > norm[2] || norm[1] > norm[3] {
                return Err(ValidationError::InvalidBboxNorm(norm));
            }
        }
        if let Some(conf) = self.source_confidence {
            if conf.is_nan() || !(0.0..=1.0).contains(&conf) {
                return Err(ValidationError::InvalidSourceConfidence(conf));
            }
        }
        Ok(())
    }
}

impl CaptureMetadata {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.image_present && self.image.trim().is_empty() {
            return Err(ValidationError::MissingImage);
        }
        for label in &self.labels {
            label.validate()?;
        }
        Ok(())
    }
}
