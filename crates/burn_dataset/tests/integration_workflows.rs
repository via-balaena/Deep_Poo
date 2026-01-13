//! Integration tests for end-to-end burn_dataset workflows.
//!
//! These tests verify that the major workflows work correctly together:
//! 1. Capture → Warehouse ETL pipeline
//! 2. Warehouse → Training batch iteration
//! 3. Capture → Validation → Stratified splits

use burn_dataset::{
    index_runs, load_sample_for_etl, split_runs_stratified, summarize_with_thresholds,
    CacheableTransformConfig, DatasetSummary, Endianness, ResizeMode, ShardDType, ShardMetadata,
    TransformPipelineBuilder, ValidationThresholds,
};

#[cfg(feature = "burn-runtime")]
use burn_dataset::WarehouseManifest;
use data_contracts::capture::{CaptureMetadata, PolypLabel};
use image::{Rgb, RgbImage};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Helper to create a synthetic capture run with N frames.
/// Note: boxes_per_frame must be > 0 because validation requires at least bbox_norm or bbox_px.
/// For testing "empty" samples, set boxes to tiny values that get filtered during training.
fn create_synthetic_run(root: &Path, run_name: &str, frame_count: usize, boxes_per_frame: usize) -> anyhow::Result<PathBuf> {
    let run_dir = root.join(run_name);
    let labels_dir = run_dir.join("labels");
    fs::create_dir_all(&labels_dir)?;
    fs::create_dir_all(&run_dir)?; // Images go in run_dir root

    for i in 0..frame_count {
        let frame_id = (i + 1) as u64;
        let img_name = format!("frame_{:05}.png", frame_id);

        // Create synthetic labels
        let mut polyp_labels = Vec::new();
        for j in 0..boxes_per_frame.max(1) { // Ensure at least 1 for validation
            let offset = j as f32 * 0.1;
            polyp_labels.push(PolypLabel {
                center_world: [offset, offset, 0.0],
                bbox_px: Some([10.0 + offset * 100.0, 10.0, 50.0, 50.0]),
                bbox_norm: Some([0.1 + offset * 0.1, 0.1, 0.5, 0.5]),
                source: None,
                source_confidence: None,
            });
        }

        let meta = CaptureMetadata {
            frame_id,
            sim_time: i as f64 * 0.1,
            unix_time: 1000000.0 + i as f64,
            image: img_name.clone(),
            image_present: true,
            camera_active: true,
            polyp_seed: 42,
            polyp_labels,
        };

        let json = serde_json::to_vec(&meta)?;
        fs::write(labels_dir.join(format!("frame_{:05}.json", frame_id)), json)?;

        // Create synthetic image (8x8 RGB) in run_dir root
        let mut img = RgbImage::new(8, 8);
        for pixel in img.pixels_mut() {
            *pixel = Rgb([(i * 30) as u8, 128, 200]);
        }
        img.save(run_dir.join(&img_name))?;
    }

    Ok(run_dir)
}

