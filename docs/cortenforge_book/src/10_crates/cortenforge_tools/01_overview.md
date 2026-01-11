# cortenforge-tools (shared): Overview
Quick read: What this crate does and where it fits.

## Problem statement
Bundle tooling bins and helpers for capture/overlay/ETL/export and simple inference, shared across apps. Bins are now config-driven and app-agnostic; app-specific behavior lives in app repos.

## Scope
- Bins: overlay_labels, prune_empty, warehouse_etl/export/cmd, single_infer, gpu_probe (shared-ish); app-facing bins gated by features (datagen_scheduler, tui, datagen).
- Shared helpers: CLI services and warehouse commands in `services` / `warehouse_commands` (only live here today).
- Uses substrate crates: capture_utils, data_contracts, vision_core, inference/models, cli_support, burn_dataset.

## Configuration
Tools read `cortenforge-tools.toml` from the repo root by default (or `CORTENFORGE_TOOLS_CONFIG` for a custom path). Use it to set binary names, paths, and command templates so tools work across app repos without hardcoded assumptions.

```toml
sim_bin = "sim_view"
assets_root = "assets"
captures_root = "assets/datasets/captures"

[warehouse]
train_template = "cargo train_hp --model ${MODEL} --batch-size ${BATCH}"
```

Template placeholders:
- `${MODEL}`, `${BATCH}`, `${LOG_EVERY}`, `${EXTRA_ARGS}`
- `${MANIFEST}`, `${STORE}`, `${WGPU_BACKEND}`, `${WGPU_ADAPTER}`

Config precedence: CLI flags > config file > environment > defaults.

## Non-goals
- No app-specific world/config baked into shared bins; app repos handle app-specific flows.
- No recorder/meta/world definitions; uses shared schemas/helpers.

## Who should use it
- Consumers needing CLI tooling for captures/warehouse/inference without app-specific logic.
- Contributors trimming/splitting the crate: move app-only bins out, fold shared helpers into existing crates (cli_support/capture_utils) and keep a thin bin crate.
