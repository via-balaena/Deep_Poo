# inference: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide a detector factory that loads model checkpoints (TinyDet/BigDet via models) and returns a `Detector` implementation for runtime/tools, with a heuristic fallback when weights are absent.

## Scope
- Factory API to load checkpoints and build Burn-backed detectors.
- Heuristic detector fallback for cases without weights.
- Feature flags for backends (`backend-ndarray` default, `backend-wgpu` opt-in) and model variants (`tinydet`/`bigdet`).

## Non-goals
- No capture/runtime scheduling (handled by vision_runtime).
- No model definitions (from models) or training (from training).
- No app-specific detector logic; keeps interface generic.

## Who should use it
- Runtime/inference plugins (vision_runtime) needing a detector handle.
- Tools performing offline inference (e.g., single_infer) that need a factory.
- Contributors adding backends or selection logic.

## Links
- Source: `crates/inference/src/lib.rs`
- Module: `crates/inference/src/factory.rs`
- Module: `crates/inference/src/plugin.rs`
- Docs.rs: https://docs.rs/cortenforge-inference
