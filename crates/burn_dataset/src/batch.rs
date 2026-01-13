//! Batch iteration for training and validation.

use crate::aug::{DatasetConfig, TransformPipeline};
use crate::capture::index_runs;
use crate::splits::split_runs;
use crate::types::{BurnDatasetError, DatasetResult, DatasetSample, SampleIndex};
use rand::{seq::SliceRandom, SeedableRng};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[cfg(feature = "burn-runtime")]
use crate::capture::load_sample;
#[cfg(feature = "burn-runtime")]
use rayon::prelude::*;

#[cfg(feature = "burn-runtime")]
pub(crate) const DEFAULT_LOG_EVERY_SAMPLES: usize = 1000;

pub fn build_train_val_iters(
    root: &Path,
    val_ratio: f32,
    train_cfg: DatasetConfig,
    val_cfg: Option<DatasetConfig>,
) -> DatasetResult<(BatchIter, BatchIter)> {
    let indices = index_runs(root)?;
    let (train_idx, val_idx) = split_runs(indices, val_ratio);
    // For val, default to no shuffle/aug.
    let val_cfg = val_cfg.unwrap_or_else(|| DatasetConfig {
        shuffle: false,
        drop_last: false,
        flip_horizontal_prob: 0.0,
        color_jitter_prob: 0.0,
        color_jitter_strength: 0.0,
        scale_jitter_prob: 0.0,
        noise_prob: 0.0,
        blur_prob: 0.0,
        ..train_cfg.clone()
    });
    // Train typically shuffles; keep whatever caller set.
    let train_iter = BatchIter::from_indices(train_idx, train_cfg)?;
    let val_iter = BatchIter::from_indices(val_idx, val_cfg)?;
    Ok((train_iter, val_iter))
}


pub struct BurnBatch<B: burn::tensor::backend::Backend> {
    pub images: burn::tensor::Tensor<B, 4>,
    pub boxes: burn::tensor::Tensor<B, 3>,
    pub box_mask: burn::tensor::Tensor<B, 2>,
    pub frame_ids: burn::tensor::Tensor<B, 1>,
}

#[cfg(feature = "burn-runtime")]
pub struct BatchIter {
    indices: Vec<SampleIndex>,
    cursor: usize,
    cfg: DatasetConfig,
    processed_samples: usize,
    processed_batches: usize,
    skipped_total: usize,
    skipped_empty: usize,
    skipped_missing: usize,
    skipped_errors: usize,
    warn_once: bool,
    warned_counts: bool,
    started: Instant,
    total_load_time: Duration,
    total_assemble_time: Duration,
    last_log: Instant,
    last_logged_samples: usize,
    log_every_samples: Option<usize>,
    permissive_errors: bool,
    images_buf: Vec<f32>,
    boxes_buf: Vec<f32>,
    mask_buf: Vec<f32>,
    frame_ids_buf: Vec<f32>,
    trace_path: Option<PathBuf>,
    trace_file: Option<std::fs::File>,
    pipeline: TransformPipeline,
}

#[cfg(feature = "burn-runtime")]
impl BatchIter {
    pub fn from_root(root: &Path, cfg: DatasetConfig) -> DatasetResult<Self> {
        let indices = index_runs(root)?;
        Self::from_indices(indices, cfg)
    }

