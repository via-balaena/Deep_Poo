# Examples (burn_dataset)

## 1) Index and summarize a captures root
```rust,ignore
use burn_dataset::{index_runs, summarize_with_thresholds, ValidationThresholds};
use std::path::Path;

fn main() -> burn_dataset::DatasetResult<()> {
    let indices = index_runs(Path::new("assets/datasets/captures"))?;
    let report = summarize_with_thresholds(&indices, &ValidationThresholds::from_env())?;
    println!("summary outcome = {:?}", report.outcome);
    Ok(())
}
```

## 2) Load a run eagerly
```rust,ignore
use burn_dataset::load_run_dataset;
use std::path::Path;

fn main() -> burn_dataset::DatasetResult<()> {
    let samples = load_run_dataset(Path::new("assets/datasets/captures/run_00001"))?;
    println!("loaded {} samples", samples.len());
    Ok(())
}
```

## 3) Build train/val iterators (burn_runtime feature)
```rust,ignore
fn main() -> burn_dataset::DatasetResult<()> {
    #[cfg(feature = "burn_runtime")]
    {
        use burn_dataset::{build_train_val_iters, DatasetConfig, BatchIter};
        let train_cfg = DatasetConfig::default();
        let (mut train, mut val) = build_train_val_iters(
            std::path::Path::new("assets/datasets/captures"),
            0.2,
            train_cfg,
            None,
        )?;
        // Example: pull one batch on NdArray backend
        let device = <burn_ndarray::NdArray<f32> as burn::tensor::backend::Backend>::Device::default();
        if let Some(batch) = train.next_batch::<burn_ndarray::NdArray<f32>>(4, &device)? {
            println!("batch boxes shape = {:?}", batch.boxes.dims());
        }
    }
    Ok(())
}
```
