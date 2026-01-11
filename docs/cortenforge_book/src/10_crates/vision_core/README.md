# vision_core

## Overview
Vision interfaces and data shapes shared by runtime, tools, and training.

## Usage
Import `vision_core::prelude::*` and implement detectors/recorders in higher layers; docs.rs: https://docs.rs/cortenforge-vision-core; source: https://github.com/via-balaena/CortenForge/tree/main/crates/vision_core.

## Pitfalls
Keep heavy backends out of this crate; wire them in inference/training instead.
