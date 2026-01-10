# data_contracts

[![crates.io](https://img.shields.io/crates/v/cortenforge-data-contracts.svg)](https://crates.io/crates/cortenforge-data-contracts) [![docs.rs](https://docs.rs/cortenforge-data-contracts/badge.svg)](https://docs.rs/cortenforge-data-contracts) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

Data contracts for run manifests and capture metadata shared across the CortenForge stack. Provides serde-friendly structs plus validation helpers for manifests and per-frame capture labels.

> Deprecated: this crate was renamed to `cortenforge-data-contracts`. Please use the new crate name for future installs.

## Features
- No default features; serde-based types only.
- Validates manifest timestamps/frame counts and capture label bounding boxes.

## License
Apache-2.0 (see `LICENSE` in the repo root).
