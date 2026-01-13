use burn::tensor::TensorData;
use burn::tensor::{backend::Backend, Tensor};
use burn_dataset::BurnBatch;
use data_contracts::capture::CaptureMetadata;
use data_contracts::preprocess::{stats_from_chw_f32, stats_from_rgb_u8};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct DatasetPathConfig {
    pub root: PathBuf,
    pub labels_subdir: String,
    pub images_subdir: String,
}

#[derive(Debug, Clone)]
pub struct RunSample {
    pub image: PathBuf,
    pub metadata: CaptureMetadata,
}

#[derive(Debug, Clone)]
pub struct CollatedBatch<B: Backend> {
    pub images: Tensor<B, 4>,
    /// Normalized boxes per sample (shape: [batch, max_boxes, 4]).
    pub boxes: Tensor<B, 3>,
    /// Mask indicating which box slots are populated (shape: [batch, max_boxes]).
    pub box_mask: Tensor<B, 2>,
    /// Global/image features per sample (mean/std RGB, aspect ratio, box count) shape [batch, F].
    pub features: Tensor<B, 2>,
}

impl DatasetPathConfig {
    pub fn load(&self) -> anyhow::Result<Vec<RunSample>> {
        let mut samples = Vec::new();
        let labels_dir = self.root.join(&self.labels_subdir);
        for entry in fs::read_dir(&labels_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let meta: CaptureMetadata = serde_json::from_slice(&fs::read(&path)?)?;
            meta.validate()
                .map_err(|e| anyhow::anyhow!("invalid metadata {:?}: {e}", path))?;
            let img_path = self.root.join(&self.images_subdir).join(&meta.image);
            samples.push(RunSample {
                image: img_path,
                metadata: meta,
            });
        }
        Ok(samples)
    }
}

pub fn collate<B: Backend>(
    samples: &[RunSample],
    max_boxes: usize,
) -> anyhow::Result<CollatedBatch<B>> {
    if samples.is_empty() {
        anyhow::bail!("cannot collate empty batch");
    }
    let max_boxes = max_boxes.max(1);

    // Load first image to establish dimensions.
    let first = image::open(&samples[0].image)
        .map_err(|e| anyhow::anyhow!("failed to open image {:?}: {e}", samples[0].image))?
        .to_rgb8();
    let (width, height) = first.dimensions();

    let batch = samples.len();
    let num_pixels = (width * height) as usize;
    let mut image_buf: Vec<f32> = Vec::with_capacity(batch * num_pixels * 3);
    let mut features: Vec<f32> = Vec::with_capacity(batch * 8); // mean/std RGB, aspect, box_count

    // Gather normalized boxes, truncated to max_boxes.
    let mut all_boxes: Vec<Vec<[f32; 4]>> = Vec::with_capacity(batch);

    for (idx, sample) in samples.iter().enumerate() {
        let img = if idx == 0 {
            first.clone()
        } else {
            let img = image::open(&sample.image)
                .map_err(|e| anyhow::anyhow!("failed to open image {:?}: {e}", sample.image))?;
            let rgb = img.to_rgb8();
            let (w, h) = rgb.dimensions();
            if w != width || h != height {
                anyhow::bail!(
                    "image dimensions differ within batch: {:?} is {}x{}, expected {}x{}",
                    sample.image,
                    w,
                    h,
                    width,
                    height
                );
            }
            rgb
        };

        let stats = stats_from_rgb_u8(width, height, img.as_raw())
            .map_err(|e| anyhow::anyhow!("failed to compute image stats: {e}"))?;

        // Push normalized pixel data in CHW order.
        for c in 0..3 {
            for y in 0..height {
                for x in 0..width {
                    let p = img.get_pixel(x, y);
                    let v = p[c] as f32 / 255.0;
                    image_buf.push(v);
                }
            }
        }

        let mut boxes = Vec::new();
        for label in &sample.metadata.labels {
            let bbox = if let Some(norm) = label.bbox_norm {
                norm
            } else if let Some(px) = label.bbox_px {
                [
                    px[0] / width as f32,
                    px[1] / height as f32,
                    px[2] / width as f32,
                    px[3] / height as f32,
                ]
            } else {
                continue;
            };
            boxes.push(bbox);
            if boxes.len() >= max_boxes {
                break;
            }
        }
        let box_count = boxes.len() as f32;
        features.extend_from_slice(&stats.feature_vector(box_count));
        all_boxes.push(boxes);
    }

    let mut boxes_buf = vec![0.0f32; batch * max_boxes * 4];
    let mut mask_buf = vec![0.0f32; batch * max_boxes];
    for (b, boxes) in all_boxes.iter().enumerate() {
        for (i, bbox) in boxes.iter().enumerate() {
            let base = (b * max_boxes + i) * 4;
            boxes_buf[base..base + 4].copy_from_slice(bbox);
            mask_buf[b * max_boxes + i] = 1.0;
        }
    }

    let device = &B::Device::default();
    let images = Tensor::<B, 4>::from_data(
        TensorData::new(image_buf, [batch, 3, height as usize, width as usize]),
        device,
    );
    let boxes =
        Tensor::<B, 3>::from_data(TensorData::new(boxes_buf, [batch, max_boxes, 4]), device);
    let box_mask = Tensor::<B, 2>::from_data(TensorData::new(mask_buf, [batch, max_boxes]), device);

    let features = Tensor::<B, 2>::from_data(TensorData::new(features, [batch, 8]), device);

    Ok(CollatedBatch {
        images,
        boxes,
        box_mask,
        features,
    })
}

pub fn collate_from_burn_batch<B: Backend>(
    batch: BurnBatch<B>,
    max_boxes: usize,
) -> anyhow::Result<CollatedBatch<B>> {
    let dims = batch.images.dims();
    let batch_size = dims[0];
    let channels = dims[1];
    let height = dims[2];
    let width = dims[3];
    let max_boxes = max_boxes.max(1);

    if channels != 3 {
        anyhow::bail!("expected 3-channel images, got {channels}");
    }
    let box_dims = batch.boxes.dims();
    if box_dims[1] != max_boxes {
        anyhow::bail!(
            "warehouse batch max_boxes {actual} does not match requested {expected}",
            actual = box_dims[1],
            expected = max_boxes
        );
    }

    let image_data = batch
        .images
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default();
    let mask_data = batch
        .box_mask
        .clone()
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_default();
    let pixels_per_channel = height * width;

    if image_data.len() != batch_size * channels * pixels_per_channel {
        anyhow::bail!("unexpected image buffer size in warehouse batch");
    }
    if mask_data.len() != batch_size * max_boxes {
        anyhow::bail!("unexpected box mask size in warehouse batch");
    }

    let mut features: Vec<f32> = Vec::with_capacity(batch_size * 8);
    for b in 0..batch_size {
        let start = b * channels * pixels_per_channel;
        let slice = &image_data[start..start + channels * pixels_per_channel];
        let stats = stats_from_chw_f32(width, height, slice)
            .map_err(|e| anyhow::anyhow!("failed to compute image stats: {e}"))?;
        let mask_start = b * max_boxes;
        let box_count = mask_data[mask_start..mask_start + max_boxes]
            .iter()
            .filter(|v| **v > 0.0)
            .count() as f32;
        features.extend_from_slice(&stats.feature_vector(box_count));
    }

    let device = batch.images.device();
    let features = Tensor::<B, 2>::from_data(TensorData::new(features, [batch_size, 8]), &device);

    Ok(CollatedBatch {
        images: batch.images,
        boxes: batch.boxes,
        box_mask: batch.box_mask,
        features,
    })
}
