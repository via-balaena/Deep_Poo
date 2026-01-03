# colon_sim_tools

CLI utilities packaged in the `tools` crate:

- Core (always available): `overlay_labels`, `prune_empty`, `warehouse_etl`, `warehouse_export`, `warehouse_cmd`.
- Feature-gated:
  - `tui` (enable `--features tui`): requires `crossterm`/`ratatui`.
  - `datagen_scheduler` (enable `--features scheduler`): requires `sysinfo`.
  - `gpu_nvidia` (optional): pulls in `nvml-wrapper` for NVML-based telemetry (used by datagen_scheduler).

Shared deps:
- `data_contracts` for capture/manifest schemas.
- `vision_core` for overlay helpers (used by `overlay_labels`).

Usage examples:
- `cargo run -p colon_sim_tools --bin prune_empty -- --input ... --output ...`
- `cargo run -p colon_sim_tools --features tui --bin tui -- --help`
- `cargo run -p colon_sim_tools --features scheduler --bin datagen_scheduler -- --help`

Quick sanity check:
- `cargo check -p colon_sim_tools --all-features`
