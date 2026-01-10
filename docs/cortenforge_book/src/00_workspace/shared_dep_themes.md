## Shared dependency themes
**Why**: A quick sense of the external libraries the workspace leans on.
**How it fits**: Helps you understand upgrade risk and shared constraints.
**Learn more**: See [Workspace Metadata](workspace_metadata.md) for publish posture.
High-level overview of the shared third-party dependencies across the workspace. Use this as a
map, not a full inventory.

| Theme | Details |
| --- | --- |
| Burn | Burn 0.14 (burn-core 0.14.1) across models/training/inference; burn-ndarray and burn-wgpu backends. |
| Errors/serde | serde/serde_json, anyhow/thiserror for errors. |
| Serialization | bincode 2.0.1 for checkpoint/weight formats (3.0.0 is a stub on crates.io; revisit when real). |
| Bevy/Rapier | bevy for sim_core/vision_runtime and some tools; bevy_rapier3d for physics integration in sim_core. |
| CLI | clap for CLIs; cli_support reused by tools. |
| Data/IO | image/png, rayon for capture/tools; arrow/parquet in tools. |
| Hashing | sha2 for content hashing in capture/tools. |
| Temp/FS | tempfile for tests and local staging. |

## Workspace dependency policy
- Shared third-party deps should be centralized in root `[workspace.dependencies]`.
- Member crates should use `workspace = true` and keep crate-specific features in place.
- Optional deps remain optional in member crates.
- Single-use deps may stay per-crate to keep the workspace list focused.
- **Bevy is the exception today**: crates are split on default features, so it stays per-crate
  until the feature set is unified. After that, centralize Bevy.
