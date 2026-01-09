# cortenforge-tools (shared): Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| RunManifestSummary | struct | Summary info for a run |
| RunInfo | struct | Info about a run (path, metadata) |
| ServiceError | enum | Errors from service helpers |
| ServiceCommand | struct | Command wrapper for spawning services |
| DatagenOptions | struct | Options to build datagen command |
| TrainOptions | struct | Options to build train command |
| list_runs | fn | List runs under a root |
| spawn | fn | Spawn a service command |
| datagen_command | fn | Build datagen command |
| train_command | fn | Build train command |
| read_metrics | fn | Read metrics from a path |
| read_log_tail | fn | Tail logs from a path |
| is_process_running | fn | Check if a PID is running |
| read_status | fn | Read status JSON from a path |
| draw_rect / normalize_box | re-export | Overlay helpers from vision_core |
| generate_overlays / prune_run / JsonRecorder | re-export | Recorder helpers from capture_utils |
| WarehouseStore | enum | Warehouse store target (local/object store) |
| ModelKind | enum | Model selection for command builder |
| CmdConfig<'a> | struct | Config for warehouse command builder |
| DEFAULT_CONFIG | const | Default command config |
| Shell | enum | Shell target for command rendering |
| build_command | fn | Build warehouse command string |
| Modules (pub mod) | module | overlay, recorder, services, warehouse_commands (builder/common) |
