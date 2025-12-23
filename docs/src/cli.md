# CLI Usage

The simulator includes a Clap-based CLI to control run mode, seeding, outputs, and headless operation.

## Flags
- `--mode <sim|datagen>`: run interactively (`sim`, default) or headless data generation (`datagen`).
- `--seed <u64>`: optional seed override for reproducible polyp layouts; if omitted, a time-based seed is used.
- `--output-root <path>`: where run folders are written. Default: `assets/datasets/captures`.
- `--max-frames <N>`: optional capture frame cap for data runs; stops recording after N frames.
- `--headless`: hide the main window/offscreen rendering (useful for datagen).

## Binaries
- `sim_view`: interactive/visible sim (also used for visible datagen with `--mode datagen`).
- `datagen_headless`: headless data-gen runner.
- `overlay_labels`: draw bounding boxes onto captured frames.

### Runtime hotkeys (vision)
- `-`/`=`: decrease/increase objectness threshold.
- `[`/`]`: decrease/increase IoU threshold.
- `B`: toggle between Burn and heuristic detectors; HUD shows the active mode/box stats.
- Burn checkpoint: place model at `checkpoints/tinydet.bin` (runtime loads automatically). If missing or load fails, sim falls back to the heuristic detector and shows a fallback banner in the HUD.

## Examples
- Interactive sim (visible):
  ```bash
  cargo run --release --bin sim_view
  ```
- Data-gen, offscreen/headless, capped frames, custom output:
  ```bash
  cargo run --release --bin datagen_headless -- --seed 1234 --output-root /tmp/runs --max-frames 600
  ```
- Visible datagen (for debugging the pipeline):
  ```bash
  cargo run --release --bin sim_view -- --mode datagen --max-frames 500
  ```
- Overlay previously captured run:
  ```bash
  cargo run --release --bin overlay_labels -- assets/datasets/captures/run_1234567890123
  ```