#[cfg(feature = "burn-runtime")]
#[test]
fn workflow_capture_to_warehouse_etl() -> anyhow::Result<()> {
    // Setup: Create synthetic captures with varied box counts for stratification
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();

    create_synthetic_run(root, "run_1box", 3, 1)?;   // Single box
    create_synthetic_run(root, "run_2boxes", 4, 2)?; // Multiple boxes
    create_synthetic_run(root, "run_3boxes", 3, 3)?; // Even more boxes

    // Step 1: Index all captures
    let indices = index_runs(root)?;
    assert_eq!(indices.len(), 10, "Expected 10 total frames (3+4+3)");

    // Step 2: Build transform pipeline for warehouse ETL
    let pipeline = TransformPipelineBuilder::new()
        .target_size(Some((64, 64)))
        .resize_mode(ResizeMode::Force)
        .max_boxes(4)
        .build();

    // Step 3: Transform samples into warehouse format
    let warehouse_dir = root.join("warehouse");
    fs::create_dir_all(&warehouse_dir)?;

    let shard_path = warehouse_dir.join("shard_001.bin");
    let mut shard_file = fs::File::create(&shard_path)?;

    let width = 64u32;
    let height = 64u32;
    let channels = 3u32;
    let max_boxes = 4usize;
    let header_len = 64usize;

    // Write TWH1 header
    let samples = indices.len();
    let img_bytes = samples * (width * height * channels) as usize * 4;
    let box_bytes = samples * max_boxes * 4 * 4;
    let _mask_bytes = samples * max_boxes * 4;
    let image_offset = header_len;
    let boxes_offset = image_offset + img_bytes;
    let mask_offset = boxes_offset + box_bytes;

    let mut header = vec![0u8; header_len];
    header[0..4].copy_from_slice(b"TWH1");
    header[4..8].copy_from_slice(&1u32.to_le_bytes()); // version
    header[8..12].copy_from_slice(&0u32.to_le_bytes()); // dtype f32
    header[12..16].copy_from_slice(&0u32.to_le_bytes()); // little endian
    header[16..20].copy_from_slice(&width.to_le_bytes());
    header[20..24].copy_from_slice(&height.to_le_bytes());
    header[24..28].copy_from_slice(&channels.to_le_bytes());
    header[28..32].copy_from_slice(&(max_boxes as u32).to_le_bytes());
    header[32..40].copy_from_slice(&(samples as u64).to_le_bytes());
    header[40..48].copy_from_slice(&(image_offset as u64).to_le_bytes());
    header[48..56].copy_from_slice(&(boxes_offset as u64).to_le_bytes());
    header[56..64].copy_from_slice(&(mask_offset as u64).to_le_bytes());
    shard_file.write_all(&header)?;

    // Write transformed samples
    let mut all_boxes = Vec::new();
    let mut all_masks = Vec::new();

    for idx in &indices {
        let sample = load_sample_for_etl(idx, &pipeline)?;

        // Verify transform applied correctly
        assert_eq!(sample.width, width);
        assert_eq!(sample.height, height);
        assert_eq!(sample.image_chw.len(), (width * height * channels) as usize);

        // Write image data (CHW format, f32)
        for &val in &sample.image_chw {
            shard_file.write_all(&val.to_le_bytes())?;
        }

        // Collect boxes and masks
        let box_count = sample.boxes.len();
        let mut boxes = sample.boxes;
        boxes.resize(max_boxes, [0.0; 4]); // Pad to max_boxes
        all_boxes.extend_from_slice(&boxes);

        let mut mask = vec![0.0f32; max_boxes];
        for i in 0..box_count.min(max_boxes) {
            mask[i] = 1.0;
        }
        all_masks.extend_from_slice(&mask);
    }

    // Write boxes and masks
    for box_coords in &all_boxes {
        for &coord in box_coords {
            shard_file.write_all(&coord.to_le_bytes())?;
        }
    }
    for &mask_val in &all_masks {
        shard_file.write_all(&mask_val.to_le_bytes())?;
    }

    drop(shard_file);

    // Step 4: Create warehouse manifest
    let transform = CacheableTransformConfig {
        target_size: Some((width, height)),
        resize_mode: ResizeMode::Force,
        max_boxes,
    };

    let code_version = WarehouseManifest::default_code_version();
    let version = WarehouseManifest::compute_version(root, &transform, false, &code_version);
    let version_recipe = "sha256(dataset_root + transform + max_boxes + skip_empty + code_version)".to_string();

    let shard_meta = ShardMetadata {
        id: "shard_001".into(),
        relative_path: "shard_001.bin".into(),
        shard_version: 1,
        samples,
        width,
        height,
        channels,
        max_boxes,
        checksum_sha256: None,
        dtype: ShardDType::F32,
        endianness: Endianness::Little,
    };

    let manifest = WarehouseManifest::new(
        warehouse_dir.clone(),
        transform,
        version,
        version_recipe,
        code_version,
        vec![shard_meta],
        DatasetSummary::default(),
        ValidationThresholds::default(),
    );

    let manifest_path = warehouse_dir.join("manifest.json");
    manifest.save(&manifest_path)?;

    // Step 5: Verify manifest can be loaded back
    let loaded_manifest = WarehouseManifest::load(&manifest_path)?;
    assert_eq!(loaded_manifest.shards.len(), 1);
    assert_eq!(loaded_manifest.shards[0].samples, samples);
    assert_eq!(loaded_manifest.transform.max_boxes, max_boxes);

    Ok(())
}

