use clap::Parser;
use cli_support::common::CaptureOutputArgs;
use cli_support::seed::resolve_seed;
use cortenforge_tools::{services, ToolConfig};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Headless datagen launcher (wrapper over sim_view)"
)]
struct Args {
    /// Seed for the run (defaults to randomized in resolve_seed).
    #[arg(long)]
    seed: Option<u64>,
    /// Override max frames for the capture run.
    #[arg(long)]
    max_frames: Option<u32>,
    #[command(flatten)]
    capture: CaptureOutputArgs,
    /// Run headless (default true for datagen).
    #[arg(long, default_value_t = true)]
    headless: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let seed = resolve_seed(args.seed);
    let cfg = ToolConfig::load();
    let opts = services::DatagenOptions {
        output_root: args.capture.output_root,
        seed: Some(seed),
        max_frames: args.max_frames,
        headless: args.headless,
        prune_empty: args.capture.prune_empty,
        prune_output_root: args.capture.prune_output_root,
    };
    let status = services::datagen_command_with_config(&cfg, &opts)
        .and_then(|cmd| services::spawn(&cmd))?
        .wait()?;
    if !status.success() {
        anyhow::bail!("datagen exited with status {:?}", status);
    }
    Ok(())
}
