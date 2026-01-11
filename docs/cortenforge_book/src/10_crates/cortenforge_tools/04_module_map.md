# cortenforge-tools (shared): Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Exposes modules and re-exports capture_utils.
- `overlay`: Re-exports overlay helpers from vision_core.
- `recorder`: Re-exports recorder helpers from capture_utils (JsonRecorder, generate_overlays, prune_run).
- `services`: Shared CLI/service helpers.
  - Types: RunManifestSummary, RunInfo, ServiceCommand, DatagenOptions, TrainOptions.
  - Functions: list_runs, spawn, datagen_command, train_command, read_metrics/logs/status.
- `warehouse_commands`: Common/Builder submodules for warehouse command generation.
  - Types: WarehouseStore, ModelKind, CmdConfig, DEFAULT_CONFIG, Shell.
  - Functions: build_command.
- `bin/`: Binaries (overlay_labels, prune_empty, warehouse_etl/export/cmd, single_infer, gpu_probe).
  - App-gated bins: datagen, datagen_scheduler, tui.

Cross-module dependencies:
- services/warehouse_commands use cli_support and substrate crates.
- overlay/recorder wrap vision_core/capture_utils.
- bins wire these helpers into CLIs.
