//! Loading and indexing capture dataset files.

use crate::aug::{DatasetConfig, TransformPipeline};
use crate::types::{BurnDatasetError, DatasetResult, DatasetSample, LabelEntry, PolypLabel, ResizeMode, RunSummary, DatasetSummary, SampleIndex};
use image;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;

fn validate_label_entry(meta: &LabelEntry, path: &Path) -> DatasetResult<()> {
    if meta.image.trim().is_empty() {
        return Err(BurnDatasetError::Validation {
            path: path.to_path_buf(),
            msg: "missing image filename".to_string(),
        });
    }
    if meta.polyp_labels.is_empty() {
        return Err(BurnDatasetError::Validation {
            path: path.to_path_buf(),
            msg: "no polyp_labels entries".to_string(),
        });
    }
    for (i, lbl) in meta.polyp_labels.iter().enumerate() {
        match (lbl.bbox_norm, lbl.bbox_px) {
            (None, None) => {
                return Err(BurnDatasetError::Validation {
                    path: path.to_path_buf(),
                    msg: format!("polyp_labels[{i}] has no bbox_norm or bbox_px"),
                });
            }
            (Some(b), _) => {
                if b.iter().any(|v| !v.is_finite() || *v < 0.0 || *v > 1.0) {
                    return Err(BurnDatasetError::Validation {
                        path: path.to_path_buf(),
                        msg: format!("polyp_labels[{i}] bbox_norm out of [0,1]"),
                    });
                }
                if b[0] >= b[2] || b[1] >= b[3] {
                    return Err(BurnDatasetError::Validation {
                        path: path.to_path_buf(),
                        msg: format!("polyp_labels[{i}] bbox_norm min>=max ({b:?})"),
                    });
                }
            }
            (_, Some(b)) => {
                if b.iter().any(|v| !v.is_finite()) {
                    return Err(BurnDatasetError::Validation {
                        path: path.to_path_buf(),
                        msg: format!("polyp_labels[{i}] bbox_px contains non-finite values"),
                    });
                }
                if b[0] >= b[2] || b[1] >= b[3] {
                    return Err(BurnDatasetError::Validation {
                        path: path.to_path_buf(),
                        msg: format!("polyp_labels[{i}] bbox_px min>=max ({b:?})"),
                    });
                }
            }
        }
    }
    Ok(())
}

fn label_has_box(meta: &LabelEntry) -> bool {
    meta.polyp_labels.iter().any(|lbl| {
        lbl.bbox_norm
            .is_some_and(|b| b.iter().all(|v| v.is_finite()) && b[0] < b[2] && b[1] < b[3])
            || lbl
                .bbox_px
                .is_some_and(|b| b.iter().all(|v| v.is_finite()) && b[0] < b[2] && b[1] < b[3])
    })
}

pub fn summarize_runs(indices: &[SampleIndex]) -> DatasetResult<DatasetSummary> {
    let mut by_run: std::collections::BTreeMap<PathBuf, RunSummary> =
        std::collections::BTreeMap::new();
    for idx in indices {
        let entry = by_run
            .entry(idx.run_dir.clone())
            .or_insert_with(|| RunSummary {
                run_dir: idx.run_dir.clone(),
                ..Default::default()
            });
        match fs::read(&idx.label_path)
            .ok()
            .and_then(|raw| serde_json::from_slice::<LabelEntry>(&raw).ok())
        {
            None => {
                entry.invalid += 1;
            }
            Some(meta) => {
                if meta.image.trim().is_empty() {
                    entry.missing_image += 1;
                    continue;
                }
                let img_path = idx.run_dir.join(&meta.image);
                if !meta.image_present || !img_path.exists() {
                    entry.missing_file += 1;
                    continue;
                }
                entry.total += 1;
                if label_has_box(&meta) {
                    entry.non_empty += 1;
                } else {
                    entry.empty += 1;
                }
            }
        }
    }
    let mut totals = RunSummary::default();
    for summary in by_run.values() {
        totals.total += summary.total;
        totals.non_empty += summary.non_empty;
        totals.empty += summary.empty;
        totals.missing_image += summary.missing_image;
        totals.missing_file += summary.missing_file;
        totals.invalid += summary.invalid;
    }
    Ok(DatasetSummary {
        runs: by_run.into_values().collect(),
        totals,
    })
}

