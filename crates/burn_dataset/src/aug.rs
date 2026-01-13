//! Image augmentation and transformation pipeline.

use crate::types::{
    CacheableTransformConfig, DatasetResult, DatasetSample, DetectionLabel, LabelEntry, ResizeMode,
};
use image::imageops::FilterType;
use rand::{Rng, SeedableRng};
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct DatasetConfig {
    /// Resize all images to this (width, height). If None, images must already share shape.
    pub target_size: Option<(u32, u32)>,
    /// How to resize images when target_size is set.
    pub resize_mode: ResizeMode,
    /// Probability of applying a horizontal flip augmentation.
    pub flip_horizontal_prob: f32,
    /// Probability of applying a light color jitter (brightness/contrast).
    pub color_jitter_prob: f32,
    /// Max jitter scale for brightness/contrast.
    pub color_jitter_strength: f32,
    /// Probability of applying a scale jitter (zoom in/out with bbox-safe padding/cropping).
    pub scale_jitter_prob: f32,
    /// Min scale factor for scale jitter.
    pub scale_jitter_min: f32,
    /// Max scale factor for scale jitter.
    pub scale_jitter_max: f32,
    /// Probability of adding uniform noise per channel.
    pub noise_prob: f32,
    /// Max absolute noise added (0-1 range).
    pub noise_strength: f32,
    /// Probability of applying a blur.
    pub blur_prob: f32,
    /// Blur sigma (passed to image::imageops::blur).
    pub blur_sigma: f32,
    /// Cap on boxes per image; extras are dropped, padding uses zeros with mask.
    pub max_boxes: usize,
    /// Shuffle samples before iteration.
    pub shuffle: bool,
    /// Seed for reproducible shuffling.
    pub seed: Option<u64>,
    /// Skip frames with no bounding boxes.
    pub skip_empty_labels: bool,
    /// Drop the last partial batch (training stability for small batches).
    pub drop_last: bool,
    /// Optional transform pipeline override; if None, built from other fields.
    pub transform: Option<TransformPipeline>,
}

