# Release checklist (CortenForge crates)

Target version: `0.2.1`

Note: Keep all workspace crates on a single shared version for simplicity (e.g., 0.2.1 across the board).

Follow these steps to publish the `cortenforge-*` crates and tag a release. Adjust the crate list if new crates are added.

## Prereqs
- Make sure working tree is clean and on the intended release branch.
- Ensure `cargo` is logged into crates.io and has publish rights.
- Confirm versions: currently `0.2.1` across all crates.
- Release notes: update the changelog entry in `docs/cortenforge_book/src/00_workspace/changelog.md`.

## Release notes (v0.2.1)
- Added `gpu_probe` with a stable JSON schema and `--format=json` flag.
- Cross-platform GPU probing: macOS (stub), Linux (NVIDIA/AMD/Intel), Windows (NVIDIA + AMD stub feature flag).
- `gpu_macos_helper` retained as a deprecated alias (planned removal in v0.3+); docs updated accordingly.

## Release notes (v0.3.0 draft)
- Feature flags normalized: use `burn-runtime` and `gpu-nvidia` (legacy aliases removed).

## Crate order (publish)
1. `cortenforge-data-contracts`
2. `cortenforge-models`
3. `cortenforge-burn-dataset`
4. `cortenforge-cli-support`
5. `cortenforge-vision-core`
6. `cortenforge-sim-core`
7. `cortenforge-capture-utils`
8. `cortenforge-inference`
9. `cortenforge-vision-runtime`
10. `cortenforge-training`
11. `cortenforge-tools` (publishable; `publish = true`)
12. `cortenforge` (umbrella)

## Steps
1) Bump versions (aligned `0.2.1`) and update changelog/release notes if applicable.
2) `cargo fmt --all`
3) `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4) `cargo test --workspace --locked`
5) For each crate above:
   - `cargo publish -p <crate> --dry-run`
   - `cargo publish -p <crate>`
   - Wait for crates.io indexing (a few minutes) before publishing dependents.
   - Optional copy/paste loop:
     - `for c in cortenforge-data-contracts cortenforge-models cortenforge-burn-dataset cortenforge-cli-support cortenforge-vision-core cortenforge-sim-core cortenforge-capture-utils cortenforge-inference cortenforge-vision-runtime cortenforge-training cortenforge-tools cortenforge; do cargo publish -p "$c"; sleep 90; done`
6) Regenerate lockfile: `cargo generate-lockfile` (optional, keeps root in sync).
7) Tag the repo: `git tag -a v0.2.1 -m "Release v0.2.1"` and push tags: `git push --tags`.
8) Update docs/README with the release notes and any feature/backends notes.

## Verify tags + crates.io
- Tag points at the expected commit: `git show v0.2.1 --no-patch`
- Tag is on origin: `git ls-remote --tags origin v0.2.1`
- Crates.io shows 0.2.1: `https://crates.io/crates/<crate>`

## Notes
- Burn patch: upstream has published a fixed `burn-core` release; remove any vendored patch and drop `[patch.crates-io]` overrides before publishing.
- Bincode note: `bincode 3.0.0` is a stub on crates.io (compile_error); use `2.0.1` until 3.x is real.
- Keep path deps out of manifests; use versioned deps only.
- If publishing `cortenforge-tools`, set `publish = true` in `tools/Cargo.toml` and ensure its deps are published first.

## Upgrade cadence
- Target: quarterly dependency upgrades, or immediately when upstream breakage requires it.
- If MSRV changes, update both `README.md` and CI toolchain pins in the same PR.
