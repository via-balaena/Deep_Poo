# Tools Refactor Checklist (App-Agnostic)

Goal: keep the tooling in CortenForge but make all app-specific assumptions configurable so the tools can target any app repo.

## Prompts to complete (in order)
- [x] Inventory app-specific assumptions.
  - [x] List hardcoded binaries (`sim_view`, `train`) and where they are called.
  - [x] List hardcoded paths (`assets/datasets/...`) in bins and helpers.
  - [x] List app-specific CLI flags and presets (e.g., `--mode datagen`, `cargo train_hp`).
  - [x] List UI/log/metrics paths assumed by the TUI and scheduler.
  - Inventory findings:
    - `tools/src/services.rs:125-152`: `sim_view` binary name + `--mode datagen` + datagen flags are hardcoded.
    - `tools/src/services.rs:167-195`: `train` binary name + training flags are hardcoded.
    - `tools/src/bin/datagen.rs:30-64`: `sim_view` binary name + `--mode datagen` + specific CLI flags hardcoded.
    - `tools/src/bin/datagen_scheduler.rs:41-56`: default output root `assets/datasets/captures`.
    - `tools/src/bin/datagen_scheduler.rs:185-193`: default prune suffix `captures_filtered`.
    - `tools/src/bin/datagen_scheduler.rs:196-207`: `gpu_macos_helper` probe assumes helper binary name.
    - `tools/src/bin/datagen_scheduler.rs:210-245`: `nvidia-smi`, `rocm-smi`, `radeontop`, `intel_gpu_top` hardcoded probes.
    - `tools/src/bin/tui.rs:119-153`: paths `assets/datasets/captures` and `assets/datasets/captures_filtered`.
    - `tools/src/bin/tui.rs:150-182`: log/metrics/status paths `logs/train_status.json`, `checkpoints/metrics.jsonl`, `logs/train.log`.
    - `tools/src/bin/tui.rs:228-233`: status paths `logs/train_hp_status.json`, `logs/train_status.json`.
    - `tools/src/bin/tui.rs:253`: window title `Deep Poo` hardcoded.
    - `tools/src/bin/prune_empty.rs:15-19`: default input/output roots under `assets/datasets`.
    - `tools/src/bin/overlay_labels.rs:11-18`: default run root `assets/datasets/captures`.
    - `tools/src/bin/warehouse_etl.rs:22-26`: default input root `assets/datasets/captures_filtered`.
    - `tools/src/bin/warehouse_cmd.rs:154-157`: default manifest `output_root/manifest.json`.
    - `tools/src/warehouse_commands/builder.rs:42`: `cargo train_hp` command hardcoded.
    - `tools/src/warehouse_commands/common.rs:111-118`: default manifest path `artifacts/tensor_warehouse/v<version>/manifest.json`.
    - `tools/Cargo.toml:7-18`: package/lib name set to `cortenforge-tools` / `cortenforge-tools` (rename complete).

