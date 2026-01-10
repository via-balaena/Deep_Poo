# Glossary
**Why**: Translate shared terms without hunting across pages.
**How it fits**: Use this when a term feels unfamiliar or overloaded.
**Learn more**: Start with [Workspace Overview](overview.md).

| Term | Meaning |
| --- | --- |
| CortenForge | Substrate of shared crates (sim/capture/vision/training/inference/tools). |
| App repo | Domain apps (e.g., colon_sim) that consume this substrate. |
| SimRunMode / ModeSet | Runtime mode gating (Common/SimDatagen/Inference). |
| Recorder | Capture pipeline (meta + world state + sinks). Defaults to JSON sink in capture_utils. |
| Warehouse | Tensor artifacts produced by ETL (`warehouse_etl`), consumed by training. |
| TinyDet / BigDet | Model variants defined in `models`. |
| Burn patch | Previous workaround for `burn-core 0.14.0`; fixed in `burn-core 0.19.1` (no patch required). |
| NdArray / WGPU | Default CPU backend vs optional GPU backend gated by features. |
| cortenforge-tools | Tooling package (crate `cortenforge_tools`); planned split of app-specific vs shared utilities. |
| CLI support | Shared argument parsing/helpers in `cli_support`. |
| ETL | Transforming captures to warehouse shards/manifests. |
