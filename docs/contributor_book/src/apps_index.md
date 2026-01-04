# Build & Dev Environment

How to set up a contributor workstation, which features to use by default, and how to keep iterations fast.

## Prereqs
- Rust 1.75+ (`rustup`), with `rustfmt` and `clippy` components installed.
- `mdbook` for docs (`cargo install mdbook`), and optional `mdbook-mermaid` if we add diagrams.
- Optional: `cargo deny`, `cargo hakari` if you want to mirror CI’s dependency checks.

## Defaults vs. features
- Default builds/tests use NdArray backends; no GPU/WGPU required.
- GPU/WGPU paths are behind features (`backend-wgpu` on training/inference, `gpu_nvidia` on tools).
- `colon_sim_tools` feature flags: `tui`, `scheduler`, `gpu_nvidia` gate heavier deps.
- Burn is temporarily patched to a vendored `burn-core 0.14.0` until upstream fixes the bincode publish break.

## Dev loop
- Fast pass: `cargo fmt --all && cargo clippy --workspace --all-targets -D warnings && cargo test --workspace`.
- Locked pass: `cargo test --workspace --locked` (useful before publishing).
- Docs: `mdbook build docs/contributor_book`.
- Optional dep checks: `cargo hakari generate && cargo hakari manage-deps`; `cargo deny check`.

## Platform notes
- macOS/Linux are supported; defaults avoid GPU. If enabling WGPU, ensure platform drivers are present (Metal on macOS, Vulkan/DX on others).
- Burn may download libtorch for certain backends; keep an eye on toolchain downloads during first build.

## Tips
- Gate heavy/GPU code behind features; keep default tests lean.
- Use `--features` selectively when working on GPU paths or tool extras.
- Keep the lockfile fresh before release; pinned burn-core patch can be dropped once upstream ships a fix.
