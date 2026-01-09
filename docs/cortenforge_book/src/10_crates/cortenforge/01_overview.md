# cortenforge (umbrella): Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide a facade/umbrella crate that re-exports the CortenForge stack with feature wiring to make it easy for consumers to opt into runtime, vision, training, inference, and tooling components via a single dependency.

## Scope
- Re-export member crates (sim_core, vision_core/runtime, data_contracts, capture_utils, models, training, inference, burn_dataset, cli_support) with feature mapping.
- Feature wiring for common stacks (burn-runtime/burn-wgpu, model variants, runtime/vision toggles).

## Non-goals
- No own implementations; purely a facade.
- No app-specific defaults; consumers still select features explicitly.

## Who should use it
- Downstream consumers wanting a single entry point to the substrate with feature toggles rather than pinning each crate individually.
- Contributors maintaining feature mappings and keeping versions aligned across the stack.

## Links
- Source: `crates/cortenforge/src/lib.rs`
- Docs.rs: https://docs.rs/cortenforge
