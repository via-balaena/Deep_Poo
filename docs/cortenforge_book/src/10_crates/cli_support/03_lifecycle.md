# cli_support: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
- Define CLI structs in tools/apps using provided opts:
  ```rust,ignore
  #[derive(clap::Parser)]
  struct Args {
      #[clap(flatten)]
      capture: CaptureOutputArgs,
      #[clap(flatten)]
      warehouse: WarehouseOutputArgs,
      #[clap(long)]
      seed: Option<u64>,
  }
  let args = Args::parse();
  let capture_opts: CaptureOutputOpts = (&args.capture).into();
  let warehouse_opts: WarehouseOutputOpts = (&args.warehouse).into();
  ```
- Resolve seeds:
  ```rust,ignore
  let seed = resolve_seed(args.seed);
  ```
- Pass parsed options into tooling/runtime setup.

## Execution flow
- CLI derives parse args; `flatten`ed structs supply common options.
- Optional Bevy integration if `bevy-resource` feature is used to insert resources.
- Caller uses parsed values to configure capture/warehouse/weights/thresholds.

## Notes
- Stateless; lifecycle ends after parsing/using args. Features enable Bevy resource adapters if needed.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
