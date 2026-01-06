# Build & Run

## Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets --all-features -D warnings`
- Check: `cargo check --workspace`
- Test: `cargo test --workspace --locked`
- Docs: `cargo doc --workspace --no-deps`
- mdBook: `mdbook build docs/contributor_book` and `mdbook build docs/dissection_book`

## Feature flags
- Defaults: NdArray backends; GPU/WGPU opt-in (`backend-wgpu` on training/inference/models; `gpu_nvidia` on tools).
- Model variants: `tinydet` (default), `bigdet`.
- Tools: `tui`, `scheduler`, `gpu_nvidia` gate app-specific bins.

## Common flags
- `--locked` to enforce lockfile resolution (useful before publish).
- `--all-features` for full surface area (opt-in GPU/tooling paths).
- `--features <list>` to target specific stacks (e.g., `backend-wgpu`, `tui`).

## Troubleshooting (skeleton)
- Build fails due to burn-core/bincode: ensure patch override present until upstream fix; publish may fail without lockfile.
- GPU/WGPU issues: enable the right feature flags; skip on non-GPU hosts; check driver availability.
- Tooling bins: ensure required features (`tui`, `scheduler`, `gpu_nvidia`) are enabled; keep inputs minimal for smokes.
- Docs build: install mdBook; run `mdbook test` for doctests; mark non-runnable snippets with `ignore`.
- Mermaid diagrams: install `mdbook-mermaid` (`cargo install mdbook-mermaid`) before building the dissection book.
- Dependency policy: shared deps should use root `[workspace.dependencies]`, but `bevy` stays per-crate until feature/default-features are unified.
