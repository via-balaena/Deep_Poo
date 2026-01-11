# CortenForge

Shared Rust crates for the CortenForge simulation substrate (capture, ETL, training, inference, and tooling). This repo is library-only; the `colon_sim` app and other demos now live in their own repositories.

- What’s here: the root `cortenforge` umbrella crate, plus `sim_core`, `vision_core` / `vision_runtime`, `data_contracts`, `capture_utils`, `models`, `training`, `inference`, and `cortenforge-tools` under `crates/` and `tools/`.
- Example app: see https://github.com/via-balaena/Deep-Poo for a colonoscopy robot sim built with CortenForge.
- Docs:
  - CortenForge book (primary): https://via-balaena.github.io/CortenForge/ (source in `docs/cortenforge_book`, build with `mdbook build docs/cortenforge_book`).
  - Legacy contributor/dissection books have been retired.
- User quickstart (apps): clone the app repo (e.g., `https://github.com/via-balaena/Deep-Poo`), build with `cargo run -p sim_view` or `inference_view`, and wire hooks as needed. This repo stays library-only.
- Releases: see `RELEASE.md` for publish/tag steps.
- License: Apache-2.0 by default; see `LICENSE` and `COMMERCIAL_LICENSE.md`.

## Repository layout
- `Cargo.toml`: workspace root + umbrella crate (`cortenforge`).
- `crates/`: core and mid‑layer libraries (`sim_core`, `vision_*`, `data_contracts`, `models`, `training`, `inference`, `capture_utils`, `burn_dataset`, `cli_support`).
- `tools/`: `cortenforge-tools` crate (app‑agnostic tooling bins and helpers).
- `docs/`: books and release material (`docs/cortenforge_book` is the primary book).
- `todo/`: local planning (gitignored).

## Quick start
- Build/test the crates: `cargo test --workspace --locked`
- Format: `cargo fmt --all`
- Docs: `mdbook build docs/cortenforge_book`
- Tools config: create `cortenforge-tools.toml` at repo root (or set `CORTENFORGE_TOOLS_CONFIG`) to customize paths/commands.

## Using the crates from crates.io
- Add deps with `version = "0.2.0"` (examples: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-training`, `cortenforge-inference`, `cortenforge-cli-support`, `cortenforge-burn-dataset`, `cortenforge-tools`).
- Umbrella crate: `cortenforge` is at `0.2.0` (includes an optional `tools` feature).
- Feature flags:
  - `cortenforge-training`: `backend-wgpu` (optional GPU); defaults to NdArray.
  - `cortenforge-inference`: `backend-wgpu` (optional GPU); defaults to NdArray; `tinydet`/`bigdet` feature gates.
  - Tools (`cortenforge-tools`): `scheduler`, `tui`, `gpu-nvidia`.
  - Vision/runtime crates are lean by default; enable only what you need.
- Feature policy: keep defaults light, gate heavy backends/tools behind explicit flags, and document any new feature in the book’s feature matrix.
- Note: 0.3.0 removes legacy feature aliases; use `burn-runtime` and `gpu-nvidia` only.
- MSRV: Rust 1.85+ across crates (CI uses 1.89.0 for toolchain compatibility).
- Note: `burn-core` is now on the fixed 0.19.1 release; no vendored patch is required.

## Contributing
See `docs/cortenforge_book` for architecture, guided app building, and crate deep dives.
