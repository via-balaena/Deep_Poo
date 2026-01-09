# Feature Flags
**Why**: Know the default feature posture before you enable anything heavy.
**How it fits**: Feature gates shape build size, runtime backends, and tooling.
**Learn more**: See [Build & Run](build_and_run.md) for command patterns.

## Workspace-wide themes
High-level defaults and stack-wide feature expectations.

| Theme | Details |
| --- | --- |
| Default backend | training/inference/models enable `backend-ndarray` by default; GPU/WGPU is opt-in via `backend-wgpu`. |
| Model variants | `tinydet`/`bigdet` across models/training/inference. |
| Tools | `tui`, `scheduler`, `gpu_nvidia` gate app-specific/heavy bins in cortenforge-tools. |
| Umbrella crate | `cortenforge` re-exports features to enable stacks (sim-core, vision-core/runtime, training/inference). |
| Burn-core | 0.14.1 fixes the prior bincode publish break; no patch required. |

## Per-crate highlights
Quick reference for feature toggles that matter most per crate.

| Crate | Highlights |
| --- | --- |
| models | `tinydet` (default), `bigdet`. |
| training | `backend-ndarray` (default), `backend-wgpu`, `tinydet` (default), `bigdet`. |
| inference | `backend-ndarray` (default), `backend-wgpu`, `tinydet` (default), `bigdet`. |
| cortenforge (umbrella) | Features map to member crates (sim-core, vision-core/runtime, models, training, inference, capture-utils, cli-support, burn-dataset); `burn-runtime`/`burn-wgpu` stacks wire burn deps. |
| cortenforge-tools | `tui`, `scheduler`, `gpu_nvidia`; defaults are lean (no extra features). |
| cli_support | Optional `bevy`/`bevy-resource` for resource integration. |
| burn_dataset | `burn-runtime` wires burn + rayon/memmap2/crossbeam; `burn-ndarray`/`burn-wgpu` optional. |

## Hygiene guidance
Rules of thumb for keeping features and builds predictable.
1) Keep defaults light (NdArray, no heavy GPU deps) to keep CI fast.
2) Gate app-specific or heavy tooling behind explicit features; avoid enabling by default.
3) When adding new features, document what they gate and ensure clippy/tests run with and without them as appropriate.
4) Avoid adding patch overrides unless needed for local development.
