# training: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Provide Burn-based training and evaluation pipelines for TinyDet/BigDet models using warehouse manifests produced by the CortenForge capture/ETL flow.

## Scope
- Training/eval CLIs (`train`, `eval`) driving `run_train`/`run_eval`.
- Dataset loader from warehouse manifests to tensors; collate/loss/matching, optimizer/checkpoint I/O.
- Feature flags for backends (`backend-ndarray` default, `backend-wgpu` opt-in) and model variants (`tinydet`/`bigdet`).

## Non-goals
- No model definitions (comes from models crate).
- No capture/ETL; assumes warehouse artifacts already exist.
- No runtime/live inference; that’s in inference/vision_runtime.

## Who should use it
- Users training/evaluating TinyDet/BigDet models from captured data.
- Contributors adding losses/augmentations/schedulers or adjusting CLI flags.

## Links
- Source: `training/src/lib.rs`
- Module: `training/src/dataset.rs`
- Module: `training/src/util.rs`
- Docs.rs: https://docs.rs/cortenforge-training
