use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use data_contracts::capture::CaptureMetadata;

#[derive(Parser, Debug)]
#[command(
    name = "prune_empty",
    about = "Copy runs while dropping frames with empty polyp_labels"
)]
struct Args {
    /// Input root containing run_* directories.
    #[arg(long, default_value = "assets/datasets/captures")]
    input: PathBuf,
    /// Output root where filtered runs will be written.
    #[arg(long, default_value = "assets/datasets/captures_filtered")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    fs::create_dir_all(&args.output).context("create output root")?;

    let mut runs_processed = 0usize;
    let mut frames_kept = 0usize;
    let mut frames_skipped = 0usize;

    for entry in fs::read_dir(&args.input).context("read input root")? {
        let entry = entry?;
        let run_path = entry.path();
        if !run_path.is_dir() {
            continue;
        }
        if run_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("run_"))
            != Some(true)
        {
            continue;
        }
        runs_processed += 1;
        let run_name = run_path
            .file_name()
            .map(|s| s.to_owned())
            .expect("run dir has a name");
        let out_run = args.output.join(run_name);
        fs::create_dir_all(out_run.join("labels")).context("create labels dir")?;
        fs::create_dir_all(out_run.join("images")).context("create images dir")?;
        fs::create_dir_all(out_run.join("overlays")).context("create overlays dir")?;

        // Copy manifest if present.
        let manifest_in = run_path.join("run_manifest.json");
        if manifest_in.exists() {
            let manifest_out = out_run.join("run_manifest.json");
            let _ = fs::copy(&manifest_in, &manifest_out);
        }

        let labels_dir = run_path.join("labels");
        for lbl in fs::read_dir(&labels_dir).context("read labels dir")? {
            let lbl = lbl?;
            if lbl.path().extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let raw =
                fs::read(lbl.path()).with_context(|| format!("read {}", lbl.path().display()))?;
            let meta: CaptureMetadata = serde_json::from_slice(&raw)
                .with_context(|| format!("parse {}", lbl.path().display()))?;
            if !meta.image_present || meta.polyp_labels.is_empty() {
                frames_skipped += 1;
                continue;
            }
            frames_kept += 1;
            // copy label
            let out_label = out_run.join("labels").join(lbl.file_name());
            fs::write(&out_label, &raw)?;
            // copy image
            let in_img = run_path.join(&meta.image);
            let out_img = out_run.join(&meta.image);
            if let Some(parent) = out_img.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&in_img, &out_img).with_context(|| {
                format!("copy image {} to {}", in_img.display(), out_img.display())
            })?;
            // copy overlay if present
            if let Some(fname) = Path::new(&meta.image).file_name() {
                let overlay_in = run_path.join("overlays").join(fname);
                if overlay_in.exists() {
                    let overlay_out = out_run.join("overlays").join(fname);
                    fs::copy(&overlay_in, &overlay_out).ok();
                }
            }
        }
    }

    println!(
        "Prune complete: runs processed {}, frames kept {}, frames skipped {}",
        runs_processed, frames_kept, frames_skipped
    );
    Ok(())
}