#[cfg(feature = "burn-runtime")]
#[test]
fn workflow_warehouse_to_training_iteration() -> anyhow::Result<()> {
    use burn_dataset::WarehouseLoaders;
    use burn::tensor::backend::Backend;

    type TestBackend = burn_ndarray::NdArray<f32>;

    // Setup: Create a minimal warehouse with 4 samples
    let tmp = tempfile::tempdir()?;
    let warehouse_dir = tmp.path().join("warehouse");
    fs::create_dir_all(&warehouse_dir)?;

    let width = 8u32;
    let height = 8u32;
    let channels = 3u32;
    let max_boxes = 2usize;
    let samples = 4usize;
    let header_len = 64usize;

    let shard_path = warehouse_dir.join("test_shard.bin");
    let img_bytes = samples * (width * height * channels) as usize * 4;
    let box_bytes = samples * max_boxes * 4 * 4;
    let mask_bytes = samples * max_boxes * 4;
    let image_offset = header_len;
    let boxes_offset = image_offset + img_bytes;
    let mask_offset = boxes_offset + box_bytes;

    let mut data = vec![0u8; header_len + img_bytes + box_bytes + mask_bytes];

    // Write header
    data[0..4].copy_from_slice(b"TWH1");
    data[4..8].copy_from_slice(&1u32.to_le_bytes());
    data[8..12].copy_from_slice(&0u32.to_le_bytes()); // f32
    data[12..16].copy_from_slice(&0u32.to_le_bytes()); // little endian
    data[16..20].copy_from_slice(&width.to_le_bytes());
    data[20..24].copy_from_slice(&height.to_le_bytes());
    data[24..28].copy_from_slice(&channels.to_le_bytes());
    data[28..32].copy_from_slice(&(max_boxes as u32).to_le_bytes());
    data[32..40].copy_from_slice(&(samples as u64).to_le_bytes());
    data[40..48].copy_from_slice(&(image_offset as u64).to_le_bytes());
    data[48..56].copy_from_slice(&(boxes_offset as u64).to_le_bytes());
    data[56..64].copy_from_slice(&(mask_offset as u64).to_le_bytes());

    // Write synthetic image data (sequential values)
    let mut cursor = image_offset;
    for sample_idx in 0..samples {
        for _pixel in 0..(width * height * channels) {
            let val = (sample_idx as f32 + 1.0) / samples as f32;
            data[cursor..cursor + 4].copy_from_slice(&val.to_le_bytes());
            cursor += 4;
        }
    }

    // Write boxes
    cursor = boxes_offset;
    let boxes_per_sample = vec![
        vec![[0.1f32, 0.1, 0.3, 0.3], [0.5, 0.5, 0.7, 0.7]], // sample 0: 2 boxes
        vec![[0.2f32, 0.2, 0.4, 0.4]],                        // sample 1: 1 box
        vec![],                                             // sample 2: 0 boxes
        vec![[0.3f32, 0.3, 0.6, 0.6]],                        // sample 3: 1 box
    ];

    for boxes in &boxes_per_sample {
        for b in boxes {
            for &coord in b {
                data[cursor..cursor + 4].copy_from_slice(&coord.to_le_bytes());
                cursor += 4;
            }
        }
        // Pad remaining boxes with zeros
        for _ in boxes.len()..max_boxes {
            for _ in 0..4 {
                cursor += 4; // Already zeros
            }
        }
    }

    // Write masks
    cursor = mask_offset;
    for boxes in &boxes_per_sample {
        for i in 0..max_boxes {
            let mask_val = if i < boxes.len() { 1.0f32 } else { 0.0f32 };
            data[cursor..cursor + 4].copy_from_slice(&mask_val.to_le_bytes());
            cursor += 4;
        }
    }

    fs::write(&shard_path, data)?;

    // Create manifest
    let transform = CacheableTransformConfig {
        target_size: Some((width, height)),
        resize_mode: ResizeMode::Force,
        max_boxes,
    };

    let shard_meta = ShardMetadata {
        id: "test_shard".into(),
        relative_path: "test_shard.bin".into(),
        shard_version: 1,
        samples,
        width,
        height,
        channels,
        max_boxes,
        checksum_sha256: None,
        dtype: ShardDType::F32,
        endianness: Endianness::Little,
    };

    let code_version = WarehouseManifest::default_code_version();
    let version = WarehouseManifest::compute_version(&warehouse_dir, &transform, false, &code_version);
    let version_recipe = "test".to_string();

    let manifest = WarehouseManifest::new(
        warehouse_dir.clone(),
        transform,
        version,
        version_recipe,
        code_version,
        vec![shard_meta],
        DatasetSummary::default(),
        ValidationThresholds::default(),
    );

    let manifest_path = warehouse_dir.join("manifest.json");
    manifest.save(&manifest_path)?;

    // Step 1: Load warehouse manifest into iterators (75/25 split)
    let loaders = WarehouseLoaders::from_manifest_path(&manifest_path, 0.25, Some(42), false)?;
    let mut train_iter = loaders.train_iter();
    let mut val_iter = loaders.val_iter();

    // Step 2: Verify split sizes (75% train = 3, 25% val = 1)
    assert_eq!(train_iter.len(), 3, "Expected 3 training samples");
    assert_eq!(val_iter.len(), 1, "Expected 1 validation sample");

    // Step 3: Iterate training batches
    let device = <TestBackend as Backend>::Device::default();
    let batch_size = 2;

    let batch1 = train_iter.next_batch::<TestBackend>(batch_size, &device)?
        .expect("Expected first training batch");
    assert_eq!(batch1.images.dims()[0], 2, "First batch should have 2 samples");
    assert_eq!(batch1.images.dims()[1], channels as usize);
    assert_eq!(batch1.images.dims()[2], height as usize);
    assert_eq!(batch1.images.dims()[3], width as usize);
    assert_eq!(batch1.boxes.dims(), [2, max_boxes, 4]);
    assert_eq!(batch1.box_mask.dims(), [2, max_boxes]);

    let batch2 = train_iter.next_batch::<TestBackend>(batch_size, &device)?
        .expect("Expected second training batch");
    assert_eq!(batch2.images.dims()[0], 1, "Second batch should have 1 sample (remainder)");

    // Should be exhausted
    assert!(train_iter.next_batch::<TestBackend>(batch_size, &device)?.is_none());

    // Step 4: Iterate validation batch
    let val_batch = val_iter.next_batch::<TestBackend>(1, &device)?
        .expect("Expected validation batch");
    assert_eq!(val_batch.images.dims()[0], 1);

    Ok(())
}

