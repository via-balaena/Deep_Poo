# common (cli_support)

## Responsibility
- Shared CLI option structs and helpers for capture, warehouse, inference, and WGPU hints.
- Keep CLI-parsed structs (`Args`-derived) separate from internal option structs.

## Key items
- `ThresholdOpts`: objectness/IoU thresholds for detectors.
- `WeightsOpts`: optional path to detector weights.
- `CaptureOutputArgs` (`clap::Args`): output/prune flags for capture tooling.
- `CaptureOutputOpts`: internal config; `resolve_prune_output_root` computes default `<output_root>_filtered`.
- `WarehouseOutputArgs`/`WarehouseOutputOpts`: output roots for warehouse tooling.
- `WgpuEnvHints`: optional strings for backend/adapter/power/log hints.

## Invariants / Gotchas
- `CaptureOutputOpts::resolve_prune_output_root` rewrites the file name of `output_root`; ensure callers pass a path (not dir ending with `/`).
- Thresholds/weights are thin wrappers; validation (e.g., range checks) is not performed here.
- WGPU hints are inert; consumers must apply them to env/logging.

## Cross-module deps
- Used by CLI binaries across the workspace (capture tools, warehouse commands).
- Intended to be feature-light; purely data holders with minimal logic.
