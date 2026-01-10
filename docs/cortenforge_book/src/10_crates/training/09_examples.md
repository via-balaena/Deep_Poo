# Examples (training)
Quick read: Minimal examples you can adapt safely.

## 1) Load dataset samples
```rust,ignore
use training::{DatasetConfig};

fn main() -> anyhow::Result<()> {
    let cfg = DatasetConfig {
        root: "assets/datasets/captures/run_00001".into(),
        labels_subdir: "labels".into(),
        images_subdir: "images".into(),
    };
    let samples = cfg.load()?;
    println!("loaded {} samples", samples.len());
    Ok(())
}
```

## 2) Collate a batch (NdArray backend)
```rust,ignore
use training::{DatasetConfig, RunSample, collate, TrainBackend};

fn main() -> anyhow::Result<()> {
    let cfg = DatasetConfig {
        root: "assets/datasets/captures/run_00001".into(),
        labels_subdir: "labels".into(),
        images_subdir: "images".into(),
    };
    let samples = cfg.load()?;
    let batch = collate::<TrainBackend>(&samples[..2], 16)?;
    println!("batch images shape = {:?}", batch.images.dims());
    Ok(())
}
```

## 3) Run training (high level)
```rust,ignore
use clap::Parser;
use training::util::{run_train, TrainArgs};

fn main() -> anyhow::Result<()> {
    let args = TrainArgs::parse();
    run_train(args)?;
    Ok(())
}
```

## Links
- Source: `crates/training/src/dataset.rs`
- Source: `crates/training/src/util.rs`
