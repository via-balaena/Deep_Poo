//! Dataset validation and quality checks.

use crate::capture::{index_runs, summarize_runs};
use crate::types::{
    DatasetResult, DatasetSummary, SampleIndex, ValidationOutcome, ValidationReport,
    ValidationThresholds,
};
use std::path::Path;

fn apply_thresholds(
    label: &str,
    count: usize,
    ratio: f32,
    max_count: Option<usize>,
    max_ratio: Option<f32>,
    outcome: &mut ValidationOutcome,
    reasons: &mut Vec<String>,
) {
    if let Some(max) = max_count {
        if count > max {
            *outcome = ValidationOutcome::Fail;
            reasons.push(format!("{label}: {count} exceeds max {max}"));
        }
    }
    if let Some(max_r) = max_ratio {
        if ratio > max_r {
            *outcome = ValidationOutcome::Fail;
            reasons.push(format!(
                "{label}: ratio {:.3} exceeds max {:.3}",
                ratio, max_r
            ));
        }
    }
    if count > 0 {
        if *outcome == ValidationOutcome::Pass {
            *outcome = ValidationOutcome::Warn;
        }
        reasons.push(format!("{label}: {count} observed"));
    }
}

pub fn validate_summary(
    summary: DatasetSummary,
    thresholds: &ValidationThresholds,
) -> ValidationReport {
    let totals = &summary.totals;
    let checked = totals.total + totals.missing_file + totals.missing_image + totals.invalid;
    let denom = checked.max(1) as f32;
    let missing = totals.missing_file + totals.missing_image;

    let mut outcome = ValidationOutcome::Pass;
    let mut reasons = Vec::new();

    apply_thresholds(
        "missing (image/file)",
        missing,
        missing as f32 / denom,
        thresholds.max_missing,
        thresholds.max_missing_ratio,
        &mut outcome,
        &mut reasons,
    );
    apply_thresholds(
        "invalid labels",
        totals.invalid,
        totals.invalid as f32 / denom,
        thresholds.max_invalid,
        thresholds.max_invalid_ratio,
        &mut outcome,
        &mut reasons,
    );
    apply_thresholds(
        "empty labels",
        totals.empty,
        totals.empty as f32 / denom,
        thresholds.max_empty,
        thresholds.max_empty_ratio,
        &mut outcome,
        &mut reasons,
    );

    ValidationReport {
        outcome,
        reasons,
        summary,
    }
}

pub fn summarize_with_thresholds(
    indices: &[SampleIndex],
    thresholds: &ValidationThresholds,
) -> DatasetResult<ValidationReport> {
    let summary = summarize_runs(indices)?;
    Ok(validate_summary(summary, thresholds))
}

pub fn summarize_root_with_thresholds(
    root: &Path,
    thresholds: &ValidationThresholds,
) -> DatasetResult<ValidationReport> {
    let indices = index_runs(root)?;
    summarize_with_thresholds(&indices, thresholds)
}
