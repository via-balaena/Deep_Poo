# Tools crate

Contributor guide to `colon_sim_tools`: bins, shared helpers, feature flags, and how to add new commands.

## Binaries
- Core:
  - `overlay_labels`: render boxes onto PNGs for a capture run.
  - `prune_empty`: drop empty-label frames and write a filtered copy.
  - `warehouse_etl`: build tensor warehouse shards/manifests from captures.
  - `warehouse_export`: export warehouse data.
  - `warehouse_cmd`: emit a training command line tailored to backend/shell.
  - `single_infer`: run inference on a single image and write boxed output.
- Feature-gated:
  - `datagen_scheduler` (feature `scheduler`): schedule headless datagen runs.
  - `tui` (feature `tui`): interactive terminal UI.
  - `gpu_nvidia` (feature `gpu_nvidia`): NVML support for scheduler.

## Shared helpers
- Reuse `data_contracts` and `vision_core`/`capture_utils` for overlay/prune/recorder schema.
- Common CLI args imported from `colon_sim::cli`.
- Warehouse helpers live in `tools/src/services.rs` and `tools/src/warehouse_commands/`; prefer adding helpers there instead of duplicating logic in bins.

## Defaults and features
- Tools default to lean deps; heavy backends are behind features (see above).
- WGPU-heavy paths should be feature-gated; default to NdArray where possible for tests.

## Adding a tool
- Place the bin at `tools/src/bin/your_tool.rs`.
- Reuse existing CLI parsers from `colon_sim::cli` when applicable.
- Use `capture_utils`/`vision_core` helpers for overlays/prune/labels; keep schemas consistent with `data_contracts`.
- If the tool needs new shared logic, add it to `tools/src/services.rs` or `tools/src/warehouse_commands/` and import it.

## Testing tools
- Prefer fast, backend-light tests (NdArray); gate GPU/WGPU usage behind features.
- Add smoke tests under `tools/tests/` for new commands; reuse fixtures where possible.
