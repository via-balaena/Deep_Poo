# models: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Define Burn-based model architectures/configs (TinyDet/BigDet) used across training and inference in the CortenForge stack.

## Scope
- Model definitions/configs for TinyDet and BigDet.
- Feature flags to gate variants (`tinydet`, `bigdet`).
- Pure model code; no training loop or inference factory logic.

## Non-goals
- No dataset loading or training/eval loop (handled by training).
- No checkpoint loading or detector factory (handled by inference).
- No app/domain-specific heads beyond provided variants.

## Who should use it
- Training crate to construct models/checkpoints.
- Inference crate to load checkpoints and build detectors.
- Contributors adding model variants or adjusting configs.

## Links
- Source: `crates/models/src/lib.rs`
- Docs.rs: https://docs.rs/models
