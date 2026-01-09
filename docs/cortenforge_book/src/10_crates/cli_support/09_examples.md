# Examples (cli_support)
Quick read: Minimal examples you can adapt safely.

## 1) Parse capture args with clap and convert
```rust,ignore
use clap::Parser;
use cli_support::{CaptureOutputArgs, CaptureOutputOpts};

#[derive(Parser)]
struct Args {
    #[command(flatten)]
    capture: CaptureOutputArgs,
}

fn main() {
    let args = Args::parse();
    let opts: CaptureOutputOpts = (&args.capture).into();
    println!("output_root={:?}", opts.output_root);
}
```

## 2) Apply WGPU env hints
```rust,ignore
use cli_support::WgpuEnvHints;

fn main() {
    let hints = WgpuEnvHints {
        backend: Some("vulkan".into()),
        adapter_name: Some("NVIDIA".into()),
        power_pref: Some("high-performance".into()),
        rust_log: Some("info".into()),
    };
    for (k, v) in [
        ("WGPU_BACKEND", hints.backend),
        ("WGPU_ADAPTER_NAME", hints.adapter_name),
        ("WGPU_POWER_PREF", hints.power_pref),
        ("RUST_LOG", hints.rust_log),
    ] {
        if let Some(val) = v {
            std::env::set_var(k, val);
        }
    }
}
```

## 3) Resolve seed
```rust,ignore
use cli_support::resolve_seed;

fn main() {
    let seed = resolve_seed(None);
    println!("seed = {}", seed);
}
```

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
