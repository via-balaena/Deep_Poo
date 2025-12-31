# Overview

This page orients newcomers to the full data/training pipeline and gives a 10-minute quickstart. Replace the diagram placeholder once a visual is ready.

![Pipeline diagram — replace with final graphic](../media/pipeline_overview.png)

## Lifecycle at a glance

| Stage | What happens | Key artifacts/inputs | Primary commands |
| --- | --- | --- | --- |
| Ingest | Capture and prune raw data | Raw captures, filters | `cargo run --bin data_ingest …` |
| ETL | Validate, transform, shard | `manifest.json`, `shard_*.bin` | `cargo run -p colon_sim_tools --bin warehouse_etl …` |
| Warehouse | Store versioned tensors | `artifacts/tensor_warehouse/v<version>/` | n/a (consumed by training) |
| Train | Read warehouse, train model | checkpoints, logs | `cargo train_hp -- --tensor-warehouse …` |
| Evaluate | Analyze outputs/metrics | metrics, Parquet exports | `cargo run -p colon_sim_tools --bin warehouse_export …` |

## 10-minute quickstart

Follow these steps end-to-end; swap paths to match your machine.

1) Ingest or prepare filtered data roots (see ingestion chapter for capture/prune commands).  
2) Build the warehouse (see `reference/commands.md` for full snippet).
3) Train from the manifest (see `reference/commands.md` for full snippet). Set WGPU env vars if needed (see `reference/wgpu_envs.md`).
4) Inspect outputs: logs, checkpoints, and optional Parquet export:  
```bash
cargo run -p colon_sim_tools --bin warehouse_export -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json \
  --out logs/warehouse_summary.parquet
```
5) Troubleshoot with the FAQ if anything looks off, then iterate.

## Build your own sim on the platform
- Root crate is orchestration/CLI only (`src/cli/*`, `run_app`). Domain systems live in `apps/colon_sim` (reference app) or your own app crate.
- Core crates: `sim_core` (Bevy plumbing), `vision_core`/`vision_runtime` (detector interfaces + capture/inference plugins), `inference` (Burn-backed detector factory), `models` (TinyDet/BigDet).
- Tools: `colon_sim_tools` hosts CLIs (overlay/prune/warehouse/datagen/scheduler/tui, single_infer, gpu helper).
- Recorder: runs in the substrate; installs a default JSON sink. Apps provide recorder world-state updates and can inject custom sinks.
- App hook points: provide controls/autopilot via `SimHooks`; recorder metadata/world-state via `RecorderMetaProvider`/`RecorderWorldState`; add domain systems in your app crate (see `apps/colon_sim` as a reference).
- Sample app: `apps/hello_substrate` is a minimal demo showing how to wire your own app/plugin on the substrate without colon-specific systems.
- Migration note: see `MIGRATION.md` at repo root for a summary of the refactor (root orchestrator, app crates, tools move).