impl Default for DatasetConfig {
    fn default() -> Self {
        Self {
            target_size: Some((512, 512)),
            resize_mode: ResizeMode::Letterbox,
            flip_horizontal_prob: 0.0,
            color_jitter_prob: 0.0,
            color_jitter_strength: 0.1,
            scale_jitter_prob: 0.0,
            scale_jitter_min: 0.8,
            scale_jitter_max: 1.2,
            noise_prob: 0.0,
            noise_strength: 0.02,
            blur_prob: 0.0,
            blur_sigma: 1.0,
            max_boxes: 16,
            shuffle: true,
            seed: None,
            skip_empty_labels: true,
            drop_last: false,
            transform: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransformPipeline {
    pub cacheable: CacheableTransformConfig,
    pub target_size: Option<(u32, u32)>,
    pub resize_mode: ResizeMode,
    pub flip_horizontal_prob: f32,
    pub color_jitter_prob: f32,
    pub color_jitter_strength: f32,
    pub scale_jitter_prob: f32,
    pub scale_jitter_min: f32,
    pub scale_jitter_max: f32,
    pub noise_prob: f32,
    pub noise_strength: f32,
    pub blur_prob: f32,
    pub blur_sigma: f32,
    pub max_boxes: usize,
    pub seed: Option<u64>,
}

impl TransformPipeline {
    pub fn from_config(cfg: &DatasetConfig) -> Self {
        Self {
            cacheable: CacheableTransformConfig {
                target_size: cfg.target_size,
                resize_mode: cfg.resize_mode,
                max_boxes: cfg.max_boxes,
            },
            target_size: cfg.target_size,
            resize_mode: cfg.resize_mode,
            flip_horizontal_prob: cfg.flip_horizontal_prob,
            color_jitter_prob: cfg.color_jitter_prob,
            color_jitter_strength: cfg.color_jitter_strength,
            scale_jitter_prob: cfg.scale_jitter_prob,
            scale_jitter_min: cfg.scale_jitter_min,
            scale_jitter_max: cfg.scale_jitter_max,
            noise_prob: cfg.noise_prob,
            noise_strength: cfg.noise_strength,
            blur_prob: cfg.blur_prob,
            blur_sigma: cfg.blur_sigma,
            max_boxes: cfg.max_boxes,
            seed: cfg.seed,
        }
    }

    pub fn describe(&self) -> String {
        let ts = self
            .target_size
            .map(|(w, h)| format!("{}x{}", w, h))
            .unwrap_or_else(|| "none".to_string());
        format!(
            "target_size={} resize={:?} flip_p={:.2} color_jitter_p={:.2} strength={:.2} scale_jitter_p={:.2} range=[{:.2},{:.2}] noise_p={:.2} strength={:.3} blur_p={:.2} sigma={:.2} max_boxes={} seed={}",
            ts,
            self.resize_mode,
            self.flip_horizontal_prob,
            self.color_jitter_prob,
            self.color_jitter_strength,
            self.scale_jitter_prob,
            self.scale_jitter_min,
            self.scale_jitter_max,
            self.noise_prob,
            self.noise_strength,
            self.blur_prob,
            self.blur_sigma,
            self.max_boxes,
            self.seed
                .map(|s| s.to_string())
                .unwrap_or_else(|| "none".to_string())
        )
    }

    pub(crate) fn apply(
        &self,
        img: image::RgbImage,
        meta: &LabelEntry,
    ) -> DatasetResult<DatasetSample> {
        let (mut width, mut height) = img.dimensions();
        // Choose RNG: seeded if provided (per-frame deterministic), else thread-local.
        let mut rng_local;
        let mut seeded_rng;
        let rng: &mut dyn rand::RngCore = if let Some(seed) = self.seed {
            let mixed = seed ^ meta.frame_id;
            seeded_rng = rand::rngs::StdRng::seed_from_u64(mixed);
            &mut seeded_rng
        } else {
            rng_local = rand::rng();
            &mut rng_local
        };

        if let Some((w, h)) = self.target_size {
            match self.resize_mode {
                ResizeMode::Force => {
                    width = w;
                    height = h;
                    let mut resized = image::imageops::resize(&img, w, h, FilterType::Triangle);
                    let mut boxes = normalize_boxes(&meta.labels, w, h);
                    maybe_hflip(&mut resized, &mut boxes, self.flip_horizontal_prob, rng);
                    return build_sample_from_image(
                        resized,
                        width,
                        height,
                        boxes,
                        meta.frame_id,
                        self.max_boxes,
                    );
                }
                ResizeMode::Letterbox => {
                    let (mut resized_img, pad_w, pad_h) = letterbox_resize(&img, w, h)?;
                    let scale_x = resized_img.width() as f32 / img.width() as f32;
                    let scale_y = resized_img.height() as f32 / img.height() as f32;

                    let (norm_boxes, px_boxes) =
                        normalize_boxes_with_px(&meta.labels, img.width(), img.height());

                    let mut boxes = norm_boxes
                        .into_iter()
                        .zip(px_boxes)
                        .map(|(_norm, px)| {
                            let scaled = [
                                px[0] * scale_x + pad_w as f32,
                                px[1] * scale_y + pad_h as f32,
                                px[2] * scale_x + pad_w as f32,
                                px[3] * scale_y + pad_h as f32,
                            ];
                            [
                                scaled[0] / w as f32,
                                scaled[1] / h as f32,
                                scaled[2] / w as f32,
                                scaled[3] / h as f32,
                            ]
                        })
                        .map(|mut b| {
                            for v in b.iter_mut() {
                                *v = v.clamp(0.0, 1.0);
                            }
                            b
                        })
                        .collect::<Vec<_>>();

                    maybe_hflip(&mut resized_img, &mut boxes, self.flip_horizontal_prob, rng);
                    maybe_jitter(
                        &mut resized_img,
                        self.color_jitter_prob,
                        self.color_jitter_strength,
                        rng,
                    );
                    maybe_scale_jitter(
                        &mut resized_img,
                        &mut boxes,
                        self.scale_jitter_prob,
                        self.scale_jitter_min,
                        self.scale_jitter_max,
                        rng,
                    );
                    maybe_noise(&mut resized_img, self.noise_prob, self.noise_strength, rng);
                    maybe_blur(&mut resized_img, self.blur_prob, self.blur_sigma, rng);

                    if boxes.len() > self.max_boxes {
                        boxes.truncate(self.max_boxes);
                    }

                    return build_sample_from_image(
                        resized_img,
                        w,
                        h,
                        boxes,
                        meta.frame_id,
                        self.max_boxes,
                    );
                }
            }
        }

        let mut boxes = normalize_boxes(&meta.labels, width, height);
        let mut img = img;
        maybe_hflip(&mut img, &mut boxes, self.flip_horizontal_prob, rng);
        maybe_jitter(
            &mut img,
            self.color_jitter_prob,
            self.color_jitter_strength,
            rng,
        );
        maybe_scale_jitter(
            &mut img,
            &mut boxes,
            self.scale_jitter_prob,
            self.scale_jitter_min,
            self.scale_jitter_max,
            rng,
        );
        maybe_noise(&mut img, self.noise_prob, self.noise_strength, rng);
        maybe_blur(&mut img, self.blur_prob, self.blur_sigma, rng);
        let sample =
            build_sample_from_image(img, width, height, boxes, meta.frame_id, self.max_boxes)?;
        Ok(sample)
    }
}

#[derive(Debug, Clone)]
pub struct TransformPipelineBuilder {
    inner: TransformPipeline,
}

impl Default for TransformPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformPipelineBuilder {
    pub fn new() -> Self {
        Self {
            inner: TransformPipeline::from_config(&DatasetConfig::default()),
        }
    }
    pub fn target_size(mut self, size: Option<(u32, u32)>) -> Self {
        self.inner.target_size = size;
        self.inner.cacheable.target_size = size;
        self
    }
    pub fn resize_mode(mut self, mode: ResizeMode) -> Self {
        self.inner.resize_mode = mode;
        self.inner.cacheable.resize_mode = mode;
        self
    }
    pub fn flip_horizontal_prob(mut self, p: f32) -> Self {
        self.inner.flip_horizontal_prob = p;
        self
    }
    pub fn color_jitter(mut self, prob: f32, strength: f32) -> Self {
        self.inner.color_jitter_prob = prob;
        self.inner.color_jitter_strength = strength;
        self
    }
    pub fn scale_jitter(mut self, prob: f32, min: f32, max: f32) -> Self {
        self.inner.scale_jitter_prob = prob;
        self.inner.scale_jitter_min = min;
        self.inner.scale_jitter_max = max;
        self
    }
    pub fn noise(mut self, prob: f32, strength: f32) -> Self {
        self.inner.noise_prob = prob;
        self.inner.noise_strength = strength;
        self
    }
    pub fn blur(mut self, prob: f32, sigma: f32) -> Self {
        self.inner.blur_prob = prob;
        self.inner.blur_sigma = sigma;
        self
    }
    pub fn max_boxes(mut self, max_boxes: usize) -> Self {
        self.inner.max_boxes = max_boxes;
        self.inner.cacheable.max_boxes = max_boxes;
        self
    }
    pub fn seed(mut self, seed: Option<u64>) -> Self {
        self.inner.seed = seed;
        self
    }
    pub fn build(self) -> TransformPipeline {
        self.inner
    }
}

fn build_sample_from_image(
    img: image::RgbImage,
    width: u32,
    height: u32,
    mut boxes: Vec<[f32; 4]>,
    frame_id: u64,
    max_boxes: usize,
) -> DatasetResult<DatasetSample> {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let mut image_chw = vec![0.0f32; (width * height * 3) as usize];
    for (y, x, pixel) in img.enumerate_pixels() {
        let base = (y * width + x) as usize;
        image_chw[base] = pixel[0] as f32 / 255.0;
        image_chw[(width * height) as usize + base] = pixel[1] as f32 / 255.0;
        image_chw[2 * (width * height) as usize + base] = pixel[2] as f32 / 255.0;
    }

    if boxes.len() > max_boxes {
        boxes.truncate(max_boxes);
    }

    ONCE.call_once(|| {
        if boxes.is_empty() {
            eprintln!(
                "Debug: first sample had 0 boxes (image {}x{})",
                width, height
            );
        } else {
            eprintln!("Debug: first sample boxes {:?}", boxes);
        }
    });

    Ok(DatasetSample {
        frame_id,
        image_chw,
        width,
        height,
        boxes,
    })
}

fn letterbox_resize(
    img: &image::RgbImage,
    target_w: u32,
    target_h: u32,
) -> DatasetResult<(image::RgbImage, u32, u32)> {
    let (w, h) = img.dimensions();
    let scale = f32::min(target_w as f32 / w as f32, target_h as f32 / h as f32);
    let new_w = (w as f32 * scale).round() as u32;
    let new_h = (h as f32 * scale).round() as u32;
    let resized = image::imageops::resize(img, new_w, new_h, FilterType::Triangle);

    let pad_w = (target_w - new_w) / 2;
    let pad_h = (target_h - new_h) / 2;

    let mut canvas = image::RgbImage::new(target_w, target_h);
    image::imageops::replace(&mut canvas, &resized, pad_w.into(), pad_h.into());

    Ok((canvas, pad_w, pad_h))
}

fn normalize_boxes(labels: &[DetectionLabel], w: u32, h: u32) -> Vec<[f32; 4]> {
    labels
        .iter()
        .filter_map(|l| {
            if let Some(norm) = l.bbox_norm {
                Some(norm)
            } else {
                l.bbox_px.map(|px| {
                    [
                        px[0] / w as f32,
                        px[1] / h as f32,
                        px[2] / w as f32,
                        px[3] / h as f32,
                    ]
                })
            }
        })
        .map(|mut b| {
            for v in b.iter_mut() {
                *v = v.clamp(0.0, 1.0);
            }
            b
        })
        .collect()
}

fn normalize_boxes_with_px(
    labels: &[DetectionLabel],
    w: u32,
    h: u32,
) -> (Vec<[f32; 4]>, Vec<[f32; 4]>) {
    let mut norm = Vec::new();
    let mut pxs = Vec::new();
    for l in labels {
        if let Some(px) = l.bbox_px {
            norm.push([
                px[0] / w as f32,
                px[1] / h as f32,
                px[2] / w as f32,
                px[3] / h as f32,
            ]);
            pxs.push(px);
        } else if let Some(n) = l.bbox_norm {
            norm.push(n);
            pxs.push([
                n[0] * w as f32,
                n[1] * h as f32,
                n[2] * w as f32,
                n[3] * h as f32,
            ]);
        }
    }
    for b in norm.iter_mut() {
        for v in b.iter_mut() {
            *v = v.clamp(0.0, 1.0);
        }
    }
    (norm, pxs)
}

pub(crate) fn maybe_hflip(
    img: &mut image::RgbImage,
    boxes: &mut [[f32; 4]],
    prob: f32,
    rng: &mut dyn rand::RngCore,
) {
    if prob <= 0.0 {
        return;
    }
    if rng.random_range(0.0..1.0) < prob {
        image::imageops::flip_horizontal_in_place(img);
        for b in boxes.iter_mut() {
            let x0 = b[0];
            let x1 = b[2];
            b[0] = (1.0 - x1).clamp(0.0, 1.0);
            b[2] = (1.0 - x0).clamp(0.0, 1.0);
        }
    }
}

pub(crate) fn maybe_jitter(
    img: &mut image::RgbImage,
    prob: f32,
    strength: f32,
    rng: &mut dyn rand::RngCore,
) {
    if prob <= 0.0 || strength <= 0.0 {
        return;
    }
    if rng.random_range(0.0..1.0) >= prob {
        return;
    }
    let bright = 1.0 + rng.random_range(-strength..strength);
    let contrast = 1.0 + rng.random_range(-strength..strength);
    for pixel in img.pixels_mut() {
        for c in 0..3 {
            let v = pixel[c] as f32 / 255.0;
            let mut v = (v - 0.5) * contrast + 0.5;
            v *= bright;
            pixel[c] = (v.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
}

pub(crate) fn maybe_noise(
    img: &mut image::RgbImage,
    prob: f32,
    strength: f32,
    rng: &mut dyn rand::RngCore,
) {
    if prob <= 0.0 || strength <= 0.0 {
        return;
    }
    if rng.random_range(0.0..1.0) >= prob {
        return;
    }
    for pixel in img.pixels_mut() {
        for c in 0..3 {
            let noise = rng.random_range(-strength..strength);
            let v = (pixel[c] as f32 / 255.0 + noise).clamp(0.0, 1.0);
            pixel[c] = (v * 255.0) as u8;
        }
    }
}

pub(crate) fn maybe_scale_jitter(
    img: &mut image::RgbImage,
    boxes: &mut [[f32; 4]],
    prob: f32,
    min_scale: f32,
    max_scale: f32,
    rng: &mut dyn rand::RngCore,
) {
    if prob <= 0.0 || min_scale <= 0.0 || max_scale <= 0.0 {
        return;
    }
    if rng.random_range(0.0..1.0) >= prob {
        return;
    }
    let scale = rng.random_range(min_scale..max_scale);
    let (w, h) = img.dimensions();
    let new_w = max(1, (w as f32 * scale).round() as u32);
    let new_h = max(1, (h as f32 * scale).round() as u32);

    let resized = image::imageops::resize(img, new_w, new_h, FilterType::Triangle);
    let mut canvas = image::RgbImage::new(w, h);

    if new_w >= w && new_h >= h {
        // crop center
        let x0 = ((new_w - w) / 2) as i64;
        let y0 = ((new_h - h) / 2) as i64;
        image::imageops::replace(&mut canvas, &resized, -x0, -y0);
        let sx = scale;
        let sy = scale;
        for b in boxes.iter_mut() {
            let mut px0 = b[0] * w as f32 * sx - x0 as f32;
            let mut py0 = b[1] * h as f32 * sy - y0 as f32;
            let mut px1 = b[2] * w as f32 * sx - x0 as f32;
            let mut py1 = b[3] * h as f32 * sy - y0 as f32;
            px0 = px0.clamp(0.0, w as f32);
            py0 = py0.clamp(0.0, h as f32);
            px1 = px1.clamp(px0, w as f32);
            py1 = py1.clamp(py0, h as f32);
            b[0] = px0 / w as f32;
            b[1] = py0 / h as f32;
            b[2] = px1 / w as f32;
            b[3] = py1 / h as f32;
        }
    } else {
        // pad center
        let x0 = ((w - new_w) / 2) as i64;
        let y0 = ((h - new_h) / 2) as i64;
        image::imageops::replace(&mut canvas, &resized, x0, y0);
        let sx = scale;
        let sy = scale;
        for b in boxes.iter_mut() {
            let mut px0 = b[0] * w as f32 * sx + x0 as f32;
            let mut py0 = b[1] * h as f32 * sy + y0 as f32;
            let mut px1 = b[2] * w as f32 * sx + x0 as f32;
            let mut py1 = b[3] * h as f32 * sy + y0 as f32;
            px0 = px0.clamp(0.0, w as f32);
            py0 = py0.clamp(0.0, h as f32);
            px1 = px1.clamp(px0, w as f32);
            py1 = py1.clamp(py0, h as f32);
            b[0] = px0 / w as f32;
            b[1] = py0 / h as f32;
            b[2] = px1 / w as f32;
            b[3] = py1 / h as f32;
        }
    }

    *img = canvas;
}

pub(crate) fn maybe_blur(
    img: &mut image::RgbImage,
    prob: f32,
    sigma: f32,
    rng: &mut dyn rand::RngCore,
) {
    if prob <= 0.0 || sigma <= 0.0 {
        return;
    }
    if rng.random_range(0.0..1.0) >= prob {
        return;
    }
    let blurred = image::imageops::blur(img, sigma);
    *img = blurred;
}
#[cfg(test)]
mod aug_tests {
    use super::maybe_hflip;
    use rand::rng;

    #[test]
    fn hflip_boxes_are_inverted() {
        let mut img = image::RgbImage::new(2, 2);
        let mut boxes = vec![[0.25, 0.0, 0.75, 1.0]];
        let mut rng = rng();
        maybe_hflip(&mut img, &mut boxes, 1.0, &mut rng);
        let flipped = boxes[0];
        assert!((flipped[0] - 0.25).abs() < 1e-6);
        assert!((flipped[2] - 0.75).abs() < 1e-6);
        assert!(flipped[0] < flipped[2]);
    }
}
