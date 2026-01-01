# colon_sim (Deep Poo) — reference app

Reference domain app built on the CortenForge substrate. It holds the world/entities, HUD, controls/autopilot, and recorder world-state updates. Formerly the root focus of this repo (Deep Poo); now lives here as the canonical example app.

## Video
https://github.com/user-attachments/assets/cbf42edf-c61e-476c-b1e8-549b5f5b7580

## Controls
- `P` begin automated process
- `C` toggle camera between free-fly and probe POV
- `O` data run shortcut: enables autopilot + probe POV; auto-starts recording after a short delay and auto-stops at tunnel end (no return leg captured)
- `L` start/stop recording; HUD shows `REC :: on`
- `H` toggle HUD; `Esc` quits; `WASD` + mouse to fly; `Shift/Ctrl` speed up/down

## Binaries
- `sim_view`: interactive sim/datagen (uses this app)
- `inference_view`: live inference viewer
- `datagen` wrapper: headless capture (in `colon_sim_tools`, shells out to `sim_view`)
- Training bins live under `training/` (train/eval)
- CLI tools are in `tools/` (overlay_labels, prune_empty, warehouse_*, single_infer; feature-gated: tui, datagen_scheduler)

## Running
- Interactive sim: `cargo run --bin sim_view` (add `--release` for smoother playback)
- Headless capture: `cargo run -p colon_sim_tools --bin datagen` (add `--release` + build `sim_view` in the same profile)
- Inference view: `cargo run --bin inference_view`
- Tools: `cargo run -p colon_sim_tools --bin <tool> -- --help`

## Capturing + overlays
- Toggle to probe camera: press `C` until HUD shows `VISION :: cam=ON`.
- Start/stop recording: press `L`. Frames + JSON labels save under `assets/datasets/captures/run_<timestamp>/`.
- Data run shortcut: press `O` for autopilot + probe POV; recording auto-starts after ~8s and auto-stops at tunnel end.
- Render boxes onto PNGs (writes to `<run>/overlays` by default):
  ```bash
  cargo run -p colon_sim_tools --release --bin overlay_labels -- assets/datasets/captures/run_<timestamp>
  ```
  Or pick an output dir:
  ```bash
  cargo run -p colon_sim_tools --release --bin overlay_labels -- assets/datasets/captures/run_<timestamp> /tmp/overlays
  ```

## Polyp randomization / reproducibility
- Each run randomizes polyp count, spacing, size/shape, color, and twist.
- Seed control: pass `--seed <n>` to `sim_view`/`datagen` or set `POLYP_SEED=<number>` to reproduce a layout.
- The seed used for a run is stored in the capture JSON (`polyp_seed`) for traceability.

## Debug collider view
- Set `RAPIER_DEBUG_WIREFRAMES` in `src/lib.rs` to `true` to show collider wireframes (orange), or `false` to hide them. Rebuild/run after changing.

## Integration points (app ↔ substrate)
- Systems: wired via `AppSystemsPlugin`; bins add this alongside `sim_core::SimPlugin`/`SimRuntimePlugin` and vision/inference plugins as needed.
- Recorder: update `sim_core::recorder_meta::RecorderWorldState` (e.g., head_z, stop flag) via your app systems; the substrate recorder installs a default sink (`JsonRecorder`) but you can inject custom sinks.

Keep core crates detector-free and domain-agnostic; app crates supply the concrete systems and world logic.
