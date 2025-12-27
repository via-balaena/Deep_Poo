## Warehouse Commands Refactor — Bite-Size Steps

- Step 1: Inspect current binaries (`tools/warehouse_commands/bin/*.rs`) and list shared vs. diverging config values (backend, adapter, shell). Confirm no hidden flags are used elsewhere.
  - Shared values: `manifest="artifacts/tensor_warehouse/v<version>/manifest.json"`, `store=Stream`, `prefetch=Some(8)`, `model=Big`, `batch_size=32`, `log_every=1`, `extra_args=""`, `WGPU_POWER_PREF` and `RUST_LOG` envs set identically.
  - Diverging values: `wgpu_backend` is `dx12` for the PowerShell binaries, `vulkan` for the Bash binaries; `wgpu_adapter` is either `AMD` or `NVIDIA` per binary; shell choice picks `ps_builder` vs `sh_builder`.
  - Hidden flags: none found beyond these four binaries; `CmdConfig` and builders are only referenced inside `tools/warehouse_commands`.
- Step 2: Sketch a target CLI (e.g., `warehouse_cmd --shell ps|sh --adapter amd|nvidia --backend dx12|vulkan --extra-args ""`) and note defaults to keep parity with existing configs.
  - CLI shape: `warehouse_cmd --shell <ps|sh> --adapter <amd|nvidia> [--backend <dx12|vulkan>] [--batch-size <int>] [--log-every <int>] [--manifest <path>] [--store <stream|memory|mmap>] [--prefetch <int>] [--extra-args <string>]`.
  - Defaults to match today: `shell` implied by platform? (pick explicit default `sh`), `adapter` required (no default), `backend` default `vulkan` for sh, `dx12` for ps; `manifest` defaults to `artifacts/tensor_warehouse/v<version>/manifest.json`; `store=stream` with `prefetch=8`; `model=big`; `batch_size=32`; `log_every=1`; `extra_args=""`.
  - Output command selection: builder picks env formatting based on `shell`; `WGPU_ADAPTER_NAME` only set when `adapter` provided; `WGPU_POWER_PREF` and `RUST_LOG` always set; `prefetch` only emitted when `store=stream`.
  - Backward compatibility: include shorthand subcommands like `amd-ps`, `amd-sh`, `nvidia-ps`, `nvidia-sh` that prefill args to preserve old UX while encouraging the unified flags.
- Step 3: Introduce a `Shell` enum and unify `ps_builder.rs` + `sh_builder.rs` into one builder that only branches on env prefix/separator; keep output strings identical to current commands.
  - Add `enum Shell { PowerShell, Bash }` plus helpers: `env_kv(&self, key, val)` -> `"$env:KEY=\"val\""` or `"KEY=\"val\""`; `sep(&self)` -> `"; "` or `" "`.
  - Create a single `build_command(cfg: &CmdConfig, shell: Shell) -> String` using the helpers; reuse existing env/value ordering to avoid diffing outputs.
  - Drop `ps_builder.rs` and `sh_builder.rs` after migration; update callers to the unified builder.
  - Preserve conditional emission: only include `WAREHOUSE_PREFETCH` when `store == Stream`, and `WGPU_ADAPTER_NAME` when adapter provided.
- Step 4: In `common.rs`, add `Default` for `CmdConfig`, `Display` for enums, and small constructors like `CmdConfig::with_adapter(adapter)` to centralize defaults.
  - Implement `Default` for `CmdConfig` mirroring current shared values (manifest, stream+prefetch 8, big, batch 32, log 1, backend vulkan, no adapter, extra_args empty).
  - Implement `Display` (or `Into<&'static str>`) for `WarehouseStore` and `ModelKind` to simplify builder stringification.
  - Add helpers: `CmdConfig::with_adapter(adapter: &'a str)`, `CmdConfig::with_backend(backend: &'a str)`, and optional `CmdConfig::stream(prefetch: usize)`; keep lifetimes explicit.
  - Update existing code to use `Default::default()` + chained setters instead of duplicating `CmdConfig` literals.
- Step 5: Replace the four binaries with a single `main.rs` that uses a CLI parser (e.g., `clap`) to select shell/adapter/backend and then calls the shared builder.
  - Add `tools/warehouse_commands/bin/main.rs` using `clap` derive: flags for `--shell`, `--adapter`, `--backend`, `--manifest`, `--store`, `--prefetch`, `--batch-size`, `--log-every`, `--model`, `--extra-args`.
  - Provide convenience subcommands/aliases (`amd-ps`, `amd-sh`, `nvidia-ps`, `nvidia-sh`) that prefill defaults matching current binaries.
  - Wire args into a `CmdConfig` starting from `Default` + helper constructors; pass into unified `build_command`.
  - Remove or deprecate old bin targets (or keep as thin wrappers calling into `main` with fixed args) to maintain cargo entry points if needed.
- Step 6: Add unit tests around the builder to snapshot the generated one-liners for representative configs (AMD PS, AMD SH, NVIDIA PS, NVIDIA SH).
  - Add tests in `tools/warehouse_commands/lib` (or `tests/`) that construct `CmdConfig` + `Shell` pairs for the four presets and assert exact string matches against current outputs.
  - Include a test covering `store != Stream` to ensure `WAREHOUSE_PREFETCH` is omitted, and one with `extra_args` set to verify passthrough.
  - Keep ordering identical to current builders to make string equality stable.
- Step 7: Run `cargo fmt` and `cargo clippy` for the tools crate; fix any lint fallout.
  - Run `cargo fmt --package warehouse_commands` (or workspace-level) and commit formatting.
  - Run `cargo clippy --package warehouse_commands -- -D warnings`; address lint findings in the new unified builder/CLI.
- Step 8: Update `README`/docs to explain the new CLI usage and remove references to old per-vendor binaries.
  - Add a short section in `README.md` (or a tools-specific doc) showing `warehouse_cmd --shell ps --adapter amd` style usage plus the legacy aliases for quick copy/paste.
  - Note that old binaries are removed/deprecated and point users to the new CLI flags/subcommands.
  - Include a small table mapping old commands to new equivalents to ease transition.

## Implementation Plan (sequenced)

1) Add `Shell` enum + unified builder module; migrate `ps_builder`/`sh_builder` call sites to it while keeping outputs identical. Remove old builders after parity is confirmed.
2) Extend `common.rs` with `Default`, `Display` for enums, and helper constructors; refactor existing configs to use `Default + setters`.
3) Introduce `tools/warehouse_commands/bin/main.rs` with `clap` CLI and legacy subcommand aliases that reproduce current binaries.
4) Delete or stub old bin targets to call into the new CLI for backward compatibility.
5) Add unit tests asserting command strings for the four presets, plus edge cases for `store != Stream` and `extra_args`.
6) Run `cargo fmt` and `cargo clippy -- -D warnings`; fix any lint issues.
7) Update README/docs with new usage and mapping table.
