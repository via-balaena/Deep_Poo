# CortenForge

Shared Rust crates for the CortenForge simulation substrate (capture, ETL, training, inference, and tooling). This repo is library-only; the `colon_sim` app and other demos now live in their own repositories.

- What’s here: `sim_core`, `vision_core` / `vision_runtime`, `data_contracts`, `capture_utils`, `models`, `training`, `inference`, `colon_sim_tools`, plus supporting crates under `crates/`.
- What moved: the `colon_sim` reference app, `hello_substrate`, and other app binaries. Use the dedicated app repo to run the interactive sim or headless wrappers: https://github.com/via-balaena/Deep-Poo
- Docs: contributor mdBook under `docs/contributor_book` (architecture/extension points). The old user book has been retired; user-facing flows now live in app-specific repos.
- User quickstart (apps): clone the app repo (e.g., `https://github.com/via-balaena/Deep-Poo`), build with `cargo run -p sim_view` or `inference_view`, and wire hooks as needed. This repo stays library-only.
- Releases: see `RELEASE.md` for publish/tag steps.
- License: Apache-2.0 by default; see `LICENSE` and `COMMERCIAL_LICENSE.md`.

## Quick start
- Build/test the crates: `cargo test --workspace --locked`
- Format: `cargo fmt --all`
- Docs: `mdbook build docs/contributor_book`

## Using the crates from crates.io
- Add deps with `version = "0.1.1"` (examples: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-training`, `cortenforge-inference`, `cortenforge-cli-support`, `cortenforge-burn-dataset`).
- Feature flags:
  - `cortenforge-training`: `backend-wgpu` (optional GPU); defaults to NdArray.
  - `cortenforge-inference`: `backend-wgpu` (optional GPU); defaults to NdArray; `tinydet`/`bigdet` feature gates.
  - Tools (`colon_sim_tools`): `scheduler`, `tui`, `gpu_nvidia` (not published by default).
  - Vision/runtime crates are lean by default; enable only what you need.
- MSRV: Rust 1.75+ across crates.
- Note: `burn-core` is temporarily patched to a vendored 0.14.0; we’ll drop the patch once upstream releases a fixed version.

## Commercial opportunities
- Via Balaena™ is offering a 50% profit split on commercial deals you source and help close. Reach out if you have leads or want to collaborate on deployments.

## Contributing
See `docs/contributor_book` for architecture, extension points, and testing notes. App contributions now belong in the app repositories.
