# models

[![crates.io](https://img.shields.io/crates/v/cortenforge-models.svg)](https://crates.io/crates/cortenforge-models) [![docs.rs](https://docs.rs/cortenforge-models/badge.svg)](https://docs.rs/cortenforge-models) [![MSRV](https://img.shields.io/badge/rustc-1.75+-orange.svg)](#)

Burn-based model definitions for the CortenForge stack.

## Contents
- `TinyDet` / `TinyDetConfig`: small detector MLP.
- `BigDet` / `BigDetConfig`: configurable multibox MLP (depth/hidden/max_boxes/input_dim) with helper to clamp boxes to \[0,1\].
- `prelude`: re-export of configs and models.

## Features
- `tinydet` (default): includes TinyDet.
- `bigdet`: includes BigDet.

## License
Apache-2.0 (see `LICENSE` in the repo root).