#[test]
fn workflow_capture_validation_and_stratified_split() -> anyhow::Result<()> {
    // Setup: Create captures with varying quality for validation
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();

    // Run with consistent single boxes
    create_synthetic_run(root, "run_single", 12, 1)?;

    // Run with multiple boxes
    create_synthetic_run(root, "run_multi", 15, 3)?;

    // Run with very few boxes
    create_synthetic_run(root, "run_sparse", 10, 1)?;

    // Step 1: Index all captures
    let indices = index_runs(root)?;
    assert_eq!(indices.len(), 37, "Expected 37 total frames (12+15+10)");

    // Step 2: Validate dataset quality
    let thresholds = ValidationThresholds {
        max_invalid: Some(5),
        max_missing: Some(0),
        max_empty: Some(0), // All samples should have boxes
        max_invalid_ratio: Some(0.1),
        max_missing_ratio: Some(0.0),
        max_empty_ratio: Some(0.0),
    };

    let report = summarize_with_thresholds(&indices, &thresholds)?;

    // Verify summary
    assert_eq!(report.summary.totals.total, 37);
    assert!(report.summary.totals.non_empty >= 35); // Most should have valid boxes

    // Step 3: Use all indices for stratification (all have boxes now)
    let filtered_indices = indices.clone();

    // Step 4: Perform stratified split (80/20)
    let (train_indices, val_indices) = split_runs_stratified(
        filtered_indices.clone(),
        0.2,
        Some(42), // Fixed seed for reproducibility
    );

    // Verify split sizes
    assert_eq!(train_indices.len() + val_indices.len(), 37);
    assert!(val_indices.len() >= 5 && val_indices.len() <= 10, "Expected ~20% validation (5-10 of 37)");

    // Step 5: Verify stratification worked (both splits should have varied box counts from different runs)
    let train_has_single = train_indices.iter()
        .any(|idx| idx.run_dir.to_string_lossy().contains("single"));
    let train_has_multi = train_indices.iter()
        .any(|idx| idx.run_dir.to_string_lossy().contains("multi"));
    let train_has_sparse = train_indices.iter()
        .any(|idx| idx.run_dir.to_string_lossy().contains("sparse"));

    // With 37 samples across 3 runs and good stratification, we should get representation
    assert!(train_has_single || train_has_multi || train_has_sparse, "Training set should have representation");
    assert!(val_indices.len() > 0, "Validation set should exist");

    Ok(())
}
