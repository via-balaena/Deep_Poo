# Build & Run
**Why**: A small, reliable set of commands you can trust.
**How it fits**: Use this before PRs and when sanity-checking changes.
**Learn more**: See [Dev Loop](dev_loop.md) and [Reproducibility](reproducibility.md).

## Commands
Quick reference for common build and verification tasks.

| Task | Command |
| --- | --- |
| Format | `cargo fmt --all` |
| Lint | `cargo clippy --workspace --all-targets --all-features -D warnings` |
| Check | `cargo check --workspace` |
| Test | `cargo test --workspace --locked` |
| Docs | `cargo doc --workspace --no-deps` |
| mdBook | `mdbook build docs/cortenforge_book` |

## Feature flags
Default behavior and opt-in switches for common workflows.

| Topic | Default | Notes |
| --- | --- | --- |
| Backends | NdArray | GPU/WGPU opt‑in via `backend-wgpu` (training/inference/models); `gpu-nvidia` on tools. |
| Model variants | `tinydet` | `bigdet` optional. |
| Tools | none | `tui`, `scheduler`, `gpu-nvidia` gate app‑specific bins. |

## Common flags
Flags that change dependency resolution or feature surfaces.

| Flag | Meaning |
| --- | --- |
| `--locked` | Enforce lockfile resolution (useful before publish). |
| `--all-features` | Full surface area (opt-in GPU/tooling paths). |
| `--features <list>` | Target specific stacks (e.g., `backend-wgpu`, `tui`). |

## Troubleshooting (skeleton)
Common failure modes and the fastest fix.

| Issue | Guidance |
| --- | --- |
| Build fails due to burn-core/bincode | Ensure burn-core is 0.19.1+ and bincode is 2.0.1; refresh the lockfile; publish may fail without a lockfile. |
| GPU/WGPU issues | Enable the right feature flags; skip on non-GPU hosts; check driver availability. |
| Tooling bins | Ensure required features (`tui`, `scheduler`, `gpu-nvidia`) are enabled; keep inputs minimal for smokes. |
| Docs build | Install mdBook; run `mdbook test` for doctests; mark non-runnable snippets with `ignore`. |
| Mermaid diagrams | Install `mdbook-mermaid` (`cargo install mdbook-mermaid`) before building this book. |
| Dependency policy | Shared deps should use root `[workspace.dependencies]`, but `bevy` stays per-crate until feature/default-features are unified. |
