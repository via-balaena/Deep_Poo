# Examples (colon_sim_tools)

## 1) List runs and print summary
```rust,ignore
use colon_sim_tools::services;
use std::path::Path;

fn main() -> Result<(), services::ServiceError> {
    let runs = services::list_runs(Path::new("assets/datasets/captures"))?;
    for run in runs {
        println!(
            "{:?} labels={} images={} overlays={}",
            run.path, run.label_count, run.image_count, run.overlay_count
        );
    }
    Ok(())
}
```

## 2) Build a datagen command
```rust,ignore
use colon_sim_tools::services::{datagen_command, DatagenOptions};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let cmd = datagen_command(&DatagenOptions {
        output_root: PathBuf::from("assets/datasets/captures"),
        seed: Some(1234),
        max_frames: Some(100),
        headless: true,
        prune_empty: true,
        prune_output_root: None,
    })?;
    println!("{:?} {:?}", cmd.program, cmd.args);
    Ok(())
}
```

## 3) Build a warehouse command line
```rust,ignore
use colon_sim_tools::warehouse_commands::{builder, common};

fn main() {
    let cfg = common::DEFAULT_CONFIG
        .with_manifest("artifacts/tensor_warehouse/v1/manifest.json")
        .with_batch_size(16)
        .with_model(common::ModelKind::Tiny);
    let bash_cmd = builder::build_command(&cfg, builder::Shell::Bash);
    println!("bash: {}", bash_cmd);
}
```
