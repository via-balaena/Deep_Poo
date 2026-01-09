# Release checklist (CortenForge crates)

Target version: `0.1.3`

Note: For the next releases (1.4), you can bump all workspace crates to a single shared version (e.g., 0.1.4) for simplicity. (currently each individual crate is at 0.1.2 and umbrella is at 0.1.3)

Follow these steps to publish the `cortenforge-*` crates and tag a release. Adjust the crate list if new crates are added.

## Prereqs
- Make sure working tree is clean and on the intended release branch.
- Ensure `cargo` is logged into crates.io and has publish rights.
- Confirm versions: currently `0.1.x` across all crates.
- Release notes: update the changelog entry in `docs/cortenforge_book/src/00_workspace/changelog.md`.

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
11. `cortenforge` (umbrella)
12. `cortenforge-tools` (publish if desired; currently publish = false)

## Steps
1) Bump versions (aligned `0.1.x`) and update changelog/release notes if applicable.
2) `cargo fmt --all`
3) `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4) `cargo test --workspace --locked`
5) For each crate above:
   - `cargo publish -p <crate> --dry-run`
   - `cargo publish -p <crate>`
   - Wait for crates.io indexing (a few minutes) before publishing dependents.
6) Regenerate lockfile: `cargo generate-lockfile` (optional, keeps root in sync).
7) Tag the repo: `git tag -a v0.1.x -m "Release v0.1.x"` and push tags: `git push --tags`.
8) Update docs/README with the release notes and any feature/backends notes.

## Notes
- Burn patch: upstream has published a fixed `burn-core` release; remove the vendored patch (`vendor/burn-core-0.14.0`) and drop the `[patch.crates-io]` override before publishing.
- Keep path deps out of manifests; use versioned deps only.
- If publishing `cortenforge-tools`, set `publish = true` in `tools/Cargo.toml` and ensure its deps are published first.
