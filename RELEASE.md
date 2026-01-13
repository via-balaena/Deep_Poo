# Release checklist (CortenForge crates)

Target version: `0.5.1`

Note: Keep all workspace crates on a single shared version for simplicity (e.g., 0.5.1 across the board).

Follow these steps to publish the `cortenforge-*` crates and tag a release. Adjust the crate list if new crates are added.

## Version Management

**Automated version synchronization** is now available via `scripts/sync_versions.sh`:

```bash
# Check for version mismatches (dry-run)
./scripts/sync_versions.sh

# Apply version sync if needed
./scripts/sync_versions.sh --apply
```

This script:

- Parses the workspace version from root `Cargo.toml`
- Updates all internal `cortenforge-*` dependency versions
- Shows diffs before applying changes
- Prevents version drift across the 11-crate workspace

**CI validation**: Version consistency is enforced in CI (`.github/workflows/ci.yml`). The CI will fail if:

- Any crate package version doesn't match the workspace version
- Any internal `cortenforge-*` dependency has a mismatched version
- Any publishable manifest contains path dependencies

## Prereqs

- Make sure working tree is clean and on the intended release branch.
- Ensure `cargo` is logged into crates.io and has publish rights.
- Confirm versions: currently `0.5.1` across all crates.
  - Run `./scripts/sync_versions.sh` to verify
- Release notes: update the changelog entry in `docs/cortenforge_book/src/00_workspace/changelog.md`.

## Release notes (v0.5.1)

- Added automated version synchronization script (`scripts/sync_versions.sh`)
- Added CI validation for version consistency
- Refactored `burn_dataset` into focused modules (capture, warehouse, batch, splits, aug, types, validation, parquet)
- Added comprehensive integration tests for E2E workflows
- All workspace crates synchronized to 0.5.1

## Previous release notes (v0.5.0)

- Removed `gpu_amd_windows` alias and legacy warehouse command fallback/tests in tools.
- Removed legacy crate-name deprecation notices from crate README/Cargo descriptions.
- Docs updated to remove legacy alias guidance and note the breaking changes.

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

1) **Bump versions** in root `Cargo.toml` to target version (e.g., `0.5.2`)

2) **Sync all internal dependencies**:

   ```bash
   ./scripts/sync_versions.sh --apply
   ```

3) **Update changelog** and release notes in this file and in `docs/cortenforge_book/src/00_workspace/changelog.md`

4) **Format and lint**:

   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```

5) **Test**:

   ```bash
   cargo test --workspace --locked
   ```

6) **Dry-run publish** for each crate to verify:

   ```bash
   cargo publish -p cortenforge-data-contracts --dry-run
   cargo publish -p cortenforge-models --dry-run
   cargo publish -p cortenforge-burn-dataset --dry-run
   # ... (repeat for all crates in dependency order)
   ```

7) **Publish crates** in dependency order (see "Crate order" section below):

   ```bash
   # Manual (recommended for first-time or careful releases):
   cargo publish -p cortenforge-data-contracts
   # Wait 2-3 minutes for crates.io indexing
   cargo publish -p cortenforge-models
   # ... continue in order

   # Automated (for experienced releases):
   for c in cortenforge-data-contracts cortenforge-models cortenforge-burn-dataset cortenforge-cli-support cortenforge-vision-core cortenforge-sim-core cortenforge-capture-utils cortenforge-inference cortenforge-vision-runtime cortenforge-training cortenforge-tools cortenforge; do
     cargo publish -p "$c"
     sleep 120  # 2 minutes between crates
   done
   ```

8) **Tag the release**:

   ```bash
   git tag -a v0.5.2 -m "Release v0.5.2"
   git push --tags
   ```

9) **Regenerate lockfile** (optional, keeps root in sync):

   ```bash
   cargo generate-lockfile
   ```

10) **Update docs/README** with release notes and any feature/backend notes

## Verify tags + crates.io

- Tag points at the expected commit: `git show v0.5.1 --no-patch`
- Tag is on origin: `git ls-remote --tags origin v0.5.1`
- Crates.io shows 0.5.1: `https://crates.io/crates/<crate>`
- Version consistency: `./scripts/sync_versions.sh` (should show "✓ All versions synchronized")

## Notes
- Burn patch: upstream has published a fixed `burn-core` release; remove any vendored patch and drop `[patch.crates-io]` overrides before publishing.
- Bincode note: `bincode 3.0.0` is a stub on crates.io (compile_error); use `2.0.1` until 3.x is real.
- Keep path deps out of manifests; use versioned deps only.
- If publishing `cortenforge-tools`, set `publish = true` in `tools/Cargo.toml` and ensure its deps are published first.

## Upgrade cadence
- Target: quarterly dependency upgrades, or immediately when upstream breakage requires it.
- If MSRV changes, update both `README.md` and CI toolchain pins in the same PR.
