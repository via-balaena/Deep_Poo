# services (cortenforge-tools)

## Responsibility
- Helpers for running the simulation/training binaries and inspecting run artifacts/logs.
- Light manifest reader and metrics/log tail utilities used by CLI bins (scheduler/tui/etc.).

## Key items
- Data structs: `RunManifestSummary`, `RunInfo`, `ServiceCommand`, `DatagenOptions`, `TrainOptions`.
- Listing/counting: `list_runs(root)`, `count_artifacts` (labels/images/overlays).
- Process orchestration: `datagen_command`, `train_command`, `spawn`.
- Metrics/log utilities: `read_metrics`, `read_log_tail`.
- Environment helpers (feature `tui`/`scheduler`): `is_process_running`, `read_status`.

## Invariants / Gotchas
- `datagen_command`/`train_command` assume sibling binaries `sim_view` and `train` in the same `target` dir; `bin_path` derives path from current exe.
- `prune_empty` options are only appended when headless; ensure CLI matches expectations.
- Manifest parsing is permissive (optional fields); missing files yield empty counts.
- Process checks/log tailing are best-effort; no retries/backoff.

## Cross-module deps
- Invoked by CLI bins (`tui`, `datagen_scheduler`, etc.) under `tools/src/bin`.
- Pairs with `warehouse_commands` for downstream ETL/training flows.
