# Design Review (burn_dataset)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Comprehensive dataset handling: validation, augmentation, splitting, batching, and sharded warehouse support.
- Flexible store abstraction (`WarehouseShardStore`) with in-memory and streaming implementations.
- Env-tunable behavior (logging, permissive mode, prefetch) without changing code.

## Risks / gaps
- Permissive/error handling via env can hide data quality issues unless monitored.
- Collation requires consistent image sizes; no automatic resize/pad path may surprise users with mixed data.
- Augmentation and buffer reuse are hand-rolled; performance could vary across workloads.

## Refactor ideas
- Add explicit configuration (struct) for permissive/logging/trace instead of env-only control, enabling code-driven behavior (and tests).
- Provide an optional resize/padding strategy in collation to handle heterogeneous datasets gracefully.
- Consider centralizing augmentation parameters and using SIMD/parallel image ops if profiling shows hotspots; add metrics hooks for skipped/errored samples.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
