# cortenforge-tools (shared): Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage (shared bins)
- Run overlay/prune:
  ```bash
  cargo run -p cortenforge-tools --bin overlay_labels -- --run <run_dir>
  cargo run -p cortenforge-tools --bin prune_empty -- --input <run_dir> --output <out_root>
  ```
- ETL/export/cmd:
  ```bash
  cargo run -p cortenforge-tools --bin warehouse_etl -- --run <run_dir> --out <warehouse_root>
  cargo run -p cortenforge-tools --bin warehouse_cmd -- --manifest <manifest> --shell bash
  cargo run -p cortenforge-tools --bin warehouse_export -- --warehouse <warehouse_root>
  ```
- Single inference:
  ```bash
  cargo run -p cortenforge-tools --bin single_infer -- --image <path> --weights <checkpoint>
  ```

## Execution flow (shared bins)
- Parse CLI args using cli_support helpers.
- Operate on capture/warehouse artifacts using capture_utils, data_contracts, burn_dataset, inference/models as needed.
- App-facing bins (`datagen`, `datagen_scheduler`, `tui`, `gpu_macos_helper`) are gated by features; planned to move to app repo.

## Notes
- Shared helpers (services/warehouse_commands) currently live here; plan to fold into cli_support/capture_utils and keep a thin bin crate or move app-specific bins out.

