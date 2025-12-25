# Datagen Scheduler (GPU-aware) — How-to

Lightweight job runner that staggers datagen launches based on CPU/RAM/GPU headroom, with pruning enabled by default.

## What it does
- Auto-detects GPU vendor (NVIDIA/AMD/Intel/macOS helper) and samples utilization (+ VRAM where available).
- Enforces CPU %, free RAM, and optional GPU % / GPU mem thresholds before launching each datagen run.
- Launches `sim_view --mode datagen --headless --prune-empty --prune-output-root <…>` so filtered runs are written.
- Never hard-fails on missing probes: if a GPU probe/tool is unavailable, it logs and falls back to CPU/RAM checks.

## Build & run
- Default (CPU/RAM + auto-probes):
  ```sh
  cargo run --release --features scheduler --bin datagen_scheduler -- \
    --count 4 --concurrency 2 --max-gpu 80 --max-gpu-mem-mb 8000
  ```
- With NVML (NVIDIA richer stats):
  ```sh
  cargo run --release --features "scheduler,gpu_nvidia" --bin datagen_scheduler -- \
    --count 4 --concurrency 2 --max-gpu 80 --max-gpu-mem-mb 8000
  ```
- macOS helper: built and used by default on macOS; no extra flag needed.

Options:
- `--count`: how many runs to launch.
- `--concurrency`: max concurrent runs.
- `--max-cpu`: max global CPU% to allow starting a run (default 85).
- `--min-free-mem-mb`: minimum free RAM to start a run (default 2048).
- `--max-gpu` / `--max-gpu-mem-mb`: optional GPU thresholds; enforced only if a probe returns stats.
- `--output-root`, `--prune-output-root`, `--seed`, `--max-frames`, `--headless`: passed through to datagen.

## Probes and requirements
- NVIDIA: `nvidia-smi` (default) or NVML via `gpu_nvidia` feature.
- AMD: `rocm-smi` (JSON preferred) or `radeontop` present.
- Intel (Linux): `intel_gpu_top` with `--json`.
- macOS: `gpu_macos_helper` binary (bundled); uses `powermetrics` + Metal VRAM stats.
- If a tool is missing or parsing fails, GPU gating is skipped with a log; CPU/RAM gating still applies.

## Current behavior and limits
- Utilization: returned for all vendors when tools respond.
- VRAM: NVIDIA (smi/NVML), AMD (rocm-smi meminfo), macOS helper (Metal), Intel best-effort from `intel_gpu_top`.
- No per-GPU selection; first device is used where applicable.
- No smoothing/logging of samples yet; thresholds are checked on each poll.

## Future improvements (optional)
- Per-GPU selection and multi-GPU awareness.
- Smoothing/averaging samples before gating to avoid transient spikes.
- CSV/JSON logging of samples for tuning thresholds.
