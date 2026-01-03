use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolypLabel {
    pub center_world: [f32; 3],
    pub bbox_px: Option<[f32; 4]>,
    pub bbox_norm: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMetadata {
    pub frame_id: u64,
    pub sim_time: f64,
    pub unix_time: f64,
    pub image: String,
    pub image_present: bool,
    pub camera_active: bool,
    pub polyp_seed: u64,
    pub polyp_labels: Vec<PolypLabel>,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("bbox_px invalid order or negative: {0:?}")]
    InvalidBboxPx([f32; 4]),
    #[error("bbox_norm out of range: {0:?}")]
    InvalidBboxNorm([f32; 4]),
    #[error("missing image path for present frame")]
    MissingImage,
}

impl PolypLabel {
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
        Ok(())
    }
}

impl CaptureMetadata {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.image_present && self.image.trim().is_empty() {
            return Err(ValidationError::MissingImage);
        }
        for label in &self.polyp_labels {
            label.validate()?;
        }
        Ok(())
    }
}
