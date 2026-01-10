# data_contracts: Overview
Quick read: What this crate does and where it fits.

## Problem statement
Define and validate shared schemas for captures, manifests, and warehouse shards used across the CortenForge stack.

## Scope
- Run manifest schema (id/seed/camera/resize/letterbox/frame count/checksum).
- Frame label schema (bbox norm/px, class, metadata, overlays optional).
- Warehouse manifest/shard schemas for ETL/training.
- Validation helpers to enforce ranges/required fields.

## Non-goals
- No I/O or recorder sinks; consumers read/write using these schemas.
- No runtime/Bevy integration; pure data definitions.
- No app-specific metadata beyond generic fields.

## Who should use it
- Capture/recorder code (capture_utils/tools) writing manifests/labels.
- ETL/training pipelines consuming warehouse manifests/shards.
- Contributors updating schemas with strict validation.

## Links
- Source: `crates/data_contracts/src/lib.rs`
- Module: `crates/data_contracts/src/capture.rs`
- Module: `crates/data_contracts/src/manifest.rs`
- Docs.rs: https://docs.rs/cortenforge-data-contracts
