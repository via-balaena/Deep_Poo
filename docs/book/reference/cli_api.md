# CLI/API reference for tools

## warehouse_etl
- Purpose: build the tensor warehouse (manifest + shards) from filtered captures.
- Default run:
  ```bash
  cargo run -p colon_sim_tools --bin warehouse_etl
  ```
- Key flags (defaults):
  - `--input-root <path>` (default `assets/datasets/captures_filtered`)
  - `--output-root <path>` (default `artifacts/tensor_warehouse`)
  - `--target-size <HxW>`
  - `--resize-mode <letterbox|...>`
  - `--max-boxes <N>`
  - `--shard-samples <N>`
  - `--skip-empty` (if available)
  - `--code-version <sha>` (or `CODE_VERSION` env)

## warehouse_cmd
- Purpose: emit one-liner training env/command based on shell/adapter/backend.
- Default run (will emit defaults for bash + nvidia vulkan):
  ```bash
  cargo run -p colon_sim_tools --bin warehouse_cmd -- --shell sh --adapter nvidia
  ```
- Key flags:
  - `--shell <ps|sh>`
  - `--adapter <amd|nvidia>`
  - `--backend <dx12|vulkan>`
  - `--manifest <path>` (defaults to `<output-root>/manifest.json`, with `--output-root` defaulting to `artifacts/tensor_warehouse`)
  - `--output-root <path>` (used only to resolve default manifest; default `artifacts/tensor_warehouse`)
  - `--store <memory|mmap|stream>`
  - `--prefetch <N>`
  - `--batch-size <N>`
  - `--log-every <N>`
  - `--extra-args <string>`
  - Convenience subcommands (if kept): `amd-ps`, `amd-sh`, `nvidia-ps`, `nvidia-sh`
- Usage: `cargo run -p colon_sim_tools --bin warehouse_cmd -- ...`

## train / train_hp variants
- Purpose: train models using the tensor warehouse.
- Default run (NdArray backend unless you enable `burn_wgpu`):
  ```bash
  cargo run -p training --features burn_runtime --bin train -- \
    --manifest artifacts/tensor_warehouse/v<version>/manifest.json
  ```
- Key flags:
  - `--tensor-warehouse <path>`
  - `--warehouse-store <memory|mmap|stream>`
  - `--batch-size <N>`
  - `--epochs <N>`
  - `--log-every <N>`
  - `--status-file <path>`
  - Other model/task-specific flags (list here once finalized).

## inference_view
- Purpose: run the trained detector live and show boxes.
- Default run (inference mode):
  ```bash
  cargo run --bin inference_view
  ```
- Key flags (defaults):
  - `--output-root <path>` (recording output, if enabled; default `assets/datasets/captures`)
  - `--infer-obj-thresh <float>` (default `0.3`)
  - `--infer-iou-thresh <float>` (default `0.5`)
  - `--detector-weights <path>` (optional; Burn checkpoint; defaults to `checkpoints/tinydet.bin` if unset)
  - `--headless <bool>` (hide window)
  - `--max-frames <N>` (optional cap)
  - `--seed <u64>` (optional)
  - `--prune-empty` / `--prune-output-root` (datagen/capture prune options; default off, output root inferred as `<output_root>_filtered` when enabled)

## single_infer
- Purpose: run the detector on a single image and emit a boxed PNG.
- Default run:
  ```bash
  cargo run -p colon_sim_tools --bin single_infer -- --image path/to/image.png
  ```
- Key flags:
  - `--image <path>` (required)
  - `--out <path>` (optional; defaults to `<stem>_boxed.png` next to input)
  - `--infer-obj-thresh <float>` (default `0.3`)
  - `--infer-iou-thresh <float>` (default `0.5`)
- Notes: requires Burn features/weights to use the trained model; falls back to heuristic if weights are missing. Set WGPU envs if needed (`WGPU_BACKEND`, `WGPU_ADAPTER_NAME`, `WGPU_POWER_PREF`).

## capture/datagen (sim_view/datagen binaries)
- Purpose: interactive sim or headless datagen capture.
- Default runs:
  ```bash
  cargo run --bin sim_view
  cargo run -p colon_sim_tools --bin datagen
  ```
- Key flags (defaults):
  - `--mode <sim|datagen|inference>` (sim_view sets this; datagen forces `datagen`)
  - `--output-root <path>` (default `assets/datasets/captures`)
  - `--prune-empty` (default `false`)
  - `--prune-output-root <path>` (default `<output_root>_filtered` when prune is enabled)
  - `--max-frames <N>` (optional cap)
  - `--headless <bool>` (default `false` unless `datagen` bin forces it)

## Notes
- Keep CLI help in sync with docs; update here when flags change.
- Add examples per tool in their respective sections (Warehouse/Training).

## Tooling (runs under `colon_sim_tools`)
- `overlay_labels`: `cargo run -p colon_sim_tools --bin overlay_labels -- <run_dir> [<out_dir>]`
- `prune_empty`: `cargo run -p colon_sim_tools --bin prune_empty -- --input <path> --output <path>`
- `datagen_scheduler`: `cargo run -p colon_sim_tools --bin datagen_scheduler -- ...`
- `warehouse_etl`: `cargo run -p colon_sim_tools --bin warehouse_etl -- ...`
- `warehouse_export`: `cargo run -p colon_sim_tools --bin warehouse_export -- ...`
- `warehouse_cmd`: `cargo run -p colon_sim_tools --bin warehouse_cmd -- ...`
- `tui` (if kept): `cargo run -p colon_sim_tools --bin tui -- ...`
