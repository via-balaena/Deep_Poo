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
- App-facing bins (`datagen`, `datagen_scheduler`, `tui`) are gated by features and remain config-driven; app repos supply any app-specific flows. `gpu_probe` is shared.

## Notes
- Shared helpers (services/warehouse_commands) live here for now; keep the crate lean and config-driven while app repos own app-specific logic.
