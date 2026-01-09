# cli_support: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide shared CLI argument parsing and helper utilities for CortenForge tools/apps (capture, warehouse, seeds, thresholds), reducing duplication across bins.

## Scope
- Common CLI structs/parsers for capture/warehouse/config options.
- Optional Bevy resource integration (`bevy`, `bevy-resource` features).
- Reusable helpers for tools and app CLIs.

## Non-goals
- No runtime logic or capture/ETL implementation; only CLI plumbing.
- No app-specific commands; keep generic to substrate tooling.

## Who should use it
- Tooling crates (cortenforge-tools; crate `cortenforge_tools`) and any app binaries needing consistent args.
- Contributors adding/adjusting shared CLI options for the stack.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
- Docs.rs: https://docs.rs/cortenforge-cli-support
