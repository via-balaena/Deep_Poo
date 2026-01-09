# cli_support: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| ThresholdOpts | struct | CLI thresholds for inference/detection |
| WeightsOpts | struct | CLI weights paths/options |
| CaptureOutputArgs | struct | CLI args for capture outputs |
| CaptureOutputOpts | struct | Output options for capture |
| WarehouseOutputArgs | struct | CLI args for warehouse outputs |
| WarehouseOutputOpts | struct | Output options for warehouse |
| WgpuEnvHints | struct | Hints for WGPU environment setup |
| SeedState | struct | Seed state container |
| resolve_seed | fn | Resolve seed from CLI input |
| Modules (pub mod) | module | common, seed |

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
- Docs.rs: https://docs.rs/cortenforge-cli-support
