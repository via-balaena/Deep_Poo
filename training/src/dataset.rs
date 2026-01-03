use burn::tensor::TensorData;
use burn::tensor::{backend::Backend, Tensor};
use data_contracts::capture::CaptureMetadata;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct DatasetConfig {
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

impl DatasetConfig {
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
    let mut features: Vec<f32> = Vec::with_capacity(batch * 6); // mean/std RGB, aspect, box_count

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

        // Push normalized pixel data in CHW order.
        let mut sum = [0f32; 3];
        let mut sumsq = [0f32; 3];
        for c in 0..3 {
            for y in 0..height {
                for x in 0..width {
                    let p = img.get_pixel(x, y);
                    let v = p[c] as f32 / 255.0;
                    image_buf.push(v);
                    sum[c] += v;
                    sumsq[c] += v * v;
                }
            }
        }
        let pix_count = (width * height) as f32;
        let mean = [sum[0] / pix_count, sum[1] / pix_count, sum[2] / pix_count];
        let std = [
            (sumsq[0] / pix_count - mean[0] * mean[0]).max(0.0).sqrt(),
            (sumsq[1] / pix_count - mean[1] * mean[1]).max(0.0).sqrt(),
            (sumsq[2] / pix_count - mean[2] * mean[2]).max(0.0).sqrt(),
        ];

        let mut boxes = Vec::new();
        for label in &sample.metadata.polyp_labels {
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
        features.extend_from_slice(&[
            mean[0],
            mean[1],
            mean[2],
            std[0],
            std[1],
            std[2],
            width as f32 / height as f32,
            box_count,
        ]);
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