/// Scan a captures root (e.g., `assets/datasets/captures`) and index all label files.
pub fn index_runs(root: &Path) -> DatasetResult<Vec<SampleIndex>> {
    let mut indices = Vec::new();
    let entries = fs::read_dir(root).map_err(|e| BurnDatasetError::Io {
        path: root.to_path_buf(),
        source: e,
    })?;
    for entry in entries {
        let Ok(run) = entry else { continue };
        let run_path = run.path();
        if !run_path.is_dir() {
            continue;
        }
        let labels_dir = run_path.join("labels");
        if !labels_dir.exists() {
            continue;
        }
        let labels_iter = fs::read_dir(&labels_dir).map_err(|e| BurnDatasetError::Io {
            path: labels_dir.clone(),
            source: e,
        })?;
        for label in labels_iter {
            let Ok(label_entry) = label else { continue };
            let label_path = label_entry.path();
            if label_path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            indices.push(SampleIndex {
                run_dir: run_path.clone(),
                label_path,
            });
        }
    }
    indices.sort_by(|a, b| a.label_path.cmp(&b.label_path));
    Ok(indices)
}

/// Load a capture run into an in-memory vector (eager). Prefer `BatchIter` for large sets.
pub fn load_run_dataset(run_dir: &Path) -> DatasetResult<Vec<DatasetSample>> {
    let labels_dir = run_dir.join("labels");
    if !labels_dir.exists() {
        return Err(BurnDatasetError::Io {
            path: labels_dir.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "labels directory missing"),
        });
    }

    let mut label_paths: Vec<PathBuf> = fs::read_dir(&labels_dir)
        .map_err(|e| BurnDatasetError::Io {
            path: labels_dir.clone(),
            source: e,
        })?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    label_paths.sort();

    let cfg = DatasetConfig {
        target_size: None,
        resize_mode: ResizeMode::Force,
        flip_horizontal_prob: 0.0,
        color_jitter_prob: 0.0,
        color_jitter_strength: 0.0,
        scale_jitter_prob: 0.0,
        scale_jitter_min: 1.0,
        scale_jitter_max: 1.0,
        noise_prob: 0.0,
        noise_strength: 0.0,
        blur_prob: 0.0,
        blur_sigma: 0.0,
        max_boxes: usize::MAX,
        shuffle: false,
        seed: None,
        skip_empty_labels: true,
        drop_last: false,
        transform: None,
    };
    let pipeline = cfg
        .transform
        .clone()
        .unwrap_or_else(|| TransformPipeline::from_config(&cfg));
    let mut samples = Vec::new();
    for label_path in label_paths {
        let sample = load_sample(
            &SampleIndex {
                run_dir: run_dir.to_path_buf(),
                label_path,
            },
            &pipeline,
        )?;
        samples.push(sample);
    }

    Ok(samples)
}

pub(crate) fn load_sample(idx: &SampleIndex, pipeline: &TransformPipeline) -> DatasetResult<DatasetSample> {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let raw = fs::read(&idx.label_path).map_err(|e| BurnDatasetError::Io {
        path: idx.label_path.clone(),
        source: e,
    })?;
    let meta: LabelEntry = serde_json::from_slice(&raw).map_err(|e| BurnDatasetError::Json {
        path: idx.label_path.clone(),
        source: e,
    })?;
    validate_label_entry(&meta, &idx.label_path)?;
    if !meta.image_present {
        return Err(BurnDatasetError::MissingImage {
            path: idx.label_path.clone(),
        });
    }

    let img_path = idx.run_dir.join(&meta.image);
    if !img_path.exists() {
        return Err(BurnDatasetError::MissingImageFile {
            path: idx.label_path.clone(),
            image: img_path,
        });
    }
    let img = image::open(&img_path)
        .map_err(|e| BurnDatasetError::Image {
            path: img_path.clone(),
            source: e,
        })?
        .to_rgb8();
    let sample = pipeline.apply(img, &meta)?;
    ONCE.call_once(|| {
        if sample.boxes.is_empty() {
            eprintln!(
                "Debug: first sample {} has 0 boxes (image {}x{})",
                idx.label_path.display(),
                sample.width,
                sample.height
            );
        } else {
            eprintln!(
                "Debug: first sample {} boxes {:?}",
                idx.label_path.display(),
                sample.boxes
            );
        }
    });
    Ok(sample)
}

/// Public entrypoint for deterministic sample loading (used by the warehouse ETL).
pub fn load_sample_for_etl(
    idx: &SampleIndex,
    pipeline: &TransformPipeline,
) -> DatasetResult<DatasetSample> {
    load_sample(idx, pipeline)
}