- [x] Define the shared config surface.
  - [x] Choose config format (TOML) and default location (`cortenforge-tools.toml` at repo root).
  - [x] Define `ToolConfig` fields (bins, paths, args, templates).
    - [x] `sim_bin` (default `sim_view`)
    - [x] `train_bin` (default `train`)
    - [x] `assets_root` (default `assets`)
    - [x] `captures_root` (default `${assets_root}/datasets/captures`)
    - [x] `captures_filtered_root` (default `${assets_root}/datasets/captures_filtered`)
    - [x] `warehouse_manifest` (default `${assets_root}/warehouse/manifest.json` or CLI override)
    - [x] `logs_root` (default `logs`)
    - [x] `metrics_path` (default `${logs_root}/metrics.jsonl` or `${assets_root}/checkpoints/metrics.jsonl`)
    - [x] `train_log_path` (default `${logs_root}/train.log`)
    - [x] `train_status_paths` (default `logs/train_hp_status.json`, `logs/train_status.json`)
    - [x] `[datagen].args` (list)
    - [x] `[training].args` (list)
    - [x] `[warehouse].train_template` (string template)
    - [x] `[ui].title` (default `CortenForge Tools`)
  - [x] Decide precedence order (CLI > config > env > defaults).
  - [x] Map CLI overrides to config fields for each bin (document which CLI flags override which fields).
    - `datagen`:
      - `--output-root` → `captures_root`
      - `--prune-output-root` → `captures_filtered_root`
      - `--seed`, `--max-frames`, `--headless`, `--prune-empty` → runtime options (no config mapping)
    - `datagen_scheduler`:
      - `--output-root` → `captures_root`
      - `--prune-output-root` → `captures_filtered_root`
      - `--count`, `--concurrency`, `--max-cpu`, `--min-free-mem-mb`, `--poll-secs` → runtime options
      - `--max-gpu`, `--max-gpu-mem-mb` → runtime options (only when GPU probes are available)
    - `tui`:
      - no CLI overrides yet (future: `--captures-root`, `--log-path`, `--metrics-path`, `--title`)
    - `overlay_labels`:
      - positional `run_dir` → `captures_root`
      - positional `out_dir` → derived output root (no config mapping)
    - `prune_empty`:
      - `--input` → `captures_root`
      - `--output` → `captures_filtered_root`
    - `warehouse_etl`:
      - `--input-root` → `captures_filtered_root`
      - `--output` (WarehouseOutputArgs) → output root only (no config mapping)
    - `warehouse_export`:
      - `--manifest` → `warehouse_manifest`
      - `--out` → output parquet path (no config mapping)
    - `warehouse_cmd`:
      - `--manifest` → `warehouse_manifest`
      - `--backend`, `--adapter`, `--batch-size`, `--log-every`, `--model` → runtime options
      - `--extra-args` → `${EXTRA_ARGS}` placeholder
  - [x] Define validation rules and error messages for invalid config values.
  - [x] Decide path resolution rules (relative to repo root vs cwd) and env var expansion.
    - Relative paths are interpreted from the current working directory.
    - `~` expands to `$HOME`; `${VAR}` expands to environment variables.
  - [x] Decide whether `ToolConfig::load()` should cache or re-read per call.
    - No caching; load per call to reflect config changes during runs.

- [x] Refactor services to be generic.
  - [x] Update `services::datagen_command` to accept `ToolConfig`.
  - [x] Update `services::train_command` to accept `ToolConfig`.
  - [x] Remove hardcoded defaults from services.

- [x] Decouple `warehouse_cmd`.
  - [x] Replace `cargo train_hp` with a templated command.
  - [x] Support env template placeholders (e.g., `${MANIFEST}`, `${MODEL}`).

- [x] Make bins config-driven.
  - [x] `datagen`: load config + apply CLI overrides.
  - [x] `datagen_scheduler`: load config + optional GPU stats.
  - [x] `tui`: load config + use config paths/commands.
  - [x] `warehouse_cmd`: load config + generate command.
  - [x] `overlay_labels`: use config defaults for run root/output.
  - [x] `prune_empty`: use config defaults for input/output roots.
  - [x] `warehouse_etl`: use config defaults for input/output roots.
  - [x] `warehouse_export`: use config defaults for manifest/output.

- [x] Make GPU helpers optional.
  - [x] Keep `gpu_macos_helper` as a standalone probe.
  - [x] Allow scheduler to run without GPU stats.

- [x] Migration + docs.
  - [x] Ensure existing defaults still work without a config.
    - [x] Quick sanity: run one tool with no config (e.g., `cargo run -p cortenforge-tools --bin overlay_labels`) and confirm it uses defaults.
      - Result: executable runs and uses defaults; missing data path is expected until captures exist.
  - [x] Add a sample `cortenforge-tools.toml` at repo root and document it.
  - [x] Update tool docs to explain config usage.
  - [x] Add a brief deprecation note for hardcoded defaults (when to remove them).
  - [x] Add a smoke test or minimal check that loads a config and runs a command build.
  - [x] Plan a neutral crate name (rename `cortenforge-tools` after refactor to avoid app branding).
    - [x] Decision: rename to `cortenforge-tools` (crate name `cortenforge-tools`).

## Example config (draft)
```toml
sim_bin = "sim_view"
train_bin = "train"
assets_root = "assets"
captures_root = "assets/datasets/captures"
captures_filtered_root = "assets/datasets/captures_filtered"
warehouse_manifest = "assets/warehouse/manifest.json"

[datagen]
args = ["--mode", "datagen"]

[training]
args = ["--batch-size", "16", "--epochs", "10"]

[warehouse]
train_template = "cargo train_hp --model ${MODEL} --batch-size ${BATCH} --log-every ${LOG_EVERY} ${EXTRA_ARGS}"
```

## Open questions
- Preferred config format (TOML vs JSON)?
- Default config location (repo root, `assets/`, `~/.config/`)?
- Do we want a `cortenforge tools config` generator?
