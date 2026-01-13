use crate::{InferenceBackend, InferenceModel, InferenceModelConfig};
use burn::module::Module;
use burn::tensor::TensorData;
#[cfg(feature = "convolutional_detector")]
use data_contracts::preprocess::{stats_from_rgba_u8, ImageStats};
use std::path::Path;
use std::sync::{Arc, Mutex};
use vision_core::interfaces::{DetectionResult, Detector, Frame};

/// Thresholds for inference (objectness + IoU).
///
/// This is a framework-agnostic type. For Bevy applications, use
/// `InferenceThresholdsResource` from the `vision_runtime` crate.
#[derive(Debug, Clone, Copy)]
pub struct InferenceThresholds {
    pub objectness_threshold: f32,
    pub iou_threshold: f32,
}

impl Default for InferenceThresholds {
    fn default() -> Self {
        Self {
            objectness_threshold: 0.3,
            iou_threshold: 0.5,
        }
    }
}

/// Heuristic detector placeholder; used when Burn weights are unavailable.
struct HeuristicDetector {
    obj_thresh: f32,
}

impl Detector for HeuristicDetector {
    fn detect(&mut self, frame: &Frame) -> DetectionResult {
        let confidence = self.obj_thresh;
        DetectionResult {
            frame_id: frame.id,
            positive: confidence >= self.obj_thresh,
            confidence,
            boxes: Vec::new(),
            scores: Vec::new(),
        }
    }
}

struct BurnDetector {
    model: Arc<Mutex<InferenceModel<InferenceBackend>>>,
    obj_thresh: f32,
    #[allow(dead_code)]
    iou_thresh: f32,
}

impl BurnDetector {
    fn frame_to_tensor(&self, _frame: &Frame) -> TensorData {
        #[cfg(feature = "convolutional_detector")]
        {
            let (w, h) = _frame.size;
            let stats = if let Some(rgba) = &_frame.rgba {
                stats_from_rgba_u8(w, h, rgba).unwrap_or_else(|_| ImageStats {
                    mean: [0.0; 3],
                    std: [0.0; 3],
                    aspect: w as f32 / h as f32,
                })
            } else {
                ImageStats {
                    mean: [0.0; 3],
                    std: [0.0; 3],
                    aspect: w as f32 / h as f32,
                }
            };

            let mut input = Vec::with_capacity(12);
            input.extend_from_slice(&[0.0, 0.0, 1.0, 1.0]);
            input.extend_from_slice(&stats.feature_vector(0.0));
            TensorData::new(input, [1, 12])
        }
        #[cfg(not(feature = "convolutional_detector"))]
        {
            TensorData::new(vec![0.0, 0.0, 1.0, 1.0], [1, 4])
        }
    }
}

impl Detector for BurnDetector {
    fn detect(&mut self, frame: &Frame) -> DetectionResult {
        let input = self.frame_to_tensor(frame);
        let device = <InferenceBackend as burn::tensor::backend::Backend>::Device::default();
        let model = self.model.lock().expect("model mutex poisoned");
        let logits = model.forward(burn::tensor::Tensor::<InferenceBackend, 2>::from_data(
            input, &device,
        ));
        let scores = logits.into_data().to_vec::<f32>().unwrap_or_default();
        let confidence = scores.first().copied().unwrap_or(0.0);
        DetectionResult {
            frame_id: frame.id,
            positive: confidence >= self.obj_thresh,
            confidence,
            boxes: Vec::new(),
            scores,
        }
    }
}

/// Factory that will load Burn checkpoints when available; currently returns heuristic.
pub struct InferenceFactory;

impl InferenceFactory {
    pub fn build(
        &self,
        thresh: InferenceThresholds,
        weights: Option<&Path>,
    ) -> Box<dyn vision_core::interfaces::Detector + Send + Sync> {
        if let Some(det) = self.try_load_burn_detector(thresh, weights) {
            return det;
        }
        eprintln!("InferenceFactory: no valid checkpoint provided; using heuristic detector.");
        Box::new(HeuristicDetector {
            obj_thresh: thresh.objectness_threshold,
        })
    }

    fn try_load_burn_detector(
        &self,
        thresh: InferenceThresholds,
        weights: Option<&Path>,
    ) -> Option<Box<dyn vision_core::interfaces::Detector + Send + Sync>> {
        let path = weights?;
        if !path.exists() {
            return None;
        }
        let device = <InferenceBackend as burn::tensor::backend::Backend>::Device::default();
        let recorder = burn::record::BinFileRecorder::<burn::record::FullPrecisionSettings>::new();
        #[cfg(feature = "convolutional_detector")]
        let config = InferenceModelConfig {
            input_dim: Some(4 + 8),
            ..Default::default()
        };
        #[cfg(not(feature = "convolutional_detector"))]
        let config = InferenceModelConfig::default();

        match InferenceModel::<InferenceBackend>::new(config, &device)
            .load_file(path, &recorder, &device)
        {
            Ok(model) => Some(Box::new(BurnDetector {
                model: Arc::new(Mutex::new(model)),
                obj_thresh: thresh.objectness_threshold,
                iou_thresh: thresh.iou_threshold,
            })),
            Err(err) => {
                eprintln!(
                    "Failed to load detector checkpoint {:?}: {err}. Falling back to heuristic.",
                    path
                );
                None
            }
        }
    }
}