    pub fn from_indices(mut indices: Vec<SampleIndex>, cfg: DatasetConfig) -> DatasetResult<Self> {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;
        let mut rng = match cfg.seed {
            Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
            None => rand::rngs::StdRng::from_rng(&mut rand::rng()),
        };
        if cfg.shuffle {
            indices.shuffle(&mut rng);
        }
        let log_every_samples = match std::env::var("BURN_DATASET_LOG_EVERY") {
            Ok(val) => {
                if val.eq_ignore_ascii_case("off") || val.trim() == "0" {
                    None
                } else {
                    val.parse::<usize>().ok().filter(|v| *v > 0)
                }
            }
            Err(_) => Some(DEFAULT_LOG_EVERY_SAMPLES),
        };
        let permissive_errors = std::env::var("BURN_DATASET_PERMISSIVE")
            .ok()
            .map(|v| v.trim().to_ascii_lowercase())
            .map(|v| v == "0" || v == "false" || v == "off")
            .map(|strict| !strict)
            .unwrap_or(true);
        let warn_once = std::env::var("BURN_DATASET_WARN_ONCE")
            .ok()
            .map(|v| v.trim().to_ascii_lowercase())
            .map(|v| v == "1" || v == "true" || v == "on")
            .unwrap_or(false);
        let trace_path = std::env::var("BURN_DATASET_TRACE")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from);
        let now = Instant::now();
        let pipeline = cfg
            .transform
            .clone()
            .unwrap_or_else(|| TransformPipeline::from_config(&cfg));
        Ok(Self {
            indices,
            cursor: 0,
            cfg,
            processed_samples: 0,
            processed_batches: 0,
            skipped_total: 0,
            skipped_empty: 0,
            skipped_missing: 0,
            skipped_errors: 0,
            warn_once,
            warned_counts: false,
            started: now,
            total_load_time: Duration::ZERO,
            total_assemble_time: Duration::ZERO,
            last_log: now,
            last_logged_samples: 0,
            log_every_samples,
            permissive_errors,
            images_buf: Vec::new(),
            boxes_buf: Vec::new(),
            mask_buf: Vec::new(),
            frame_ids_buf: Vec::new(),
            trace_path,
            trace_file: None,
            pipeline,
        })
    }

    pub fn next_batch<B: burn::tensor::backend::Backend>(
        &mut self,
        batch_size: usize,
        device: &B::Device,
    ) -> DatasetResult<Option<BurnBatch<B>>> {
        loop {
            if self.cursor >= self.indices.len() {
                return Ok(None);
            }
            let end = (self.cursor + batch_size).min(self.indices.len());
            let slice = &self.indices[self.cursor..end];
            self.cursor = end;

            self.images_buf.clear();
            self.boxes_buf.clear();
            self.mask_buf.clear();
            self.frame_ids_buf.clear();

            let mut expected_size: Option<(u32, u32)> = None;
            let mut skipped_empty = 0usize;
            let skipped_missing = 0usize;

            let t_load = Instant::now();
            let mut loaded: Vec<_> = slice
                .par_iter()
                .enumerate()
                .map(|(i, idx)| (i, idx, load_sample(idx, &self.pipeline)))
                .collect();
            loaded.sort_by_key(|(i, _, _)| *i);
            let load_elapsed = t_load.elapsed();

            for (_i, idx, res) in loaded {
                let sample = match res {
                    Ok(s) => s,
                    Err(e) => {
                        if self.permissive_errors {
                            if !self.warn_once {
                                eprintln!(
                                    "Warning: skipping label {}: {e}",
                                    idx.label_path.display()
                                );
                            }
                            self.skipped_errors += 1;
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                };
                if self.cfg.skip_empty_labels && sample.boxes.is_empty() {
                    if !self.warn_once {
                        eprintln!(
                            "Warning: no boxes found in {} (skipping sample)",
                            idx.label_path.display()
                        );
                    }
                    skipped_empty += 1;
                    continue;
                }

                let size = (sample.width, sample.height);
                match expected_size {
                    None => expected_size = Some(size),
                    Some(sz) if sz != size => {
                        return Err(BurnDatasetError::Other(
                            "batch contains varying image sizes; set a target_size to force consistency"
                                .to_string(),
                        ));
                    }
                    _ => {}
                }

                // Ensure backing buffers are large enough after the first sample sets size.
                if let Some((w, h)) = expected_size {
                    let elems = batch_size * 3 * w as usize * h as usize;
                    if self.images_buf.capacity() < elems {
                        self.images_buf.reserve(elems - self.images_buf.capacity());
                    }
                    let box_elems = batch_size * self.cfg.max_boxes * 4;
                    if self.boxes_buf.capacity() < box_elems {
                        self.boxes_buf
                            .reserve(box_elems - self.boxes_buf.capacity());
                        self.mask_buf
                            .reserve(box_elems / 4 - self.mask_buf.capacity());
                    }
                    if self.frame_ids_buf.capacity() < batch_size {
                        self.frame_ids_buf
                            .reserve(batch_size - self.frame_ids_buf.capacity());
                    }
                }

                self.frame_ids_buf.push(sample.frame_id as f32);
                self.images_buf.extend_from_slice(&sample.image_chw);

                let mut padded = vec![0.0f32; self.cfg.max_boxes * 4];
                let mut mask = vec![0.0f32; self.cfg.max_boxes];
                for (i, b) in sample.boxes.iter().take(self.cfg.max_boxes).enumerate() {
                    padded[i * 4] = b[0];
                    padded[i * 4 + 1] = b[1];
                    padded[i * 4 + 2] = b[2];
                    padded[i * 4 + 3] = b[3];
                    mask[i] = 1.0;
                }
                self.boxes_buf.extend_from_slice(&padded);
                self.mask_buf.extend_from_slice(&mask);
            }

            if self.images_buf.is_empty() {
                if skipped_empty > 0 || skipped_missing > 0 {
                    self.skipped_total += skipped_empty + skipped_missing;
                    self.skipped_empty += skipped_empty;
                    self.skipped_missing += skipped_missing;
                    continue;
                } else {
                    return Ok(None);
                }
            }

            let t_assemble = Instant::now();
            let (width, height) = expected_size.expect("batch size > 0 ensures size is set");
            let batch_len = self.frame_ids_buf.len();
            if self.cfg.drop_last && batch_len < batch_size {
                if self.cursor >= self.indices.len() {
                    return Ok(None);
                } else {
                    continue;
                }
            }
            let image_shape = [batch_len, 3, height as usize, width as usize];
            let boxes_shape = [batch_len, self.cfg.max_boxes, 4];
            let mask_shape = [batch_len, self.cfg.max_boxes];

            let images =
                burn::tensor::Tensor::<B, 1>::from_floats(self.images_buf.as_slice(), device)
                    .reshape(image_shape);
            let boxes =
                burn::tensor::Tensor::<B, 1>::from_floats(self.boxes_buf.as_slice(), device)
                    .reshape(boxes_shape);
            let box_mask =
                burn::tensor::Tensor::<B, 1>::from_floats(self.mask_buf.as_slice(), device)
                    .reshape(mask_shape);
            let frame_ids =
                burn::tensor::Tensor::<B, 1>::from_floats(self.frame_ids_buf.as_slice(), device)
                    .reshape([batch_len]);
            let assemble_elapsed = t_assemble.elapsed();

            self.processed_samples += batch_len;
            self.processed_batches += 1;
            self.skipped_total += skipped_empty + skipped_missing;
            self.skipped_empty += skipped_empty;
            self.skipped_missing += skipped_missing;
            self.total_load_time += load_elapsed;
            self.total_assemble_time += assemble_elapsed;
            self.maybe_trace(
                batch_len,
                width as usize,
                height as usize,
                load_elapsed,
                assemble_elapsed,
            );
            self.maybe_log_progress();

            return Ok(Some(BurnBatch {
                images,
                boxes,
                box_mask,
                frame_ids,
            }));
        }
    }

    fn maybe_log_progress(&mut self) {
        let Some(threshold) = self.log_every_samples else {
            return;
        };
        let processed_since = self
            .processed_samples
            .saturating_sub(self.last_logged_samples);
        let elapsed = self.started.elapsed();
        let since_last = self.last_log.elapsed();
        let should_log = processed_since >= threshold || since_last >= Duration::from_secs(30);
        if !should_log {
            return;
        }
        let secs = elapsed.as_secs_f32().max(0.001);
        let rate = self.processed_samples as f32 / secs;
        let avg_load_ms = if self.processed_batches > 0 {
            (self.total_load_time.as_secs_f64() * 1000.0) / self.processed_batches as f64
        } else {
            0.0
        };
        let avg_assemble_ms = if self.processed_batches > 0 {
            (self.total_assemble_time.as_secs_f64() * 1000.0) / self.processed_batches as f64
        } else {
            0.0
        };
        if !self.warn_once || !self.warned_counts {
            eprintln!(
                "[dataset] batches={} samples={} skipped_empty={} skipped_missing={} skipped_errors={} elapsed={:.1}s rate={:.1} img/s avg_load_ms={:.2} avg_assemble_ms={:.2}",
                self.processed_batches,
                self.processed_samples,
                self.skipped_empty,
                self.skipped_missing,
                self.skipped_errors,
                secs,
                rate,
                avg_load_ms,
                avg_assemble_ms
            );
        }
        self.last_logged_samples = self.processed_samples;
        self.last_log = Instant::now();
        if self.warn_once {
            self.warned_counts = true;
        }
    }

    fn maybe_trace(
        &mut self,
        batch_len: usize,
        width: usize,
        height: usize,
        load_elapsed: Duration,
        assemble_elapsed: Duration,
    ) {
        let Some(path) = &self.trace_path else {
            return;
        };
        if self.trace_file.is_none() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                Ok(f) => self.trace_file = Some(f),
                Err(e) => {
                    eprintln!("Failed to open trace file {}: {e}", path.display());
                    self.trace_path = None;
                    return;
                }
            }
        }
        let Some(file) = self.trace_file.as_mut() else {
            return;
        };
        let record = serde_json::json!({
            "batch": self.processed_batches,
            "samples": batch_len,
            "width": width,
            "height": height,
            "max_boxes": self.cfg.max_boxes,
            "skipped_empty_total": self.skipped_empty,
            "skipped_missing_total": self.skipped_missing,
            "skipped_errors_total": self.skipped_errors,
            "load_ms": load_elapsed.as_secs_f64() * 1000.0,
            "assemble_ms": assemble_elapsed.as_secs_f64() * 1000.0,
            "timestamp_ms": self.started.elapsed().as_millis() as u64
        });
        if let Err(e) = writeln!(file, "{}", record) {
            eprintln!("Failed to write trace record: {e}");
            self.trace_path = None;
            self.trace_file = None;
        }
    }
}

