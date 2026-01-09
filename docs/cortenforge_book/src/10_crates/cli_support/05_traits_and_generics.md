# Traits & Generics (cli_support)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None; crate provides plain structs/helpers for CLI configuration.

## Generics and bounds
- All structs are concrete; no generic parameters or custom traits.
- Clap derives (`Args`) used where CLI parsing is needed (`CaptureOutputArgs`, `WarehouseOutputArgs`).
- SeedState optionally derives `bevy::Resource` behind `bevy-resource` feature for reuse in Bevy apps.

## Design notes
- Keeping everything concrete avoids trait-object noise in binaries; consumers convert `Args` → internal opts via `From`.
- Feature-gated Bevy derive keeps default dependency surface minimal.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
