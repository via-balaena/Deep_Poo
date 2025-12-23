#![cfg(feature = "burn_runtime")]

use anyhow::Result;
use colon_sim::tools::burn_dataset::{BatchIter, DatasetConfig, SampleIndex};
use colon_sim::burn_model::{TinyDet, TinyDetConfig};
use burn::backend::{ndarray::NdArray, autodiff::Autodiff};
use burn::lr_scheduler::linear::LinearLrSchedulerConfig;
use burn::optim::{AdamW, AdamWConfig, GradientsParams, Optimizer};
use burn::optim::adaptor::OptimizerAdaptor;
use burn::tensor::backend::AutodiffBackend;
use image::RgbImage;
use tempfile::tempdir;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

type Backend = NdArray<f32>;
type ADBackend = Autodiff<Backend>;
type Optim = OptimizerAdaptor<
    AdamW<<ADBackend as AutodiffBackend>::InnerBackend>,
    TinyDet<ADBackend>,
    ADBackend,
>;

#[test]
fn train_harness_runs_multi_step_batch_gt1() -> Result<()> {
    // Build a tiny synthetic dataset: two samples with identical boxes/images.
    let temp = tempdir()?;
    let run_dir: PathBuf = temp.path().join("run");
    let labels_dir = run_dir.join("labels");
    fs::create_dir_all(&labels_dir)?;

    // Write a tiny 8x8 RGB image.
    let img_path = run_dir.join("img1.png");
    let mut img = RgbImage::new(8, 8);
    for p in img.pixels_mut() {
        *p = image::Rgb([128, 64, 32]);
    }
    img.save(&img_path)?;

    // Write label JSON referencing the image with one normalized box.
    let label_path = labels_dir.join("img1.json");
    let json = serde_json::json!({
        "frame_id": 1,
        "image": "img1.png",
        "image_present": true,
        "polyp_labels": [
            { "bbox_norm": [0.25, 0.25, 0.75, 0.75], "bbox_px": [2.0, 2.0, 6.0, 6.0] }
        ]
    });
    let mut f = fs::File::create(&label_path)?;
    f.write_all(json.to_string().as_bytes())?;

    // Two identical entries.
    let cfg = DatasetConfig {
        target_size: Some((8, 8)),
        max_boxes: 2,
        shuffle: false,
        seed: Some(123),
        ..Default::default()
    };
    let indices = vec![
        SampleIndex {
            label_path: label_path.clone(),
            run_dir: run_dir.clone(),
        };
        4
    ];

    // Build model/optim/scheduler.
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = TinyDet::<ADBackend>::new(TinyDetConfig::default(), &device);
    let optim = AdamWConfig::new();
    let mut optim: Optim = OptimizerAdaptor::from(optim.init());
    let mut sched = LinearLrSchedulerConfig::new(1e-3, 1e-3, 2).init();

    // Two steps over batch_size=2.
    let mut loader = BatchIter::from_indices(indices, cfg).expect("loader");
    let mut steps = 0;
    while let Some(batch) = loader
        .next_batch::<ADBackend>(2, &device)
        .expect("batch")
    {
        steps += 1;
        let (obj, boxes) = model.forward(batch.images.clone());
        // Build simple targets: one box per image in the center.
        let batch_len = batch.images.dims()[0];
        let hw = obj.dims()[2] * obj.dims()[3];
        let mut tgt_obj = vec![0.0f32; batch_len * hw];
        let mut tgt_boxes = vec![0.0f32; batch_len * 4 * hw];
        let mut tgt_mask = vec![0.0f32; batch_len * 4 * hw];
        for b in 0..batch_len {
            let idx = b * hw;
            tgt_obj[idx] = 1.0;
            let base = b * 4 * hw;
            tgt_boxes[base] = 0.4;
            tgt_boxes[base + hw] = 0.4;
            tgt_boxes[base + 2 * hw] = 0.6;
            tgt_boxes[base + 3 * hw] = 0.6;
            tgt_mask[base] = 1.0;
            tgt_mask[base + hw] = 1.0;
            tgt_mask[base + 2 * hw] = 1.0;
            tgt_mask[base + 3 * hw] = 1.0;
        }
        let tgt_obj_t = burn::tensor::Tensor::<ADBackend, 1>::from_floats(tgt_obj.as_slice(), &device)
            .reshape([batch_len, 1, obj.dims()[2], obj.dims()[3]]);
        let tgt_boxes_t = burn::tensor::Tensor::<ADBackend, 1>::from_floats(tgt_boxes.as_slice(), &device)
            .reshape([batch_len, 4, obj.dims()[2], obj.dims()[3]]);
        let tgt_mask_t = burn::tensor::Tensor::<ADBackend, 1>::from_floats(tgt_mask.as_slice(), &device)
            .reshape([batch_len, 4, obj.dims()[2], obj.dims()[3]]);

        let loss = model.loss(
            obj.clone(),
            boxes.clone(),
            tgt_obj_t,
            tgt_boxes_t,
            tgt_mask_t,
            &device,
        );
        let grads = GradientsParams::from_grads(loss.backward(), &model);
        let lr = burn::lr_scheduler::LrScheduler::<ADBackend>::step(&mut sched);
        model = optim.step(lr, model, grads);
        if steps >= 2 {
            break;
        }
    }
    assert_eq!(steps, 2);
    Ok(())
}
