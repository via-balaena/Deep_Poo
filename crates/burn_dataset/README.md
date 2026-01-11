# burn_dataset

Burn dataset loading, validation, splitting, and batching utilities used by the CortenForge stack.

## Features
- `burn-runtime` (alias `burn_runtime`, off by default): enables Burn-backed batching, mmap/crossbeam/rayon helpers.
- Without `burn-runtime`, the crate still provides JSON label parsing, splitting, and filtering utilities.

## License
Apache-2.0 (see `LICENSE` in the repo root).
