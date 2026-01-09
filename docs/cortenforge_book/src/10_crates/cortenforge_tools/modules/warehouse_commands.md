# warehouse_commands (cortenforge-tools)

## Responsibility
- Provide small helpers to build environment + command strings for warehouse-based training/inference flows.
- Encapsulate model/store selections and shell-specific env formatting.

## Key items
- `WarehouseStore` (Memory/Mmap/Stream) and `ModelKind` (Tiny/Big).
- `CmdConfig`: manifest path, store mode, prefetch, model, batch size, logging cadence, WGPU backend/adapter, extra args.
- `DEFAULT_CONFIG`: reasonable streaming defaults (prefetch 8, vulkan, Big, batch 32).
- `Shell` enum (PowerShell/Bash) with formatting helpers.
- `build_command(cfg, shell)`: emits a single shell line that sets env vars (manifest, store mode, prefetch, WGPU, logging) then runs `cargo train_hp`.

## Invariants / Gotchas
- `prefetch` only applies when store=Stream.
- Env formatting differs per shell; ensure the chosen shell matches the user’s environment.
- Command assumes `cargo train_hp` target exists; consumers must supply manifests/paths.

## Cross-module deps
- Consumed by `tools/src/bin/warehouse_cmd.rs` and tests under `tools/tests`.
- Uses only std; no runtime side effects beyond string building.
